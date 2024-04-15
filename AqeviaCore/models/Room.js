const mongoose = require('mongoose');

const roomSchema = new mongoose.Schema({
    name: {
        type: String,
        required: true
    },
    description: {
        type: String,
        required: true
    },
    exits: [{
        name: String,
        leadsTo: { type: mongoose.Schema.Types.ObjectId, ref: 'Room' }
    }],
    items: [{
        type: mongoose.Schema.Types.ObjectId,
        ref: 'Item'
    }],
    npcs: [{
        type: mongoose.Schema.Types.ObjectId,
        ref: 'NPC'
    }]
});

const Room = mongoose.model('Room', roomSchema);

module.exports = Room;
