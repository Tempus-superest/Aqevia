// Player.js

const mongoose = require('mongoose');

const PlayerSchema = new mongoose.Schema({
  account: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'User'
  },
  characters: [{
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Character'
  }],
  socketId: String,
  currentRoom: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Room'    
  }
});

module.exports = mongoose.model('Player', PlayerSchema);
