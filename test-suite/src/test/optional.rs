
#[test]
fn test_optional() {
    use prieto_buffers::PrietoBuffersSerde;

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct TestStruct {
        a: Option<u8>
    }

    let test_struct = TestStruct { a: Some(42) };

    let size = test_struct.get_size();
    let mut output = Vec::new();
    output.resize(size as usize, 0);

    test_struct.serialize(&mut output);

    assert_eq!(size, 3, "{:?}", output);

    let empty_test_struct = TestStruct { a: None };

    let empty_size = empty_test_struct.get_size();

    assert_eq!(empty_size, 1);
}