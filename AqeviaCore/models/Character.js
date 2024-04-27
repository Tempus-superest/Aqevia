const mongoose = require('mongoose');

const characterSchema = new mongoose.Schema({
  name: {
    type: String,
    required: true,
    trim: true
  },
  health: {
    type: Number,
    required: true,
    default: 100 // Assuming characters start with full health
  },
  location: {
    type: mongoose.Schema.Types.ObjectId, // References a Room model
    ref: 'Room',
    required: false // Location can be optional for flexibility in initialization
  }
});

const Character = mongoose.model('Character', characterSchema);

module.exports = Character;
