use super::Map;

pub trait MapGenerator {
    fn create_map(&self, map: &mut Map);
}
