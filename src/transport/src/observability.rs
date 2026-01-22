//! Observability HTTP helpers contained in the Transport layer.

use serde::Serialize;
use std::io::{prelude::*, BufRead, BufReader};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub struct ObservabilityState {
    version: String,
    world_id: String,
    storage_backend: String,
    ready: AtomicBool,
    flush_count: AtomicUsize,
    last_flush: Mutex<Option<SystemTime>>,
    storage_error: Mutex<Option<String>>,
    start: Instant,
}

impl ObservabilityState {
    pub fn new(
        version: impl Into<String>,
        world_id: impl Into<String>,
        storage_backend: impl Into<String>,
    ) -> Self {
        ObservabilityState {
            version: version.into(),
            world_id: world_id.into(),
            storage_backend: storage_backend.into(),
            ready: AtomicBool::new(false),
            flush_count: AtomicUsize::new(0),
            last_flush: Mutex::new(None),
            storage_error: Mutex::new(None),
            start: Instant::now(),
        }
    }

    pub fn mark_storage_ready(&self, ready: bool) {
        self.ready.store(ready, Ordering::SeqCst);
    }

    pub fn storage_ready(&self) -> bool {
        self.ready.load(Ordering::SeqCst)
    }

    pub fn note_flush(&self, flush_count: usize, last_flush: Option<SystemTime>) {
        self.flush_count.store(flush_count, Ordering::SeqCst);
        if let Some(value) = last_flush {
            let mut guard = self.last_flush.lock().expect("lock poisoning");
            *guard = Some(value);
        }
        let mut error_guard = self.storage_error.lock().expect("lock poisoning");
        *error_guard = None;
    }

    pub fn note_error(&self, message: impl Into<String>) {
        self.mark_storage_ready(false);
        let mut guard = self.storage_error.lock().expect("lock poisoning");
        *guard = Some(message.into());
    }

    pub fn snapshot(&self) -> ObservabilitySnapshot {
        let uptime = self.start.elapsed().as_secs();
        let last_flush = *self.last_flush.lock().expect("lock poisoning");
        let storage_error = self.storage_error.lock().expect("lock poisoning").clone();
        ObservabilitySnapshot {
            version: self.version.clone(),
            world_id: self.world_id.clone(),
            storage_backend: self.storage_backend.clone(),
            storage_ready: self.storage_ready(),
            flush_count: self.flush_count.load(Ordering::SeqCst),
            last_flush_at: last_flush
                .map(|ts| ts.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()),
            uptime_seconds: uptime,
            storage_error,
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn world_id(&self) -> &str {
        &self.world_id
    }

    pub fn storage_backend(&self) -> &str {
        &self.storage_backend
    }
}

#[derive(Serialize)]
pub struct ObservabilitySnapshot {
    version: String,
    world_id: String,
    storage_backend: String,
    storage_ready: bool,
    flush_count: usize,
    last_flush_at: Option<u64>,
    uptime_seconds: u64,
    storage_error: Option<String>,
}

pub struct ObservabilityServer {
    shutdown: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
    addr: SocketAddr,
}

impl ObservabilityServer {
    pub fn start(state: Arc<ObservabilityState>, addr: SocketAddr) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        let actual_addr = listener.local_addr()?;
        let shutdown = Arc::new(AtomicBool::new(false));
        let thread_shutdown = Arc::clone(&shutdown);
        let listener_thread = listener.try_clone()?;
        let handle = thread::spawn(move || loop {
            if thread_shutdown.load(Ordering::SeqCst) {
                break;
            }
            match listener_thread.accept() {
                Ok((mut stream, _)) => {
                    let _ = handle_connection(&mut stream, &state);
                }
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(20));
                    continue;
                }
                Err(_) => break,
            }
        });
        Ok(ObservabilityServer {
            shutdown,
            handle: Some(handle),
            addr: actual_addr,
        })
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn shutdown(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for ObservabilityServer {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn handle_connection(stream: &mut TcpStream, state: &ObservabilityState) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");
    let mut discard = String::new();
    loop {
        let bytes = reader.read_line(&mut discard)?;
        if bytes == 0 || discard == "\r\n" {
            break;
        }
        discard.clear();
    }

    let (status, body) = match path {
        "/health" => ("200 OK", r#"{"status":"ok"}"#.to_string()),
        "/ready" => {
            if state.storage_ready() {
                ("200 OK", r#"{"status":"ready"}"#.to_string())
            } else {
                (
                    "503 Service Unavailable",
                    r#"{"status":"initializing"}"#.to_string(),
                )
            }
        }
        "/status" => {
            let snapshot = state.snapshot();
            let body = serde_json::to_string(&snapshot).unwrap_or_default();
            ("200 OK", body)
        }
        _ => ("404 Not Found", r#"{"status":"missing"}"#.to_string()),
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nCache-Control: no-store\r\nContent-Length: {}\r\n\r\n{}",
        status,
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    fn send_request(addr: SocketAddr, path: &str) -> String {
        let mut stream = TcpStream::connect(addr).expect("connect");
        let request = format!("GET {} HTTP/1.1\r\nHost: localhost\r\n\r\n", path);
        stream.write_all(request.as_bytes()).expect("write request");
        stream.shutdown(std::net::Shutdown::Write).ok();
        let mut buf = String::new();
        stream.read_to_string(&mut buf).expect("read response");
        buf
    }

    #[test]
    fn observability_endpoints_report_status() {
        let state = Arc::new(ObservabilityState::new("0.2.0", "world", "sqlite"));
        let mut server =
            ObservabilityServer::start(state.clone(), "127.0.0.1:0".parse().unwrap()).unwrap();
        let addr = server.local_addr();
        std::thread::sleep(Duration::from_millis(10));
        let health = send_request(addr, "/health");
        assert!(health.contains("200 OK"));
        let ready = send_request(addr, "/ready");
        assert!(ready.contains("503 Service Unavailable"));
        state.mark_storage_ready(true);
        let ready_ok = send_request(addr, "/ready");
        assert!(ready_ok.contains("200 OK"));
        let status = send_request(addr, "/status");
        assert!(status.contains("\"version\":\"0.2.0\""));
        server.shutdown();
    }
}
