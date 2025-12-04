use mlua::prelude::*;   
use std::fs;



fn main() -> LuaResult<()> {
    println!("Hello, world!");

    let lua = unsafe { Lua::unsafe_new() };

    let globals = lua.globals();
    let package_table: LuaTable = globals.get("package")?;
    let cpath: String = package_table.get("cpath")?;


    let script = fs::read_to_string("mylua.lua")
        .expect("无法读取 mylua.lua，请确保文件在当前目录");

    match lua.load(&script).exec() {
        Ok(_) => println!("Lua 脚本执行完成"),
        Err(e) => eprintln!("Lua 执行出错: {}", e),
    }

    Ok(())
}
