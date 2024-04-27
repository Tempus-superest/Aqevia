// server.js

const express = require('express');
const http = require('http');
const socketIo = require('socket.io');
const mongoose = require('mongoose');

const authRoutes = require('./routes/auth');
const characterRoutes = require('./routes/characterRoutes'); 
const roomRoutes = require('./routes/roomRoutes');
const itemRoutes = require('./routes/itemRoutes');

const app = express();
const server = http.createServer(app);
const io = socketIo(server);

// Connect to MongoDB
mongoose.connect('mongodb://localhost:27017/Aqevia_Test', {
  useNewUrlParser: true,
  useUnifiedTopology: true  
});

const db = mongoose.connection;

db.on('error', console.error.bind(console, 'MongoDB connection error:'));
db.once('open', () => {
  console.log('Connected to MongoDB');
});

// Define Player model
const Player = mongoose.model('Player', {
  socketId: String,
  name: String,
  room: String
});

let connectedPlayers = [];

io.on('connection', (socket) => {

  console.log('New client connected');

  const player = new Player({ 
    socketId: socket.id,
    name: 'Guest'
  });

  connectedPlayers.push(player);

  io.emit('newPlayer', player);

  socket.on('disconnect', () => {
    connectedPlayers = connectedPlayers.filter(p => p.socketId !== socket.id);
    io.emit('playerLeft', player.name);
  });

  socket.on('playerMoved', (room) => {
    player.room = room;
    io.emit('playerMoved', player);
  });

});

app.use(express.json());
app.use(express.static('AqeviaClient'));

app.use(authRoutes);
app.use('/characters', characterRoutes);
app.use('/rooms', roomRoutes); 
app.use('/items', itemRoutes);

const PORT = process.env.PORT || 3000;

server.listen(PORT, () => console.log(`Server running on port ${PORT}`));
