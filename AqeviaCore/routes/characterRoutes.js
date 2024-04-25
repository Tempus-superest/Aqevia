const express = require('express');
const router = express.Router();
const Character = require('../models/Character');

// POST route to create a new character
router.post('/create', async (req, res) => {
    try {
        const { name, health, location } = req.body;
        const newCharacter = new Character({ name, health, location });
        await newCharacter.save();
        res.status(201).json(newCharacter);
    } catch (error) {
        res.status(400).json({ message: error.message });
    }
});

// Additional character-related routes can go here

module.exports = router;
