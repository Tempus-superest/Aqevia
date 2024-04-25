const express = require('express');
const router = express.Router();
const Item = require('../models/Item');
const Character = require('../models/Character');

// Route to pick up an item
router.post('/pickup', async (req, res) => {
    const { characterId, itemId } = req.body;
    try {
        const item = await Item.findById(itemId);
        if (!item) {
            return res.status(404).send('Item not found');
        }

        const character = await Character.findById(characterId);
        if (!character) {
            return res.status(404).send('Character not found');
        }

        // Add item to character's inventory and remove from room
        character.inventory.push(item._id);
        item.location = null;
        await character.save();
        await item.save();

        res.send('Item picked up');
    } catch (error) {
        res.status(500).send('Error picking up item: ' + error.message);
    }
});

// Similarly, you can add a route for dropping items

module.exports = router;
