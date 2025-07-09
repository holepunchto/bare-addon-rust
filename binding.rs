#![no_std]

use bare_rust::{
    ffi::{js_env_t, js_value_t},
    Env, Function, Object, String,
};

#[unsafe(no_mangle)]
pub extern "C" fn bare_addon_exports(env: *mut js_env_t, _: *mut js_value_t) -> *mut js_value_t {
    let env = Env::from(env);

    let mut exports = Object::new(&env).unwrap();

    let function = Function::new(&env, |env, _| String::new(&env, "Hello from Rust").unwrap());

    exports
        .set_named_property("hello", function.unwrap())
        .unwrap();

    exports.into()
}
