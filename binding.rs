use bare_rust::{bare_exports, Env, Function, Object, String};

bare_exports!(bare_addon_exports, |env| {
    let mut exports = Object::new(&env)?;

    let function = Function::new(&env, |env, _| {
        Ok(String::new(&env, "Hello from Rust")?.into())
    })?;

    exports.set_named_property("hello", function)?;

    Ok(exports.into())
});
