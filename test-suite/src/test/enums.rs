
#[test]
fn test_enum() {
    use prieto_buffers::PrietoBuffersSerde;

    #[derive(PrietoBuffersSerde)]
    #[repr(u8)]
    enum TestEnum {
        A = 1,
        B = 2,
        C = 3
    }

    let a = TestEnum::A;
    let b:u8 = 1;

    let a_buffer = vec![0; a.get_size() as usize];
    let b_buffer = vec![0; b.get_size() as usize];

    assert_eq!(a_buffer, b_buffer);
}

#[test]
fn test_enum_struct() {
    use prieto_buffers::PrietoBuffersSerde;

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    #[repr(u8)]
    enum TestEnum {
        A = 1,
        B = 2,
        C = 3
    }

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct TestStruct {
        field1: TestEnum,
        field2: u32,
    }

    let test_struct = TestStruct {
        field1: TestEnum::B,
        field2: 42,
    };

    let mut b_struct = TestStruct {
        field1: TestEnum::A,
        field2: 0,
    };

    let mut buffer = vec![0; test_struct.get_size() as usize];

    test_struct.serialize(buffer.as_mut_slice());
    b_struct.deserialize(buffer.as_slice());

    assert_eq!(test_struct, b_struct);
}