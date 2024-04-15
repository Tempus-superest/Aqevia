const mongoose = require('mongoose');

// Schema definition for items that can be interacted with in the game
const ItemSchema = new mongoose.Schema({
    name: { type: String, required: true }, // Name of the item
    description: { type: String, required: true }, // Description of the item's features or use
    room: { type: mongoose.Schema.Types.ObjectId, ref: 'Room' } // Room where the item is located, linking to the Room schema
});

module.exports = mongoose.model('Item', ItemSchema);
