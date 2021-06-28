const { UUIDV4Field } = require('./../../');

describe('UUIDV4Field tests', () => {
  describe('With correct input', () => {
    test('on .format() it returns a random string', async () => {
      const field = new UUIDV4Field({});
      const fieldWithForeignRelationship = new UUIDV4Field({ foreignKeyTo: 'Test' });

      const UUIDRegex = /^[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}$/;
      expect(field.format()).toMatch(UUIDRegex);
      expect(fieldWithForeignRelationship.format()).toMatch(UUIDRegex);
    });

    test('on .isForeignKeyTo() it returns a foreign table name or undefined', async () => {
      const field = new UUIDV4Field({});
      const fieldWithForeignRelationship = new UUIDV4Field({ foreignKeyTo: 'Test' });

      expect(field.isForeignKeyTo()).toBeUndefined();
      expect(fieldWithForeignRelationship.isForeignKeyTo()).toEqual('Test');
    });
  });

  describe('With missing properties object', () => {
    test('on .init() it throws a error', () => {
      expect(() => {
        new UUIDV4Field();
      }).toThrow();
    });
  });
});