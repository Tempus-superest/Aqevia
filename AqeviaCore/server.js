const express = require('express');
const http = require('http');
const socketIo = require('socket.io');
const mongoose = require('mongoose');
const characterRoutes = require('./routes/characterRoutes');
const roomRoutes = require('./routes/roomRoutes');
const itemRoutes = require('./routes/itemRoutes');  // Ensure you've created this file

const app = express();
const server = http.createServer(app);
const io = socketIo(server);

// MongoDB Connection
mongoose.connect('mongodb://localhost:27017/Aqevia_Test', {
  useNewUrlParser: true,
  useUnifiedTopology: true
});
const db = mongoose.connection;
db.on('error', console.error.bind(console, 'connection error:'));
db.once('open', function () {
  console.log('Connected to MongoDB');
});

app.use(express.json());  // For parsing application/json
app.use(express.static('AqeviaClient'));  // Serve static files

// Routes
app.use('/characters', characterRoutes);
app.use('/rooms', roomRoutes);  // Use room routes
app.use('/items', itemRoutes);  // Use item routes

// Socket.io for real-time communication
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
