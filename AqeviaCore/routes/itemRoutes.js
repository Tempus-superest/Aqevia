// itemRoutes.js 

const express = require('express');
const router = express.Router();
const Item = require('../models/Item');

// GET all items
router.get('/', async (req, res) => {
  try {
    const items = await Item.find();
    res.json(items); 
  } catch (err) {
    res.status(500).json({ message: err.message});
  }
});

// GET single item
router.get('/:id', async (req, res) => {
  try {
    const item = await Item.findById(req.params.id);
    res.json(item);
  } catch (err) {
    res.status(500).json({ message: err.message});
  }
});

// CREATE a new item
router.post('/', async (req, res) => {
  const item = new Item({
    name: req.body.name,
    description: req.body.description 
  });
   
  try {
    const newItem = await item.save();
    res.status(201).json(newItem);
  } catch (err) {
    res.status(400).json({ message: err.message });
  }
});

// UPDATE an item
router.patch('/:id', async (req, res) => {
   try {
     const updated = await Item.findByIdAndUpdate(
       req.params.id,
       req.body,  
       { new: true }
     );
     res.json(updated);
   } catch (err) {
     res.status(400).json({ message: err.message });
   }
});

// DELETE an item
router.delete('/:id', async (req, res) => {
   try {
     const deleted = await Item.findByIdAndDelete(req.params.id);
     res.json({ message: 'Deleted item' });
   } catch (err) {
     res.status(500).json({ message: err.message });
   }
});

module.exports = router;
