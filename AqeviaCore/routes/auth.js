// In /routes/auth.js
const express = require('express');
const router = express.Router();
const bcrypt = require('bcryptjs');
const User = require('../models/user');
const Character = require('../models/Character');

// Registration route
router.post('/register', async (req, res) => {
    const { username, password } = req.body;

    try {
        const hashedPassword = await bcrypt.hash(password, 12);
        const newCharacter = new Character({
            name: "New Adventurer", // Default character name
            health: 100, // Default health value
            location: null // No initial location
        });
        await newCharacter.save();

        const newUser = new User({
            username,
            password: hashedPassword,
            character: newCharacter._id
        });
        await newUser.save();
        res.status(201).json({ message: "User and character registered successfully!" });
    } catch (error) {
        console.error("Registration Error: ", error);
        res.status(500).send("Error registering user and character");
    }
});

// Login route
router.post('/login', async (req, res) => {
    const { username, password } = req.body;

    try {
        // Find the user by username
        const user = await User.findOne({ username }).populate('character');
        if (!user) {
            return res.status(404).send("User not found");
        }

        // Check if the provided password matches the stored hash
        const isMatch = await bcrypt.compare(password, user.password);
        if (!isMatch) {
            return res.status(400).send("Invalid credentials");
        }

        // If login is successful, send user and character info
        res.status(200).json({
            message: "Logged in successfully",
            user: {
                id: user._id,
                username: user.username
            },
            character: user.character
        });
    } catch (error) {
        console.error("Login Error: ", error);
        res.status(500).send("Error logging in");
    }
});

// Logout route
router.post('/logout', (req, res) => {
    req.session.destroy(err => {
        if (err) {
            return res.status(500).send("Error logging out");
        }
        res.status(200).send("Logged out successfully");
    });
});
module.exports = router;

