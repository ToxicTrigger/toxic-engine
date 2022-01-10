use mlua::prelude::*;

pub struct LuaEngine {
    pub lua: Lua
}

impl LuaEngine {
    pub fn init() -> LuaEngine {
        let lua = Lua::new();
        lua.load("print( jit.version .. ' initialized on toxic-engine.' )").exec();

        let mut tmp: LuaEngine = LuaEngine{ lua: lua };
        return tmp;
    }

    pub fn print( self, text: &str ) -> LuaResult<()>
    {
        let message: String = "print(".to_owned() + text + ")";
        self.lua.load( message.as_bytes() ).exec()?;
        Ok(())
    }
}


