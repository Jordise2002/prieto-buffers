
#[test]
fn test_array() {
    use rand::RngExt;
    use prieto_buffers::PrietoBuffersSerde;

    let mut a: [u8; 500] = [0; 500];
    let mut b: [u8; 500] = [0; 500];

    let mut rng = rand::rng();
    for i in 0..500 {
        a[i] = rng.random();
    }

    let mut a_buffer = vec![];
    a_buffer.resize(a.get_size() as usize, 0);

    a.serialize(&mut a_buffer);
    b.deserialize(&a_buffer);

    assert_eq!(a, b);
}

#[test]
fn test_array_vec() {
    use rand::RngExt;
    use prieto_buffers::PrietoBuffersSerde;

    let mut a: Vec<u8> = vec![];
    let mut b: Vec<u8> = vec![];

    let mut rng = rand::rng();
    for _ in 0..500 {
        a.push(rng.random());
    }

    let mut a_buffer = vec![];
    a_buffer.resize(a.get_size() as usize, 0);

    a.serialize(&mut a_buffer);
    b.deserialize(&a_buffer);

    assert_eq!(a, b);
}

#[test]
fn test_array_struct() {
    use prieto_buffers::PrietoBuffersSerde;
    use rand::RngExt;

    #[derive(PrietoBuffersSerde, PartialEq, Debug, Default)]
    struct InnerTestStruct {
        a: [u8; 3]
    }

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct TestStruct {
        b: [InnerTestStruct; 3]
    }

    let mut rng = rand::rng();
    
    let a: TestStruct = TestStruct { b: [
        InnerTestStruct { a: [rng.random(), rng.random(), rng.random()] },
        InnerTestStruct { a: [rng.random(), rng.random(), rng.random()] },
        InnerTestStruct { a: [rng.random(), rng.random(), rng.random()] },
    ] };

    let mut b: TestStruct = TestStruct { b: [
        InnerTestStruct { a: [0, 0, 0] },
        InnerTestStruct { a: [0, 0, 0] },
        InnerTestStruct { a: [0, 0, 0] },
    ] };


    let size = a.get_size();

    let mut output = Vec::new();
    output.resize(size as usize, 0);

    a.serialize(output.as_mut_slice());
    b.deserialize(output.as_slice());

    assert_eq!(a, b);
}

#[test]
fn test_vec_struct()
{
    use prieto_buffers::PrietoBuffersSerde;
    use rand::RngExt;

    #[derive(PrietoBuffersSerde, PartialEq, Debug, Default)]
    struct InnerTestStruct {
        a: Vec<u8>
    }

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct TestStruct {
        b: Vec<InnerTestStruct>,
        c: u8
    }

    #[derive(PrietoBuffersSerde, PartialEq, Debug)]
    struct TestStructCompatible {
        b: u8,
        c: u8
    }

    let mut rng = rand::rng();
    
    let a: TestStruct = TestStruct { b: vec![
        InnerTestStruct { a: vec![rng.random(), rng.random(), rng.random()] },
        InnerTestStruct { a: vec![rng.random(), rng.random(), rng.random()] },
        InnerTestStruct { a: vec![rng.random(), rng.random(), rng.random()] },
    ], c: rng.random() };

    let mut b: TestStruct = TestStruct { b: vec![
        InnerTestStruct { a: vec![0, 0, 0] },
        InnerTestStruct { a: vec![0, 0, 0] },
        InnerTestStruct { a: vec![0, 0, 0] },
    ], c: 0 };

    let mut c: TestStructCompatible = TestStructCompatible { b: 0, c: 0 };

    let size = a.get_size();

    let mut output = Vec::new();
    output.resize(size as usize, 0);

    a.serialize(output.as_mut_slice());
    b.deserialize(output.as_slice());
    c.deserialize(output.as_slice());

    assert_eq!(a, b);
    eprint!("A: {:?}\n", a);
    eprint!("C: {:?}\n", c);
    eprint!("output: {:?}\n", output);
    
    assert_eq!(a.c, c.c);
}

#[test]
fn test_string() {
    use prieto_buffers::PrietoBuffersSerde;

    let a: String = "Hello, World!".to_string();
    let mut b: String = String::new();


    let size = a.get_size();
    let mut output = Vec::new();
    output.resize(size as usize, 0);

    a.serialize(output.as_mut_slice());
    b.deserialize(output.as_slice());

    assert_eq!(a, b);
}