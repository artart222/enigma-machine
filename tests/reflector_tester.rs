use enigma_machine::reflector::Reflector;

#[test]
fn test_reflector_swap_char() {
    // Create reflector instance .
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

    // Test valid mappings.
    assert_eq!(reflector.swap_char('a'), 'e'); // 'a' should map to 'b'.
    assert_eq!(reflector.swap_char('e'), 'a'); // 'b' should map back to 'a'.
}
