const mongoose = require('mongoose');

const roomSchema = new mongoose.Schema({
  name: {
    type: String,
    required: true,
    trim: true
  },
  description: {
    type: String,
    required: true
  },
  connections: [{
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Room'
  }]
});

const Room = mongoose.model('Room', roomSchema);

module.exports = Room;
