use std::assert_eq;

#[test]
fn test_zero_ended_string() {
    use prieto_buffers::PrietoBuffersSerde;

    let literal = "Testing 0 ended strings";

    let mut a: [u8; 31] = [0;31];
    let mut b: [u8; 31] = [0; 31];

    a[..literal.len()].copy_from_slice(literal.as_bytes());
    a[literal.len()] = 0;

    let options = prieto_buffers::SerializeOptions {
        is_zero_ended_string: true,
    };

    let size = a.get_size_with_options(options);

    let overhead = if prieto_buffers::features::ARRAY_LEN_SIZE == 0 {
         2
    }
    else {
        prieto_buffers::features::ARRAY_LEN_SIZE + 2
    };

    assert_eq!(size, (literal.len() + overhead) as u32);

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
        a: [u8; 31],
        b: [u8; 31],
    }

    #[derive(PrietoBuffersSerde, Debug, PartialEq)]
    struct TestStructCompatible {
        a: u8,
        b: [u8; 31]
    }

    let mut a = TestStruct {
        a: [1; 31],
        b: [1; 31],
    };
    let mut b = TestStruct {
        a: [1; 31],
        b: [2; 31],
    };

    let mut c = TestStructCompatible {
        a: 87,
        b: [3; 31]
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

    assert_eq!(a, b);

    c.deserialize(output.as_slice());

    assert_eq!(a.b, c.b);
}

#[test]
fn test_zero_ended_array_and_array_compatibility() {
    use prieto_buffers::PrietoBuffersSerde;

    #[derive(PrietoBuffersSerde, Debug, PartialEq)]
    struct TestStruct {
        #[zero_ended]
        a: [u8; 31],
        b: [u8; 31],
    }

    #[derive(PrietoBuffersSerde, Debug, PartialEq)]
    struct TestStructCompatible {
        a: [u8; 31],
        b: [u8; 31]
    }

    let mut a = TestStruct {
        a: [1; 31],
        b: [1; 31],
    };

    let mut b = TestStructCompatible {
        a: [1; 31],
        b: [2; 31],
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

    eprintln!("A: {:?}", a);
    eprintln!("B: {:?}", b);
    eprintln!("Output: {:?}", output);
    
    assert_eq!(a.a, b.a);
    assert_eq!(a.b, b.b);
}