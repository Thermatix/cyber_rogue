use std::collections::HashMap;

#[macro_use]
use super::{Mruby, MrubyImpl, Rc, State};

// entity_group 'basics' do
//   type :items
//   components :pos, :loc
//
//   entity 'sword' do
//     component :renderable do
//       glyph 'i'
//       bgColour Colours::White
//       fgColour Colours::Black
//     end
//   end
// end

type ValueType = crate::game::entity::components::EventValue;

type Component = HashMap<String, ValueType>;
type Components = HashMap<String, Component>;

pub struct Entity {
    components: Components,
    current_component: Option<String>,
}

ruby_interface!(scripting, mruby, state {

    // entity class
    // let class = mruby.def_class("Entity");
    mrusty_class!(Entity, "Entity", {
        def!("initialize", |_v: i32| {
            Entity { components: HashMap::new(), current_component: None }
        });
    });

    mruby.def_method_for::<Entity, _>("add_component", mrfn!(|mruby, slf: (&mut Entity), name: String; &block| {
        slf.current_component = Some(name.clone);
        slf.components.insert(name, HashMap::new());
        let res = block.call("call", vec![]).unwrap();
        slf.current_component = None;
        res
    }));

    mruby.def_method_for::<Entity, _>("attribute", mrfn!(|mruby, slf: (&mut Entity), attribute: String, value: Any| {
        match slf.components.get_mut(&slf.current_component.unwrap()) {
            Some(component) => component.insert(attribute, value.into()),
            None => todo!() // Log that component doesn't exist
        };
        mruby.bool(true)
    }));
});
