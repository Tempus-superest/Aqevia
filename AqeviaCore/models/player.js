const mongoose = require('mongoose');
const Room = require('./Room');
const Item = require('./Item');

// Schema definition for game players
const PlayerSchema = new mongoose.Schema({
    user: {
        type: mongoose.Schema.Types.ObjectId,
        ref: 'User',
        required: true // Link to the User schema for authentication
    },
    currentRoom: {
        type: mongoose.Schema.Types.ObjectId,
        ref: 'Room',
        required: true // Current location of the player in the game world
    },
    inventory: [{
        type: mongoose.Schema.Types.ObjectId,
        ref: 'Item',
        required: false // List of items the player currently holds
    }],
    stats: {
        health: {
            type: Number,
            default: 100 // Default health at the start
        },
        level: {
            type: Number,
            default: 1 // Default level at the start
        },
        experience: {
            type: Number,
            default: 0 // Experience points earned by the player
        }
    }
});

module.exports = mongoose.model('Player', PlayerSchema);
