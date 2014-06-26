extern crate libc;
extern crate std;

use super::liblua;
use super::Lua;
use super::Pushable;
use super::Readable;

pub struct UserData<T> {
    value: T
}

impl<T:Clone> UserData<T> {
    pub fn new(val: T) -> UserData<T> {
        UserData{value: val}
    }
}

impl<T> Deref<T> for UserData<T> {
    fn deref<'a>(&'a self)
        -> &'a T
    {
        &self.value
    }
}

impl<T> DerefMut<T> for UserData<T> {
    fn deref_mut<'a>(&'a mut self)
        -> &'a mut T
    {
        &mut self.value
    }
}

// TODO: handle destructors

impl<T:Clone> Pushable for UserData<T> {
    fn push_to_lua(&self, lua: &mut Lua) -> uint {
        let dataRaw = unsafe { liblua::lua_newuserdata(lua.lua, std::mem::size_of_val(&self.value) as libc::size_t) };
        let data: &mut T = unsafe { std::mem::transmute(dataRaw) };
        (*data) = self.value.clone();
        1
    }
}

impl<T:Clone> Readable for UserData<T> {
    fn read_from_lua(lua: &mut Lua, index: i32) -> Option<UserData<T>> {
        // TODO: check type
        let dataPtr = unsafe { liblua::lua_touserdata(lua.lua, index) };
        let data: &T = unsafe { std::mem::transmute(dataPtr) };
        Some(UserData{value: data.clone()})
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn readwrite() {
        let mut lua = super::super::Lua::new();
        let d = super::UserData::new(2);

        lua.set("a", d).unwrap();
        let x: super::UserData<int> = lua.get("a").unwrap();
        assert_eq!(x.value, 2)
    }
}
