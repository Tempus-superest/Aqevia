const mongoose = require('mongoose');

const itemSchema = new mongoose.Schema({
    name: String,
    description: String,
    location: { type: mongoose.Schema.Types.ObjectId, ref: 'Room' } // The room where the item is currently located
});

const Item = mongoose.model('Item', itemSchema);

module.exports = Item;
