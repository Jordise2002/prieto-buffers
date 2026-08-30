#[test]
fn test_zero_ended_string() {
    use prieto_buffers::PrietoBuffersSerde;

    let literal = "Testing 0 ended strings";

    let mut a: [u8; 500] = [0; 500];
    let mut b: [u8; 500] = [0; 500];

    a[..literal.len()].copy_from_slice(literal.as_bytes());
    a[literal.len()] = 0;

    let options = prieto_buffers::SerializeOptions {
        is_zero_ended_string: true,
    };

    let size = a.get_size_with_options(options);

    assert_eq!(size, (literal.len() + 6) as u32);

    let mut output = Vec::new();
    output.resize(size as usize, 0);

    a.serialize_with_options(output.as_mut_slice(), options);
    b.deserialize(&output);

    assert_eq!(a, b);
}

#[test]
fn test_zero_ended_string_struct() {
    use prieto_buffers::PrietoBuffersSerde;

    #[derive(PrietoBuffersSerde, Debug, PartialEq)]
    struct TestStruct {
        #[zero_ended]
        a: [u8; 1024],
        b: [u8; 1024],
    }

    let mut a = TestStruct {
        a: [1; 1024],
        b: [1; 1024],
    };
    let mut b = TestStruct {
        a: [1; 1024],
        b: [2; 1024],
    };

    a.a[0] = 'h' as u8;
    a.a[1] = 'e' as u8;
    a.a[2] = 'l' as u8;
    a.a[3] = 'l' as u8;
    a.a[4] = 'o' as u8;
    a.a[5] = '\0' as u8;

    let mut output = vec![];
    output.resize(a.get_size() as usize, 0);

    a.serialize(output.as_mut_slice());
    b.deserialize(output.as_slice());
}
