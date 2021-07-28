use specs::world::World;
use specs::WorldExt;

pub mod components;
pub use components::*;

macro_rules! register_components {
    ($($name:ident,)*) => {
        #[derive(Debug)]
        pub enum Component {
            $($name(components::$name),)*
        }

        $(
            impl From<Component> for $name {
                fn from(v: Component) -> $name {
                    match v {
                        Component::$name(r) => r,
                        _ => panic!("Expected Component::{}, received {:?}", stringify!($name), v),
                    }
                }
            }

            impl From<$name> for Component {
                fn from(c: $name) -> Component {
                    Self::$name(c)
                }
            }
        )*

        pub fn register_components<W: WorldExt>(mut world: W) -> W {
            $(world.register::<$name>();)*
            world
        }
    }
}

register_components!(
    Position,
    Renderable,
    LeftMover,
    Location,
    Player,
    RevealedTiles,
    EventStream,
    FieldOfView,
);
