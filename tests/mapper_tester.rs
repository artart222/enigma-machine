use enigma_machine::mapper::Mapper; // Adjust the path as needed

#[test]
fn test_bidirectional_mapping() {
    let connections = vec![('a', 'b'), ('c', 'd'), ('e', 'f')];
    let mapper = Mapper::new(&connections);

    // Test forward mapping
    assert_eq!(mapper.map('a'), 'b');
    assert_eq!(mapper.map('c'), 'd');
    assert_eq!(mapper.map('e'), 'f');

    // Test reverse mapping (bidirectional)
    assert_eq!(mapper.map('b'), 'a');
    assert_eq!(mapper.map('d'), 'c');
    assert_eq!(mapper.map('f'), 'e');

    // Test a character not in the mapping
    assert_eq!(mapper.map('z'), 'z'); // 'z' maps to itself
}
