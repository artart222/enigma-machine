// This is an implementation of Enigma-1 machine.
//
// Enigma machine encryption process:
// Key ==> plugboard ==> rotor1 ==> rotor2 ==> rotor3
// ==> reflector ==> rotor3 ==> rotor2 ==> rotor1
// ==> plugboard ==> lamp
//
// Alphabet    ABCDEFGHIJKLMNOPQRSTUVWXYZ
//
// UKW-A       EJMZALYXVBWFCRQUONTSPIKHGD
// UKW-B       YRUHQSLDPXNGOKMIEBFZCWVJAT
// UKW-C       FVPJIAOYEDRZXWGCTKUQSBNMHL
//
// Rotor1      EKMFLGDQVZNTOWYHXUSPAIBRCJ   Q ==> R: notch
// Rotor2      AJDKSIRUXBLHWTMCQGZNPYFVOE   E ==> F: notch
// Rotor3      BDFHJLCPRTXVZNYEIWGAKMUSQO   V ==> W: notch
//
// If, for example, rotor I is in the b position, an a enters at the letter b,
// which is wired to the k Because of the offset,
// this k enters the next rotor in the j position.
//
// This is example of hello after going trough rotors for first time.
// Hello ==> H ==> D ==> K ==> X
//           E ==> M ==> W ==> U
//           L ==> V ==> Y ==> Q
//           L ==> Q ==> Q ==> I
//           O ==> Z ==> E ==> J

use enigma_machine::plugboard::Plugboard;
use enigma_machine::reflector::Reflector;
use enigma_machine::rotor::Rotor;

fn main() {
    let plugboard: Plugboard =
        Plugboard::new([('a', 'b'), ('c', 'd'), ('e', 'f'), ('g', 'h'), ('i', 'j')]);

    let reflector: Reflector = Reflector::new([
        ('a', 'e'),
        ('b', 'j'),
        ('c', 'm'),
        ('d', 'z'),
        ('f', 'l'),
        ('g', 'y'),
        ('h', 'x'),
        ('i', 'v'),
        ('k', 'w'),
        ('n', 'r'),
        ('q', 'o'),
        ('p', 'u'),
        ('s', 't'),
    ]);

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
    let mut rotor3 = Rotor::new(
        [
            'b', 'd', 'f', 'h', 'j', 'l', 'c', 'p', 'r', 't', 'x', 'v', 'z', 'n', 'y', 'e', 'i',
            'w', 'g', 'a', 'k', 'm', 'u', 's', 'q', 'o',
        ],
        0,
        11,
    );

    let input: &str = "hello";
    // let input: &str = "qnvez";

    for char in input.chars() {
        let mut new_char = char;

        new_char = plugboard.swap_char(new_char);

        rotor1.rotate();
        if rotor1.is_on_notch() {
            rotor2.rotate();
        }
        if rotor2.is_on_notch() {
            rotor3.rotate();
        }

        new_char = rotor1.get_forward_char(new_char);
        new_char = rotor2.get_forward_char(new_char);
        new_char = rotor3.get_forward_char(new_char);

        new_char = reflector.swap_char(new_char);

        new_char = rotor3.get_backward_char(new_char);
        new_char = rotor2.get_backward_char(new_char);
        new_char = rotor1.get_backward_char(new_char);

        new_char = plugboard.swap_char(new_char);

        println!("Char: {}    New Char: {}", char, new_char);
    }
}
