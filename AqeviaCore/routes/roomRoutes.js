const express = require('express');
const router = express.Router();
const Room = require('../models/Room');

// POST route to create a new room
router.post('/create', async (req, res) => {
    try {
        const { name, description, connections } = req.body;
        const newRoom = new Room({ name, description, connections });
        await newRoom.save();
        res.status(201).json(newRoom);
    } catch (error) {
        res.status(400).json({ message: error.message });
    }
});

// GET route to fetch all rooms
router.get('/all', async (req, res) => {
    try {
        const rooms = await Room.find({});
        res.status(200).json(rooms);
    } catch (error) {
        res.status(500).json({ message: error.message });
    }
});

module.exports = router;
