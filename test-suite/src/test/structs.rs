#[test]
fn test_field_amount() {
    use prieto_buffers::PrietoBuffersSerde;

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct TestEmptyStruct {}

    let a = TestEmptyStruct {};
    let mut a_buffer = [0; 500];

    a.serialize(a_buffer.as_mut_slice());

    let (len, _) = prieto_buffers::utils::deserialize_struct_len(a_buffer.as_slice());
    assert_eq!(0, len);

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct TestStruct {
        a: u8,
        b: i8,
        c: u16,
        d: i16,
        e: u32,
        f: i32,
        g: u64,
        h: i64,
    }

    let b = TestStruct {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0,
        g: 0,
        h: 0,
    };

    let mut b_buffer = Vec::new();
    b_buffer.resize(b.get_size() as usize, 0);
    b.serialize(b_buffer.as_mut_slice());
    
    let (len, _) = prieto_buffers::utils::deserialize_struct_len(b_buffer.as_slice());
    assert_eq!(8, len);
}
