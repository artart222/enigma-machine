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

use std::collections::HashMap;

struct Rotor {
    letters_list: [char; 26],
    current_number: usize,
    notch: usize,
}

fn main() {
    let alphabet: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    // TODO: Check if there is better way to do this.
    let plugboard: HashMap<char, char> = HashMap::from([
        ('a', 'b'),
        ('b', 'a'),
        ('c', 'd'),
        ('d', 'c'),
        ('e', 'f'),
        ('f', 'e'),
        ('g', 'h'),
        ('h', 'g'),
        ('i', 'j'),
        ('j', 'i'),
    ]);

    // TODO: Check if is better way to do this.
    let reflector: HashMap<char, char> = HashMap::from([
        //             A <==> E
        //             B <==> J
        //             C <==> M
        //             D <==> Z
        //             F <==> L
        //             G <==> Y
        //             H <==> X
        //             I <==> V
        //             K <==> W
        //             N <==> R
        //             Q <==> O
        //             P <==> U
        //             S <==> T
        //
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
        //
        ('e', 'a'),
        ('j', 'b'),
        ('m', 'c'),
        ('z', 'd'),
        ('l', 'f'),
        ('y', 'g'),
        ('x', 'h'),
        ('v', 'i'),
        ('w', 'k'),
        ('r', 'n'),
        ('o', 'q'),
        ('u', 'p'),
        ('t', 's'),
    ]);

    let mut rotor1 = Rotor {
        letters_list: [
            'e', 'k', 'm', 'f', 'l', 'g', 'd', 'q', 'v', 'z', 'n', 't', 'o', 'w', 'y', 'h', 'x',
            'u', 's', 'p', 'a', 'i', 'b', 'r', 'c', 'j',
        ],
        current_number: 0,
        notch: 7,
    };
    let mut rotor2 = Rotor {
        letters_list: [
            'a', 'j', 'd', 'k', 's', 'i', 'r', 'u', 'x', 'b', 'l', 'h', 'w', 't', 'm', 'c', 'q',
            'g', 'z', 'n', 'p', 'y', 'f', 'v', 'o', 'e',
        ],
        current_number: 0,
        notch: 25,
    };
    let mut rotor3 = Rotor {
        letters_list: [
            'b', 'd', 'f', 'h', 'j', 'l', 'c', 'p', 'r', 't', 'x', 'v', 'z', 'n', 'y', 'e', 'i',
            'w', 'g', 'a', 'k', 'm', 'u', 's', 'q', 'o',
        ],
        current_number: 0,
        notch: 11,
    };

    let input: &str = "hello";

    for char in input.chars() {
        let mut new_char = char;

        // println!("Start ==> Char: {}    New Char: {}", char, new_char);

        match plugboard.get(&new_char) {
            Some(charachter) => new_char = *charachter,
            _ => new_char = new_char,
        }
        // println!("{}", new_char);
        // println!(
        //     "After first plugboard ==> Char: {}    New Char: {}",
        //     char, new_char
        // );
        // First plugboard.

        let rotor1_index = alphabet.iter().position(|&r| r == new_char).unwrap() as usize;

        rotor1.current_number += 1;
        if rotor1.current_number == 26 {
            rotor1.current_number = 0;
        }

        if rotor1.notch == rotor1.current_number {
            rotor2.current_number += 1;
            if rotor2.current_number == 26 {
                rotor2.current_number = 0;
            }
        }

        if rotor2.notch == rotor2.current_number {
            rotor3.current_number += 1;
            if rotor3.current_number == 26 {
                rotor3.current_number = 0;
            }
        }

        // println!("Rotor 1 current number: {}", rotor1.current_number);
        // println!("Rotor 2 current number: {}", rotor2.current_number);
        // println!("Rotor 3 current number: {}", rotor3.current_number);

        new_char = rotor1.letters_list[rotor1_index - rotor1.current_number];
        // println!("After Rotor I ==> New Char: {}", new_char);
        let rotor2_index = alphabet.iter().position(|&r| r == new_char).unwrap() as usize;
        new_char = rotor2.letters_list[rotor2_index - rotor2.current_number];
        // println!("After Rotor II ==> New Char: {}", new_char);
        let rotor3_index = alphabet.iter().position(|&r| r == new_char).unwrap() as usize;
        new_char = rotor3.letters_list[rotor3_index - rotor3.current_number];
        // println!("After Rotor III ==> New Char: {}", new_char);
        // println!("After first phase of rotors ==> New Char: {}", new_char);
        // End of first phase of rotors.

        match reflector.get(&new_char) {
            Some(charachter) => new_char = *charachter,
            _ => new_char = new_char,
        }
        // println!("After reflector ==> New Char: {}", new_char);
        // End of reflector code.

        let mut rotor3_index = rotor3
            .letters_list
            .iter()
            .position(|&r| r == new_char)
            .unwrap() as usize;
        if rotor3_index + rotor3.current_number == 26 {
            rotor3_index = rotor3_index - 26 + rotor3.current_number;
        }
        new_char = alphabet[rotor3_index + rotor3.current_number];

        let mut rotor2_index = rotor2
            .letters_list
            .iter()
            .position(|&r| r == new_char)
            .unwrap() as usize;
        if rotor2_index + rotor2.current_number == 26 {
            rotor2_index = rotor2_index - 26 + rotor2.current_number;
        }
        new_char = alphabet[rotor2_index + rotor2.current_number];

        let mut rotor1_index = rotor1
            .letters_list
            .iter()
            .position(|&r| r == new_char)
            .unwrap() as usize;
        if rotor1_index + rotor1.current_number == 26 {
            rotor1_index = rotor1_index - 26 + rotor1.current_number;
        }

        new_char = alphabet[rotor1_index + rotor1.current_number];
        // println!("After second phase of rotors ==> New Char: {}", new_char);
        // End of second phase of rotors.

        match plugboard.get(&new_char) {
            Some(charachter) => new_char = *charachter,
            _ => new_char = new_char,
        }
        // println!("{}", new_char);
        // println!("After second plugboard ==> New Char: {}", new_char);
        // Second plugboard.

        println!("Char: {}    New Char: {}", char, new_char);
        // println!("====================================================================================================")
    }
}
