const { Integer32Field } = require('./../../');

describe('Integer32Field tests', () => {
  describe('With correct input', () => {
    test('on .format() it returns a random string', async () => {
      const field = new Integer32Field({});
      const fieldWithForeignRelationship = new Integer32Field({ foreignKeyTo: 'Test' });

      expect(field.format()).toMatch(/^[-]{0,1}[0-9]+$/);
      expect(fieldWithForeignRelationship.format()).toMatch(/^[-]{0,1}[0-9]+$/);
    });

    test('on .isForeignKeyTo() it returns a foreign table name or undefined', async () => {
      const field = new Integer32Field({});
      const fieldWithForeignRelationship = new Integer32Field({ foreignKeyTo: 'Test' });

      expect(field.isForeignKeyTo()).toBeUndefined();
      expect(fieldWithForeignRelationship.isForeignKeyTo()).toEqual('Test');
    });
  });

  describe('With missing properties object', () => {
    test('on .init() it throws a error', () => {
      expect(() => {
        new Integer32Field();
      }).toThrow();
    });
  });
});