pub mod dumpers;
pub mod fields;
pub mod tables;

use fields::integer_32_field::JsInteger32Field;
use fields::string_field::JsStringField;
use fields::uuid_v4_field::JsUUIDV4Field;
use neon::register_module;

register_module!(mut m, {
    m.export_class::<JsUUIDV4Field>("UUIDV4Field")?;
    m.export_class::<JsInteger32Field>("Integer32Field")?;
    m.export_class::<JsStringField>("StringField")?;
    Ok(())
});
