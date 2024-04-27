// characterRoutes.js

const express = require('express');
const router = express.Router();
const Character = require('../models/Character'); 

// GET all characters 
router.get('/', async (req, res) => {
  try {
    const characters = await Character.find();
    res.json(characters);
  } catch (err) {
    res.status(500).json({ message: err.message});
  }
});

// GET single character
router.get('/:id', async (req, res) => {
   try {
     const character = await Character.findById(req.params.id);
     res.json(character);
   } catch (err) {
     res.status(500).json({ message: err.message});
   }
});

// CREATE a new character
router.post('/', async (req, res) => {
  const character = new Character({
    name: req.body.name,
    user: req.user._id // assign logged in user
  });

  try {
    const newCharacter = await character.save();
    res.status(201).json(newCharacter); 
  } catch (err) {
    res.status(400).json({ message: err.message });
  }
});

// UPDATE a character
router.patch('/:id', async (req, res) => {
  try {
    const updated = await Character.findByIdAndUpdate(
        req.params.id,
        req.body,
        { new: true }
    );
    res.json(updated);
  } catch (err) {
    res.status(400).json({ message: err.message });
  }
});

// DELETE a character
router.delete('/:id', async (req, res) => {
  try {
    const deleted = await Character.findByIdAndDelete(req.params.id);
    res.json({ message: 'Deleted character' });
  } catch (err) {
    res.status(500).json({ message: err.message });
  }
});

module.exports = router;
