#[test]
fn test_header_datatype() {
    use prieto_buffers::PrietoBuffersSerde;

    fn extract_datatype_from_header(header: u8) -> Option<prieto_buffers::FieldType> {
        let data_type = header >> 5;
        prieto_buffers::FieldType::from_u8(data_type)
    }

    let a: u8 = 1;

    let mut a_buffer = [0; 2];
    a.serialize_with_header(0, a_buffer.as_mut_slice());

    assert_eq!(
        prieto_buffers::FieldType::SingleByte,
        extract_datatype_from_header(a_buffer[0]).expect("Invalid data type header")
    );

    let b: i8 = -1;
    let mut b_buffer = [0; 2];
    b.serialize_with_header(1, b_buffer.as_mut_slice());

    assert_eq!(
        prieto_buffers::FieldType::SingleByte,
        extract_datatype_from_header(b_buffer[0]).expect("Invalid data type header")
    );

    let c: u16 = 2;

    let mut c_buffer = [0; 3];
    c.serialize_with_header(2, c_buffer.as_mut_slice());

    assert_eq!(
        prieto_buffers::FieldType::TwoBytes,
        extract_datatype_from_header(c_buffer[0]).expect("Invalid data type header")
    );

    let d: i16 = -2;

    let mut d_buffer = [0; 3];
    d.serialize_with_header(3, d_buffer.as_mut_slice());

    assert_eq!(
        prieto_buffers::FieldType::TwoBytes,
        extract_datatype_from_header(d_buffer[0]).expect("Invalid data type header")
    );

    let e: u32 = 3;

    let mut e_buffer = [0; 5];
    e.serialize_with_header(4, e_buffer.as_mut_slice());

    assert_eq!(
        prieto_buffers::FieldType::FourBytes,
        extract_datatype_from_header(e_buffer[0]).expect("Invalid data type")
    );

    let f: i32 = -3;

    let mut f_buffer = [0; 5];
    f.serialize_with_header(5, f_buffer.as_mut_slice());

    assert_eq!(
        prieto_buffers::FieldType::FourBytes,
        extract_datatype_from_header(f_buffer[0]).expect("Invalid data type header")
    );

    let g: u64 = 4;

    let mut g_buffer = [0; 9];
    g.serialize_with_header(6, g_buffer.as_mut_slice());

    assert_eq!(
        prieto_buffers::FieldType::EightBytes,
        extract_datatype_from_header(g_buffer[0]).expect("Invalid data type header")
    );

    let h: i64 = -4;

    let mut h_buffer = [0; 9];
    h.serialize_with_header(7, h_buffer.as_mut_slice());

    assert_eq!(
        prieto_buffers::FieldType::EightBytes,
        extract_datatype_from_header(h_buffer[0]).expect("Invalid data type header")
    );

    let j: f32 = 33.;

    let mut j_buffer = [0; 5];

    j.serialize_with_header(8, j_buffer.as_mut_slice());

    assert_eq!(
        prieto_buffers::FieldType::FourBytes,
        extract_datatype_from_header(j_buffer[0]).expect("Invalid data type header")
    );

    let k: f64 = 66.;

    let mut k_buffer = [0; 9];

    k.serialize_with_header(9, k_buffer.as_mut_slice());

    assert_eq!(
        prieto_buffers::FieldType::EightBytes,
        extract_datatype_from_header(k_buffer[0]).expect("invalid data type header")
    );
}

#[test]
fn test_header_field_id() {
    use prieto_buffers::PrietoBuffersSerde;

    fn extract_field_id_from_header(header: u8) -> u8 {
        header & 0b00011111
    }

    let a: u8 = 1;

    let mut a_buffer = [0; 2];
    a.serialize_with_header(0, a_buffer.as_mut_slice());

    assert_eq!(0, extract_field_id_from_header(a_buffer[0]));

    let b: i8 = -1;
    let mut b_buffer = [0; 2];
    b.serialize_with_header(1, b_buffer.as_mut_slice());

    assert_eq!(1, extract_field_id_from_header(b_buffer[0]));

    let c: u16 = 2;

    let mut c_buffer = [0; 3];
    c.serialize_with_header(2, c_buffer.as_mut_slice());

    assert_eq!(2, extract_field_id_from_header(c_buffer[0]));

    let d: i16 = -2;

    let mut d_buffer = [0; 3];
    d.serialize_with_header(3, d_buffer.as_mut_slice());

    assert_eq!(3, extract_field_id_from_header(d_buffer[0]));

    let e: u32 = 3;

    let mut e_buffer = [0; 5];
    e.serialize_with_header(4, e_buffer.as_mut_slice());

    assert_eq!(4, extract_field_id_from_header(e_buffer[0]));

    let f: i32 = -3;

    let mut f_buffer = [0; 5];
    f.serialize_with_header(5, f_buffer.as_mut_slice());

    assert_eq!(5, extract_field_id_from_header(f_buffer[0]));

    let g: u64 = 4;

    let mut g_buffer = [0; 9];
    g.serialize_with_header(6, g_buffer.as_mut_slice());

    assert_eq!(6, extract_field_id_from_header(g_buffer[0]));

    let h: i64 = -4;

    let mut h_buffer = [0; 9];
    h.serialize_with_header(7, h_buffer.as_mut_slice());

    assert_eq!(7, extract_field_id_from_header(h_buffer[0]));
}
