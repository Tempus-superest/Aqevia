const mongoose = require('mongoose');

// Schema definition for rooms in the game world
const roomSchema = new mongoose.Schema({
    name: {
        type: String,
        required: true // Name of the room
    },
    description: {
        type: String,
        required: true // Description of the room's appearance and ambiance
    },
    exits: [{
        name: String, // Name of the exit (e.g., "north", "into the cave")
        leadsTo: { type: mongoose.Schema.Types.ObjectId, ref: 'Room' } // Reference to another room this exit leads to
    }],
    items: [{
        type: mongoose.Schema.Types.ObjectId,
        ref: 'Item' // Items located in this room
    }],
    npcs: [{
        type: mongoose.Schema.Types.ObjectId,
        ref: 'NPC' // NPCs present in this room
    }]
});

const Room = mongoose.model('Room', roomSchema);

module.exports = Room;
