const mongoose = require('mongoose');
const bcrypt = require('bcryptjs');

// Schema definition for user authentication
const UserSchema = new mongoose.Schema({
    username: { type: String, required: true, unique: true }, // Unique username for each user
    password: { type: String, required: true } // Password for user, will be hashed
});

// Encrypt password before saving to database
UserSchema.pre('save', async function(next) {
    if (!this.isModified('password')) return next();
    this.password = await bcrypt.hash(this.password, 8);
});

// Method to compare entered password with stored hashed password
UserSchema.methods.comparePassword = function(candidatePassword) {
    return bcrypt.compare(candidatePassword, this.password);
};

module.exports = mongoose.model('User', UserSchema);
