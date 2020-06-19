use mrusty::{Mruby, MrubyImpl};

use std::rc::Rc;

use crate::sys::State;

#[macro_use]
macro_rules! ruby_interface {
    ($name:ident, $mruby:ident, $state:ident $function_body:block) => {
        pub fn $name($mruby: &mut mrusty::MrubyType, $state: &mut State) $function_body
    }
}

mod interface;

pub fn compile_scripts(mut state: &mut State) {
    let mut mruby = Mruby::new();

    interface::entity::scripting(&mut mruby, &mut state);
}
