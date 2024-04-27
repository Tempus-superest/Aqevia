// roomRoutes.js

const express = require('express');
const router = express.Router();
const Room = require('../models/Room');

// GET all rooms
router.get('/', async (req, res) => {
  try {
    const rooms = await Room.find();
    res.json(rooms);
  } catch (err) {
    res.status(500).json({ message: err.message});
  }
}); 

// GET single room
router.get('/:id', async (req, res) => {
  try {
    const room = await Room.findById(req.params.id);
    res.json(room);
  } catch (err) {
    res.status(500).json({ message: err.message});
  }
});

// CREATE a new room
router.post('/', async (req, res) => {
  const room = new Room({
    name: req.body.name,
    description: req.body.description
  });
  
  try {
    const newRoom = await room.save();
    res.status(201).json(newRoom);
  } catch (err) {
    res.status(400).json({ message: err.message });
  }
});

// UPDATE a room
router.patch('/:id', async (req, res) => {
   try {
     const updated = await Room.findByIdAndUpdate(
       req.params.id,
       req.body,  
       { new: true }
     );
     res.json(updated);
   } catch (err) {
     res.status(400).json({ message: err.message });
   }
});

// DELETE a room
router.delete('/:id', async (req, res) => {
   try {
     const deleted = await Room.findByIdAndDelete(req.params.id);
     res.json({ message: 'Deleted room' });
   } catch (err) {
     res.status(500).json({ message: err.message });
   }
});

module.exports = router;
