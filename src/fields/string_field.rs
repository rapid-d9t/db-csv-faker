use neon::declare_types;
use neon::prelude::*;
use rand;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub struct StringField {
    foreign_key_to: Option<String>,
}

declare_types! {
    pub class JsStringField for StringField {
        init(mut cx) {
            let properties = cx.argument::<JsObject>(0)?;

            if let Ok(name) = properties.get(&mut cx, "foreignKeyTo") {
                if let Ok(name) = name.downcast::<JsString>() {
                    return Ok(
                        StringField {
                            foreign_key_to: Some(name.value())
                        }
                    );
                }
            }

            Ok(StringField {
                foreign_key_to: None
            })
        }

        method format(mut cx) {
            let value: String = {
                let this = cx.this();
                let guard = cx.lock();
                let fields = this.borrow(&guard);
                rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect()
            };
            Ok(cx.string(value).upcast())
        }

        method isForeignKeyTo(mut cx) {
            let foreign_key_to = {
                let this = cx.this();
                let guard = cx.lock();
                let fields = this.borrow(&guard);
                fields.foreign_key_to.clone()
            };
            match foreign_key_to {
                Some(name) => Ok(cx.string(name).upcast()),
                None => Ok(cx.undefined().upcast())
            }
        }
    }
}
