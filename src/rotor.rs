const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub struct Rotor {
    letters_list: [char; 26],
    current_number: usize,
    notch: usize,
}

impl Rotor {
    pub fn new(letters_list: [char; 26], current_number: usize, notch: usize) -> Self {
        Rotor {
            letters_list,
            current_number,
            notch,
        }
    }

    pub fn rotate(&mut self) {
        self.current_number = (self.current_number + 1) % 26;
    }

    pub fn is_on_notch(&self) -> bool {
        if self.current_number == self.notch {
            true
        } else {
            false
        }
    }

    pub fn get_forward_char(&self, inp_char: char) -> char {
        let rotor_index = ALPHABET.iter().position(|&r| r == inp_char).unwrap() as usize;
        self.letters_list[rotor_index - self.current_number]
    }

    // TODO: Refactor this function.
    pub fn get_backward_char(&self, inp_char: char) -> char {
        let mut rotor_index = self
            .letters_list
            .iter()
            .position(|&r| r == inp_char)
            .unwrap() as usize;
        if rotor_index + self.current_number == 26 {
            rotor_index = rotor_index - 26 + self.current_number;
        }
        ALPHABET[rotor_index + self.current_number]
    }
}
