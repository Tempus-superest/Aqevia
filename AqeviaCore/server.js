// Import necessary modules
const express = require('express');
const http = require('http');
const socketIo = require('socket.io');

// Initialize Express app
const app = express();
const server = http.createServer(app);
const io = socketIo(server);

// Set a basic route
app.get('/', (req, res) => {
    res.send('Welcome to the Aqevia MUD Server!');
});

// Handle WebSocket connections
io.on('connection', (socket) => {
    console.log('A user connected');

    socket.on('disconnect', () => {
        console.log('User disconnected');
    });

    // Example event: handle incoming messages and broadcast them
    socket.on('message', (msg) => {
        console.log('Message received:', msg);
        io.emit('message', msg);
    });
});

// Set server to listen on a specified port
const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
