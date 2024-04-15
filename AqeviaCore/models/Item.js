const mongoose = require('mongoose');

const ItemSchema = new mongoose.Schema({
    name: { type: String, required: true },
    description: { type: String, required: true },
    room: { type: mongoose.Schema.Types.ObjectId, ref: 'Room' } // Room where the item is located
});

module.exports = mongoose.model('Item', ItemSchema);
