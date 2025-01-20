use enigma_machine::plugboard::Plugboard;

#[test]
fn test_plugboard_swap_char() {
    // Create a Plugboard instance.
    let plugboard = Plugboard::new([('a', 'b'), ('c', 'd'), ('e', 'f'), ('g', 'h'), ('i', 'j')]);

    // Test valid mappings.
    assert_eq!(plugboard.swap_char('a'), 'b'); // 'a' should map to 'b'.
    assert_eq!(plugboard.swap_char('b'), 'a'); // 'b' should map back to 'a'.

    // Test characters not in the plugboard.
    assert_eq!(plugboard.swap_char('x'), 'x'); // 'x' should be unchanged.
}
