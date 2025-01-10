use crate::mapper::Mapper;

pub struct Plugboard {
    mapper: Mapper,
}

impl Plugboard {
    pub fn new(connections_input: [(char, char); 5]) -> Self {
        let mapper = Mapper::new(&connections_input);
        Plugboard { mapper }
    }

    pub fn swap_char(&self, inp: char) -> char {
        self.mapper.map(inp)
    }
}
