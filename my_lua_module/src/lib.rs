
use mlua::prelude::*;

fn hello(_: &Lua,name: String)-> LuaResult<()>{
    println!("hello,{}!",name);
    Ok(())
}
#[mlua::lua_module]
fn my_module(lua: &Lua) -> LuaResult<LuaTable>{

    let exports = lua.create_table()?;
    exports.set("say_hello",lua.create_function(hello)?)?;
    Ok(exports)
}