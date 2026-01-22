//! Entry point for the Aqevia Engine binary that hosts a single World per deployment unit.

use aqevia_engine::Engine;

fn main() {
    let engine = Engine::new();
    println!("Server running: {}", engine.run_one_world());
}
