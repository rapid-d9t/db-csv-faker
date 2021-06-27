use neon::declare_types;
use neon::prelude::*;
use uuid::Uuid;

pub struct UUIDV4Field {
    foreign_key_to: Option<String>,
}

declare_types! {
    pub class JsUUIDV4Field for UUIDV4Field {
        init(mut cx) {
            let properties = cx.argument::<JsObject>(0)?;

            if let Ok(name) = properties.get(&mut cx, "foreignKeyTo") {
                if let Ok(name) = name.downcast::<JsString>() {
                    return Ok(
                        UUIDV4Field {
                            foreign_key_to: Some(name.value())
                        }
                    );
                }
            }

            Ok(UUIDV4Field {
                foreign_key_to: None
            })
        }

        method format(mut cx) {
            let value = {
                let this = cx.this();
                let guard = cx.lock();
                let fields = this.borrow(&guard);
                Uuid::new_v4().to_string()
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
