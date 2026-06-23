#![no_std]

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    SingleByte = 0,
    TwoBytes = 1,
    FourBytes = 2,
    EightBytes = 3,
    Struct = 4
}

impl FieldType {
    pub fn get_size(&self) -> usize {
        match self {
            FieldType::SingleByte => 1,
            FieldType::TwoBytes => 2,
            FieldType::FourBytes => 4,
            FieldType::EightBytes => 8,
            FieldType::Struct => 0// Struct sizes are dynamic and determined by their fields
        }
    }

    pub fn from_u8(value: u8) -> Option<FieldType> {
        match value {
            0 => Some(FieldType::SingleByte),
            1 => Some(FieldType::TwoBytes),
            2 => Some(FieldType::FourBytes),
            3 => Some(FieldType::EightBytes),
            4 => Some(FieldType::Struct),
            _ => None,
        }
    }
}

fn build_field_header(field_id: u8, field_type: FieldType) -> u8 {
    if field_id > 31 {
        panic!("Field ID must be between 0 and 31");
    }

    let field_type_bits = (field_type as u8) << 5;
    let field_id_bits = field_id;

    field_type_bits | field_id_bits
}

#[cfg(feature = "derive")]
pub use prieto_buffers_derive::PrietoBuffersSerde;

pub trait PrietoBuffersSerde {
    fn get_size(&self) -> u32;
    fn get_type(&self) -> FieldType;
    fn serialize(&self, bytes: &mut [u8]);
    
    fn serialize_with_header(&self, field_id: u8, bytes: &mut [u8])
    {
        bytes[0] = build_field_header(field_id, self.get_type());
        self.serialize(& mut bytes[1..]);
    }

    fn deserialize(&mut self, bytes: &[u8]);
}

impl PrietoBuffersSerde for u8 {
    fn get_size(&self) -> u32 {
        1
    }

    fn get_type(&self) -> FieldType {
        FieldType::SingleByte
    }

    fn serialize(&self, bytes: &mut [u8]) {
        bytes[0] = *self;
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        *self = bytes[0];
    }
}

impl PrietoBuffersSerde for i8 {
    fn get_size(&self) -> u32 {
        1
    }

    fn get_type(&self) -> FieldType {
        FieldType::SingleByte
    }

    fn serialize(&self, bytes: &mut [u8]) {
        bytes[0] = *self as u8;
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        *self = bytes[0] as i8;
    }
}

impl PrietoBuffersSerde for bool {
    fn get_size(&self) -> u32 {
        1
    }

    fn get_type(&self) -> FieldType {
        FieldType::SingleByte
    }

    fn serialize(&self, bytes: &mut [u8]) {
        bytes[0] = if *self { 1 } else { 0 };
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        *self = bytes[0] != 0;
    }
}

impl PrietoBuffersSerde for u16 {
    fn get_size(&self) -> u32 {
        2
    }

    fn get_type(&self) -> FieldType {
        FieldType::TwoBytes
    }

    fn serialize(&self, bytes: &mut [u8]) {
        bytes[0] = (*self & 0xFF) as u8;
        bytes[1] = ((*self >> 8) & 0xFF) as u8;
    }


    fn deserialize(&mut self, bytes: &[u8]) {
        *self = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
    }
}

impl PrietoBuffersSerde for i16 {
    fn get_size(&self) -> u32 {
        2
    }

    fn get_type(&self) -> FieldType {
        FieldType::TwoBytes
    }

    fn serialize(&self, bytes: &mut [u8]) {
        bytes[0] = (*self & 0xFF) as u8;
        bytes[1] = ((*self >> 8) & 0xFF) as u8;
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        *self = (bytes[0] as i16) | ((bytes[1] as i16) << 8);
    }
}

impl PrietoBuffersSerde for u32 {
    fn get_size(&self) -> u32 {
        4
    }

    fn get_type(&self) -> FieldType {
        FieldType::FourBytes
    }

    fn serialize(&self, bytes: &mut [u8]) {
        bytes[0] = (*self & 0xFF) as u8;
        bytes[1] = ((*self >> 8) & 0xFF) as u8;
        bytes[2] = ((*self >> 16) & 0xFF) as u8;
        bytes[3] = ((*self >> 24) & 0xFF) as u8;
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        *self = (bytes[0] as u32)
            | ((bytes[1] as u32) << 8)
            | ((bytes[2] as u32) << 16)
            | ((bytes[3] as u32) << 24);
    }
}

impl PrietoBuffersSerde for i32 {
    fn get_size(&self) -> u32 {
        4
    }

    fn get_type(&self) -> FieldType {
        FieldType::FourBytes
    }

    fn serialize(&self, bytes: &mut [u8]) {
        bytes[0] = (*self & 0xFF) as u8;
        bytes[1] = ((*self >> 8) & 0xFF) as u8;
        bytes[2] = ((*self >> 16) & 0xFF) as u8;
        bytes[3] = ((*self >> 24) & 0xFF) as u8;
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        *self = (bytes[0] as i32)
            | ((bytes[1] as i32) << 8)
            | ((bytes[2] as i32) << 16)
            | ((bytes[3] as i32) << 24);
    }
}

impl PrietoBuffersSerde for u64 {
    fn get_size(&self) -> u32 {
        8
    }

    fn get_type(&self) -> FieldType {
        FieldType::EightBytes
    }

    fn serialize(&self, bytes: &mut [u8]) {
        for i in 0..8 {
            bytes[i] = ((*self >> (i * 8)) & 0xFF) as u8;
        }
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        *self = 0;
        for i in 0..8 {
            *self |= (bytes[i] as u64) << (i * 8);
        }
    }
}

impl PrietoBuffersSerde for i64 {
    fn get_size(&self) -> u32 {
        8
    }

    fn get_type(&self) -> FieldType {
        FieldType::EightBytes
    }

    fn serialize(&self, bytes: &mut [u8]) {
        for i in 0..8 {
            bytes[i] = ((*self >> (i * 8)) & 0xFF) as u8;
        }
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        *self = 0;
        for i in 0..8 {
            *self |= (bytes[i] as i64) << (i * 8);
        }
    }
}

impl PrietoBuffersSerde for f32 {
    fn get_size(&self) -> u32 {
        4
    }

    fn get_type(&self) -> FieldType {
        FieldType::FourBytes
    }

    fn serialize(&self, bytes: &mut [u8]) {
        let integer: u32 = self.to_bits();
        integer.serialize(bytes);
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        let mut integer: u32 = 0;
        integer.deserialize(bytes);
        *self = f32::from_bits(integer);
    }
}

impl PrietoBuffersSerde for f64 {
    fn get_size(&self) -> u32 {
        8
    }

    fn get_type(&self) -> FieldType {
        FieldType::EightBytes
    }

    fn serialize(&self, bytes: &mut [u8]) {
        let integer: u64 = self.to_bits();
        integer.serialize(bytes);
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        let mut integer: u64 = 0;
        integer.deserialize(bytes);
        *self = f64::from_bits(integer);
    }
}