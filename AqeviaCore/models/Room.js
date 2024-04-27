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
  // This array stores ObjectIds that refer back to other Room documents
  connections: [{
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Room'
  }]
});

// Optionally, you can add methods to the schema for complex operations
roomSchema.methods.addConnection = function(roomId) {
  this.connections.push(roomId);
  return this.save();  // Returns a promise
};

const Room = mongoose.model('Room', roomSchema);

module.exports = Room;

