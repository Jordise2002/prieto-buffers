#[test]
fn test_header_datatype() {
    use prieto_buffers::PrietoBuffersSerde;

    let a: u8 = 1;

    let mut a_buffer = [0; 500];
    a.serialize_with_header(0, a_buffer.as_mut_slice(), None);

    let (_field_id, field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(a_buffer.as_slice());
    assert_eq!(
        prieto_buffers::FieldType::SingleByte,
        field_type
    );

    let b: i8 = -1;
    let mut b_buffer = [0; 500];
    b.serialize_with_header(1, b_buffer.as_mut_slice(), None);
    let (_field_id, field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(b_buffer.as_slice());
    assert_eq!(
        prieto_buffers::FieldType::SingleByte,
        field_type
    );

    let c: u16 = 2;

    let mut c_buffer = [0; 500];
    c.serialize_with_header(2, c_buffer.as_mut_slice(), None);

    let (_field_id, field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(c_buffer.as_slice());
    assert_eq!(
        prieto_buffers::FieldType::TwoBytes,
        field_type
    );

    let d: i16 = -2;

    let mut d_buffer = [0; 500];
    d.serialize_with_header(3, d_buffer.as_mut_slice(), None);

    let (_field_id, field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(d_buffer.as_slice());
    assert_eq!(
        prieto_buffers::FieldType::TwoBytes,
        field_type
    );

    let e: u32 = 3;

    let mut e_buffer = [0; 500];
    e.serialize_with_header(4, e_buffer.as_mut_slice(), None);

    let (_field_id, field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(e_buffer.as_slice());
    assert_eq!(
        prieto_buffers::FieldType::FourBytes,
        field_type
    );

    let f: i32 = -3;

    let mut f_buffer = [0; 500];
    f.serialize_with_header(5, f_buffer.as_mut_slice(), None);

    let (_field_id, field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(f_buffer.as_slice());
    assert_eq!(
        prieto_buffers::FieldType::FourBytes,
        field_type
    );

    let g: u64 = 4;

    let mut g_buffer = [0; 500];
    g.serialize_with_header(6, g_buffer.as_mut_slice(), None);

    let (_field_id, field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(g_buffer.as_slice());
    assert_eq!(
        prieto_buffers::FieldType::EightBytes,
        field_type
    );

    let h: i64 = -4;

    let mut h_buffer = [0; 500];
    h.serialize_with_header(7, h_buffer.as_mut_slice(), None);

    let (_field_id, field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(h_buffer.as_slice());
    assert_eq!(
        prieto_buffers::FieldType::EightBytes,
        field_type
    );

    let j: f32 = 33.;

    let mut j_buffer = [0; 500];

    j.serialize_with_header(8, j_buffer.as_mut_slice(), None);

    let (_field_id, field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(j_buffer.as_slice());
    assert_eq!(
        prieto_buffers::FieldType::FourBytes,
        field_type
    );

    let k: f64 = 66.;

    let mut k_buffer = [0; 500];

    k.serialize_with_header(9, k_buffer.as_mut_slice(), None);

    let (_field_id, field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(k_buffer.as_slice());
    assert_eq!(
        prieto_buffers::FieldType::EightBytes,
        field_type
    );
}

#[test]
fn test_header_field_id() {
    use prieto_buffers::PrietoBuffersSerde;

    let a: u8 = 1;

    let mut a_buffer = [0; 500];
    a.serialize_with_header(0, a_buffer.as_mut_slice(), None);

    let (field_id, _field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(a_buffer.as_slice());
    assert_eq!(0, field_id);

    let b: i8 = -1;
    let mut b_buffer = [0; 500];
    b.serialize_with_header(1, b_buffer.as_mut_slice(), None);

    let (field_id, _field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(b_buffer.as_slice());
    assert_eq!(1, field_id);

    let c: u16 = 2;

    let mut c_buffer = [0; 500];
    c.serialize_with_header(2, c_buffer.as_mut_slice(), None);

    let (field_id, _field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(c_buffer.as_slice());
    assert_eq!(2, field_id);

    let d: i16 = -2;

    let mut d_buffer = [0; 500];
    d.serialize_with_header(3, d_buffer.as_mut_slice(), None);

    let (field_id, _field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(d_buffer.as_slice());
    assert_eq!(3, field_id);

    let e: u32 = 3;

    let mut e_buffer = [0; 500];
    e.serialize_with_header(4, e_buffer.as_mut_slice(), None);

    let (field_id, _field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(e_buffer.as_slice());
    assert_eq!(4, field_id);

    let f: i32 = -3;

    let mut f_buffer = [0; 500];
    f.serialize_with_header(5, f_buffer.as_mut_slice(), None);

    let (field_id, _field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(f_buffer.as_slice());
    assert_eq!(5, field_id);

    let g: u64 = 4;

    let mut g_buffer = [0; 500];
    g.serialize_with_header(6, g_buffer.as_mut_slice(), None);

    let (field_id, _field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(g_buffer.as_slice());
    assert_eq!(6, field_id);

    let h: i64 = -4;

    let mut h_buffer = [0; 500];
    h.serialize_with_header(7, h_buffer.as_mut_slice(), None);

    let (field_id, _field_type, _offset) = prieto_buffers::utils::deserialize_struct_field_header(h_buffer.as_slice());
    assert_eq!(7, field_id);
}
