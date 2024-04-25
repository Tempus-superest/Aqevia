const express = require('express');
const router = express.Router();
const Room = require('../models/Room');

// POST route to create a new room
router.post('/create', async (req, res) => {
    try {
        const { name, description } = req.body;
        const newRoom = new Room({ name, description });
        await newRoom.save();
        res.status(201).json(newRoom);
    } catch (error) {
        res.status(400).json({ message: error.message });
    }
});

// GET route to fetch all rooms
router.get('/all', async (req, res) => {
    try {
        const rooms = await Room.find({}).populate('connections');
        res.status(200).json(rooms);
    } catch (error) {
        res.status(500).json({ message: error.message });
    }
});

// POST route to connect two rooms
router.post('/connect', async (req, res) => {
    const { roomId, connectToRoomId } = req.body;
    try {
        const room = await Room.findById(roomId);
        if (!room) {
            return res.status(404).send('Room not found');
        }
        // Use the addConnection method from the Room model
        await room.addConnection(connectToRoomId);
        res.status(200).send(`Room ${roomId} now connected to ${connectToRoomId}`);
    } catch (error) {
        res.status(500).send('Error connecting rooms: ' + error.message);
    }
});

module.exports = router;
