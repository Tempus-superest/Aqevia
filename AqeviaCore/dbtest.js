require('dotenv').config();
const mongoose = require('mongoose');

// Define a simple schema and model to test database operations
const testSchema = new mongoose.Schema({
    message: String
});
const TestModel = mongoose.model('Test', testSchema);

async function testDatabaseConnection() {
    try {
        // Connect to MongoDB
        await mongoose.connect(process.env.MONGO_URI, {
            useNewUrlParser: true,
            useUnifiedTopology: true
        });
        console.log('Connected to MongoDB successfully.');

        // Create a new document
        const testDoc = new TestModel({ message: 'Hello, World!' });
        await testDoc.save();
        console.log('Document saved:', testDoc);

        // Retrieve the document
        const foundDoc = await TestModel.findOne({ message: 'Hello, World!' });
        console.log('Document retrieved:', foundDoc);

        // Clean up the test document
        await TestModel.deleteMany({});
        console.log('Test documents cleaned up.');

        // Close the connection
        await mongoose.disconnect();
        console.log('Disconnected from MongoDB.');
    } catch (error) {
        console.error('Error connecting to MongoDB:', error);
    }
}

testDatabaseConnection();
