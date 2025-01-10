use std::collections::HashMap;

/// Shared functionality for bidirectional character mapping.
pub struct Mapper {
    connections: HashMap<char, char>,
}

impl Mapper {
    pub fn new(connections_input: &[(char, char)]) -> Self {
        let mut connections = HashMap::new();
        // Adding bidirectional mappings.
        for &(k, v) in connections_input {
            connections.insert(k, v);
            connections.insert(v, k);
        }
        Mapper { connections }
    }

    pub fn map(&self, inp: char) -> char {
        // Swapping characters.
        *self.connections.get(&inp).unwrap_or(&inp)
    }
}
