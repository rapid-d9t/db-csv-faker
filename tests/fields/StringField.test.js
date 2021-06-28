const { StringField } = require('./../../');

describe('StringField tests', () => {
  describe('With correct input', () => {
    test('on .format() it returns a random string', async () => {
      const field = new StringField({});
      const fieldWithForeignRelationship = new StringField({ foreignKeyTo: 'Test' });

      expect(field.format()).toMatch(/^[\w]+$/);
      expect(fieldWithForeignRelationship.format()).toMatch(/^[\w]+$/);
    });

    test('on .isForeignKeyTo() it returns a foreign table name or undefined', async () => {
      const field = new StringField({});
      const fieldWithForeignRelationship = new StringField({ foreignKeyTo: 'Test' });

      expect(field.isForeignKeyTo()).toBeUndefined();
      expect(fieldWithForeignRelationship.isForeignKeyTo()).toEqual('Test');
    });
  });

  describe('With missing properties object', () => {
    test('on .init() it throws a error', () => {
      expect(() => {
        new StringField();
      }).toThrow();
    });
  });
});