use crate::types::Type;

#[derive(Debug)]
pub struct Pokemon {
    pub health: u32,
    pub typ: Type,
    pub damage: u32,
}

impl Pokemon {
    pub fn new() -> Self {
        Self {
            health: 20,
            typ: Type::get_random_type(),
            damage: 10,
        }
    }
}
