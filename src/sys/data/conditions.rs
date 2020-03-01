use super::spawn_group::Conditions;
use specs::prelude::World;

impl Conditions {
    pub fn passed(&self, _ecs: &World) -> bool {
        use Conditions;

        match self {
            Conditions::None => true,
        }
    }
}
