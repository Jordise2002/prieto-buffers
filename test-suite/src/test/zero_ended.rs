use prieto_buffers::PrietoBuffersSerde;

#[test]
fn test_zero_ended_string() {
    let literal = "Testing 0 ended strings";
    
    let mut a: [u8; 500] = [0; 500];
    let mut b: [u8; 500] = [0; 500];

    a[..literal.len()].copy_from_slice(literal.as_bytes());
    a[literal.len()] = 0;

    let options = prieto_buffers::SerializeOptions{
        is_zero_ended_string: true
    };

    let size = a.get_size_with_options(options);
    
    assert_eq!(size, (literal.len()  + 5) as u32);

    let mut output = Vec::new();
    output.resize(size as usize, 0);

    a.serialize_with_options(output.as_mut_slice(), options);
    b.deserialize(&output);

    assert_eq!(a, b);
}