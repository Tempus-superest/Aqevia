const mongoose = require('mongoose');
const Room = require('../models/Room');
const dbHandler = require('./db-handler'); // Utility to handle DB for tests

beforeAll(async () => await dbHandler.connect());
afterEach(async () => await dbHandler.clearDatabase());
afterAll(async () => await dbHandler.closeDatabase());

test('create room without required field should fail', async () => {
    const roomWithoutRequiredField = new Room({ description: 'No name' });
    let err;
    try {
        await roomWithoutRequiredField.save();
    } catch (error) {
        err = error;
    }
    expect(err).toBeInstanceOf(mongoose.Error.ValidationError);
    expect(err.errors.name).toBeDefined();
});

test('create room with all required fields should succeed', async () => {
    const roomData = { name: 'Mystic Cave', description: 'A dark, damp cave' };
    const validRoom = new Room(roomData);
    const savedRoom = await validRoom.save();

    expect(savedRoom._id).toBeDefined();
    expect(savedRoom.name).toBe(roomData.name);
});
