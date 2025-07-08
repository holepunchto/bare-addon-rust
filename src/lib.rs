use bare_rust::{
    ffi::{js_env_t, js_value_t},
    Env, Object, String,
};

#[unsafe(no_mangle)]
pub extern "C" fn bare_addon_exports(
    env: *mut js_env_t,
    exports: *mut js_value_t,
) -> *mut js_value_t {
    let env = Env::from(env);

    let mut exports = Object::new(&env).unwrap();

    exports
        .set_named_property("hello", String::new(&env, "Hello from Rust!").unwrap())
        .unwrap();

    exports.into()
}
