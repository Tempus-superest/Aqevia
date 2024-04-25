const express = require('express');
const http = require('http');
const socketIo = require('socket.io');

const app = express();
const server = http.createServer(app);
const io = socketIo(server);

// Serve static files from the directory where 'index.html' is located
app.use(express.static('AqeviaClient'));

// Additional route for testing or other purposes
app.get('/', (req, res) => {
    res.send('Welcome to the Aqevia MUD Server!');
});

// WebSocket connection handling
io.on('connection', (socket) => {
    console.log('A user connected');
    socket.on('disconnect', () => {
        console.log('User disconnected');
    });
    socket.on('message', (msg) => {
        console.log('Message received:', msg);
        io.emit('message', msg);
    });
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
