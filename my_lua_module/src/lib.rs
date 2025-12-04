use mlua_sys::lua54::lua;
use mlua_sys::lua54::lua::*;
use std::os::raw::c_int;

// --------------------------
// C 回调包装
// --------------------------

unsafe extern "C-unwind" fn rust_add(state: *mut lua_State) -> c_int {
    let a = lua_tonumberx(state, 1, std::ptr::null_mut());
    let b = lua_tonumberx(state, 2, std::ptr::null_mut());
    lua_pushnumber(state, a + b);
    return 1;
}

unsafe extern "C-unwind" fn say_hello(state: *mut lua_State) -> c_int {
    //println!("Hello from Rust!");
    lua_pushstring(state, b"Hello from Rust!\0".as_ptr() as *const _);
    1
}
// --------------------------
// Lua require("hello") 时执行的入口
// --------------------------


#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_libhello(L: *mut lua_State) -> c_int {
    lua_newtable(L);

    lua_pushcfunction(L, rust_add);
    lua_setfield(L, -2, b"add\0".as_ptr() as _);
    lua_pushcfunction(L, say_hello);
    lua_setfield(L, -2, b"say_hello\0".as_ptr() as _);
    1
}