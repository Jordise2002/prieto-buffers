use super::*;

pub fn get_struct_len_size() -> usize {
    match features::STRUCT_LEN_SIZE {
        0 | 1 => size_of::<u8>(),
        2 => size_of::<u16>(),
        4 => size_of::<u32>(),
        _ => panic!("Unsupported STRUCT_LEN_SIZE"),
    }
}

pub fn get_struct_header_size() -> usize {
    match features::STRUCT_LEN_SIZE {
        0 => size_of::<u8>(),
        1 => size_of::<u8>() + size_of::<u8>(),
        2 => size_of::<u16>() + size_of::<u8>(),
        4 => size_of::<u32>() + size_of::<u8>(),
        _ => panic!("Unsupported STRUCT_LEN_SIZE"),
    }
}
fn build_single_byte_field_header(field_id: u8, field_type: FieldType) -> u8 {
    if field_id > 31 {
        panic!("Field ID must be between 0 and 31");
    }

    let field_type_bits = (field_type as u8) << 5;
    let field_id_bits = field_id;

    field_type_bits | field_id_bits
}

pub fn serialize_struct_len(len: u32, bytes: &mut [u8]) -> usize {
    match features::STRUCT_LEN_SIZE {
        0 | 1 => {
            (len as u8).serialize(bytes);
            size_of::<u8>()
        }
        2 => {
            (len as u16).serialize(bytes);
            size_of::<u16>()
        }
        4 => {
            (len as u32).serialize(bytes);
            size_of::<u32>()
        }
        _ => panic!("Unsupported STRUCT_LEN_SIZE"),
    }
}

pub fn deserialize_struct_len(bytes: &[u8]) -> (u32, usize) {
    match features::STRUCT_LEN_SIZE {
        0 | 1 => {
            let mut len: u8 = 0;
            len.deserialize(bytes);
            (len as u32, size_of::<u8>())
        }
        2 => {
            let mut len: u16 = 0;
            len.deserialize(bytes);
            (len as u32, size_of::<u16>())
        }
        4 => {
            let mut len: u32 = 0;
            len.deserialize(bytes);
            (len, size_of::<u32>())
        }
        _ => panic!("Unsupported STRUCT_LEN_SIZE"),
    }
}
pub fn serialize_struct_field_header(
    field_id: u32,
    field_type: FieldType,
    bytes: &mut [u8],
) -> usize {
    match features::STRUCT_LEN_SIZE {
        0 => {
            build_single_byte_field_header(field_id as u8, field_type).serialize(bytes);
            size_of::<u8>()
        }
        1 => {
            bytes[0] = field_id as u8;
            bytes[1] = field_type as u8;
            size_of::<u8>() + size_of::<u8>()
        }
        2 => {
            (field_id as u16).serialize(bytes);
            bytes[2] = field_type as u8;
            size_of::<u16>() + size_of::<u8>()
        }
        4 => {
            (field_id as u32).serialize(bytes);
            bytes[4] = field_type as u8;
            size_of::<u32>() + size_of::<u8>()
        }
        _ => panic!("Unsupported STRUCT_LEN_SIZE"),
    }
}

pub fn deserialize_struct_field_header(bytes: &[u8]) -> (u32, FieldType, usize) {
    match features::STRUCT_LEN_SIZE {
        0 => {
            let header = bytes[0];
            let field_id = (header & 0b11111) as u32;
            let field_type = FieldType::from_u8(header >> 5).expect("wrong field type");
            (field_id, field_type, size_of::<u8>())
        }
        1 => {
            let field_id = bytes[0] as u32;
            let field_type = FieldType::from_u8(bytes[1]).expect("wrong field type");
            (field_id, field_type, size_of::<u8>() + size_of::<u8>())
        }
        2 => {
            let mut field_id: u16 = 0;
            field_id.deserialize(bytes);
            let field_type = FieldType::from_u8(bytes[2]).expect("wrong field type");
            (
                field_id as u32,
                field_type,
                size_of::<u16>() + size_of::<u8>(),
            )
        }
        4 => {
            let mut field_id: u32 = 0;
            field_id.deserialize(bytes);
            let field_type = FieldType::from_u8(bytes[4]).expect("wrong field type");
            (field_id, field_type, size_of::<u32>() + size_of::<u8>())
        }
        _ => panic!("Unsupported STRUCT_LEN_SIZE"),
    }
}

pub fn serialize_array_len(len: u32, field_type: FieldType, bytes: &mut [u8]) -> usize {
    match features::ARRAY_LEN_SIZE {
        0 => {
            build_single_byte_field_header(len as u8, field_type).serialize(bytes);
            size_of::<u8>()
        }
        1 => {
            (len as u8).serialize(bytes);
            bytes[1] = field_type as u8;
            size_of::<u8>() + size_of::<u8>()
        }
        2 => {
            (len as u16).serialize(bytes);
            bytes[2] = field_type as u8;
            size_of::<u16>() + size_of::<u8>()
        }
        4 => {
            (len as u32).serialize(bytes);
            bytes[4] = field_type as u8;
            size_of::<u32>() + size_of::<u8>()
        }
        _ => panic!("Unsupported ARRAY_LEN_SIZE"),
    }
}

pub fn deserialize_array_len(bytes: &[u8]) -> (u32, FieldType, usize) {
    match features::ARRAY_LEN_SIZE {
        0 => {
            let header = bytes[0];
            let size = header & 0b11111;
            let field_type = FieldType::from_u8(header >> 5).expect("wrong field type");
            (size as u32, field_type, size_of::<u8>())
        }
        1 => {
            let size = bytes[0];
            let field_type = FieldType::from_u8(bytes[1]).expect("wrong field type");
            (size as u32, field_type, size_of::<u8>() + size_of::<u8>())
        }
        2 => {
            let mut size: u16 = 0;
            size.deserialize(bytes);
            let field_type = FieldType::from_u8(bytes[2]).expect("wrong file type");
            (size as u32, field_type, size_of::<u16>() + size_of::<u8>())
        }
        4 => {
            let mut size: u32 = 0;
            size.deserialize(bytes);
            let field_type = FieldType::from_u8(bytes[4]).expect("wrong file type");
            (size, field_type, size_of::<u32>() + size_of::<u8>())
        }
        _ => panic!("Unsupported ARRAY_LEN_SIZE"),
    }
}
