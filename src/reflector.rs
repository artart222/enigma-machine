use crate::mapper::Mapper;

pub struct Reflector {
    mapper: Mapper,
}

impl Reflector {
    pub fn new(connections_input: [(char, char); 13]) -> Self {
        let mapper = Mapper::new(&connections_input);
        Reflector { mapper }
    }

    pub fn swap_char(&self, inp: char) -> char {
        self.mapper.map(inp)
    }
}
