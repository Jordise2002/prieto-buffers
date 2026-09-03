#[test]
fn test_partial_serialization() {
    use prieto_buffers::PrietoBuffersSerde;

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct TestStruct {
        #[field_id(1)]
        a: u32,
        #[field_id(2)]
        b: u8,
        #[field_id(3)]
        c: [u16; 31]
    }

    let mut test_struct = TestStruct {
        a: 0,
        b: 0,
        c: [0; 31]
    };

    let a_test_struct = TestStruct {
        a: 666,
        b: 0,
        c: [0; 31]
    };

    let b_test_struct = TestStruct {
        a: 0,
        b: 42,
        c: [0; 31]
    };

    let c_test_struct = TestStruct {
        a: 0,
        b: 0,
        c: [1; 31]
    };

    let mut a_buffer = vec![];
    a_buffer.resize(a_test_struct.get_size_field(1) as usize, 0);

    a_test_struct.serialize_field(1, a_buffer.as_mut_slice());

    let mut b_buffer = vec![];
    b_buffer.resize(b_test_struct.get_size_field(2) as usize, 0);

    b_test_struct.serialize_field(2, b_buffer.as_mut_slice());

    let mut c_buffer = vec![];
    c_buffer.resize(c_test_struct.get_size_field(3) as usize, 0);

    c_test_struct.serialize_field(3, c_buffer.as_mut_slice());

    test_struct.deserialize(a_buffer.as_slice());
    test_struct.deserialize(b_buffer.as_slice());
    test_struct.deserialize(c_buffer.as_slice());

    assert_eq!(test_struct.a, a_test_struct.a);
    assert_eq!(test_struct.b, b_test_struct.b);
    assert_eq!(test_struct.c, c_test_struct.c);
}