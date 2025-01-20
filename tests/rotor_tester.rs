use enigma_machine::rotor::Rotor;

// Alphabet    ABCDEFGHIJKLMNOPQRSTUVWXYZ
//
// Rotor1      EKMFLGDQVZNTOWYHXUSPAIBRCJ   Q ==> R: notch
// Rotor2      AJDKSIRUXBLHWTMCQGZNPYFVOE   E ==> F: notch
// Rotor3      BDFHJLCPRTXVZNYEIWGAKMUSQO   V ==> W: notch

#[test]
fn constructor_test() {
    let alphabet: [char; 26] = [
        'e', 'k', 'm', 'f', 'l', 'g', 'd', 'q', 'v', 'z', 'n', 't', 'o', 'w', 'y', 'h', 'x', 'u',
        's', 'p', 'a', 'i', 'b', 'r', 'c', 'j',
    ];
    let start = 0;
    let notch_position = 7;
    let rotor = Rotor::new(alphabet.clone(), start, notch_position);
    assert_eq!(rotor.get_current_number(), 0);
    assert_eq!(rotor.get_notch(), 7);
    assert_eq!(rotor.get_letters_list(), alphabet);
}

#[test]
fn get_forward_char_test() {
    let mut rotor1 = Rotor::new(
        [
            'e', 'k', 'm', 'f', 'l', 'g', 'd', 'q', 'v', 'z', 'n', 't', 'o', 'w', 'y', 'h', 'x',
            'u', 's', 'p', 'a', 'i', 'b', 'r', 'c', 'j',
        ],
        0,
        7,
    );
    rotor1.rotate();
    let new_char = 'h';
    let new_char = rotor1.get_forward_char(new_char);
    assert_eq!(new_char, 'd'); // 'h' should map to 'd'.
}

#[test]
fn get_backward_char_test() {
    let mut rotor1 = Rotor::new(
        [
            'e', 'k', 'm', 'f', 'l', 'g', 'd', 'q', 'v', 'z', 'n', 't', 'o', 'w', 'y', 'h', 'x',
            'u', 's', 'p', 'a', 'i', 'b', 'r', 'c', 'j',
        ],
        0,
        7,
    );
    rotor1.rotate();
    let new_char = 'd';
    let new_char = rotor1.get_backward_char(new_char);
    assert_eq!(new_char, 'h'); // 'd' should map to 'h'.
}

#[test]
fn notch_test() {
    // Create two Rotor instance.
    let mut rotor1 = Rotor::new(
        [
            'e', 'k', 'm', 'f', 'l', 'g', 'd', 'q', 'v', 'z', 'n', 't', 'o', 'w', 'y', 'h', 'x',
            'u', 's', 'p', 'a', 'i', 'b', 'r', 'c', 'j',
        ],
        0,
        7,
    );
    let mut rotor2 = Rotor::new(
        [
            'a', 'j', 'd', 'k', 's', 'i', 'r', 'u', 'x', 'b', 'l', 'h', 'w', 't', 'm', 'c', 'q',
            'g', 'z', 'n', 'p', 'y', 'f', 'v', 'o', 'e',
        ],
        0,
        25,
    );

    // Rotating rotor1 7 time for making it to go on notch
    // and trigger rotating rotor2.
    for _ in 0..7 {
        rotor1.rotate();
        if rotor1.is_on_notch() {
            rotor2.rotate();
        }
    }

    let new_char = 'h';
    let new_char = rotor1.get_forward_char(new_char);
    let new_char = rotor2.get_forward_char(new_char);
    assert_eq!(new_char, 'k'); // 'h' should map to 'k'.
}
