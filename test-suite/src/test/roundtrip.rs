
#[test]
fn test_base_types_roundtrip() {
    use prieto_buffers::PrietoBuffersSerde;

    //Add one of every base type
    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct RoundtripTestStruct {
        a: u8,
        b: i8,
        c: u16,
        d: i16,
        e: u32,
        f: i32,
        g: u64,
        h: i64,
        i: bool,
        j: f32,
        k: f64
    }

    let a = RoundtripTestStruct {
        a: 1,
        b: -2,
        c: 3,
        d: -4,
        e: 5,
        f: -6,
        g: 7,
        h: -8,
        i: true,
        j: 66.,
        k: 33.
    };

    //Init with default values
    let mut b = RoundtripTestStruct {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0,
        g: 0,
        h: 0,
        i: false,
        j: 0.,
        k: 0.
    };

    let mut output = Vec::new();

    let size = a.get_size();

    output.resize(size as usize, 0);

    a.serialize(output.as_mut_slice());

    b.deserialize(output.as_slice());

    assert_eq!(a, b);
}

#[test]
fn test_nested_roundtrip() {
    use prieto_buffers::PrietoBuffersSerde;

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct InnerInnerTestStruct {
        a: u64,
        b: i64,
    }

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct InnerTestStruct {
        c: u8,
        d: InnerInnerTestStruct,
        e: i8,
    }

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct TestStruct {
        f: u32,
        g: InnerTestStruct,
        h: i16
    }

    let a = TestStruct {
        f: 1,
        g: InnerTestStruct { c: 2, d: InnerInnerTestStruct { a: 3, b: 4 }, e: 5 },
        h: 6
    };

    //We init with 0 values
    let mut b = TestStruct {
        f: 0,
        g: InnerTestStruct { c: 0, d: InnerInnerTestStruct { a: 0, b: 0 }, e: 0 },
        h: 0
    };

    let size = a.get_size();

    let mut output = Vec::new();
    output.resize(size as usize, 0);

    a.serialize(output.as_mut_slice());
    b.deserialize(output.as_slice());

    assert_eq!(a, b);
}

#[test]
fn test_empty_struct() {
    use prieto_buffers::PrietoBuffersSerde;

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct EmptyStruct {

    }

    let a = EmptyStruct {};
    let mut b = EmptyStruct {};

    let size = a.get_size();

    let mut output = Vec::new();
    output.resize(size as usize, 0);
    
    a.serialize(output.as_mut_slice());
    b.deserialize(output.as_slice());

    assert_eq!(a, b);
}