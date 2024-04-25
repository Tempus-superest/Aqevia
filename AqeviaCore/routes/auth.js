// In /routes/auth.js or wherever you handle authentication
const express = require('express');
const router = express.Router();
const bcrypt = require('bcryptjs');
const User = require('../models/user');
const Character = require('../models/Character');

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

module.exports = router;
