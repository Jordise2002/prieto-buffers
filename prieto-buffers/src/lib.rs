#![cfg_attr(not(feature = "std"), no_std)]

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    SingleByte = 0,
    TwoBytes = 1,
    FourBytes = 2,
    EightBytes = 3,
    Struct = 4,
    Array = 5,
    None = 6, //Represents an empty field, used for Option<T> when T is None should not be serialized
}

impl FieldType {
    pub fn get_size(&self) -> usize {
        match self {
            FieldType::SingleByte => 1,
            FieldType::TwoBytes => 2,
            FieldType::FourBytes => 4,
            FieldType::EightBytes => 8,
            FieldType::Struct => 0, // Struct sizes are dynamic and determined by their fields
            FieldType::Array => 0,  // Array sizes are dynamic and determined by their elements
            FieldType::None => 0,
        }
    }

    pub fn from_u8(value: u8) -> Option<FieldType> {
        match value {
            0 => Some(FieldType::SingleByte),
            1 => Some(FieldType::TwoBytes),
            2 => Some(FieldType::FourBytes),
            3 => Some(FieldType::EightBytes),
            4 => Some(FieldType::Struct),
            5 => Some(FieldType::Array),
            6 => Some(FieldType::None),
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

    fn should_serialize(&self) -> bool {
        true
    }

    fn serialize_with_header(&self, field_id: u8, bytes: &mut [u8]) {
        bytes[0] = build_field_header(field_id, self.get_type());
        self.serialize(&mut bytes[1..]);
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

impl<T: PrietoBuffersSerde + Default> PrietoBuffersSerde for Option<T> {
    fn get_size(&self) -> u32 {
        match self {
            Some(value) => value.get_size(),
            None => 0,
        }
    }

    fn get_type(&self) -> FieldType {
        T::default().get_type()
    }

    fn serialize(&self, bytes: &mut [u8]) {
        if let Some(value) = self {
            value.serialize(bytes);
        }
    }

    fn should_serialize(&self) -> bool {
        self.is_some()
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        let mut value = T::default();
        value.deserialize(bytes);
        *self = Some(value);
    }
}

impl<T: PrietoBuffersSerde + Default, const N: usize> PrietoBuffersSerde for [T; N] {
    fn get_size(&self) -> u32 {
        let mut size: u32 = size_of::<u32>() as u32; // Size for the length prefix
        for item in self.iter() {
            size += item.get_size() + 1; // +1 for the field header
        }
        size
    }

    fn get_type(&self) -> FieldType {
        FieldType::Array
    }

    fn serialize(&self, bytes: &mut [u8]) {
        let mut offset = 0;

        let size: u32 = N as u32;
        size.serialize(bytes);
        offset += size_of::<u32>();

        for item in self.iter() {
            item.serialize_with_header(0, bytes[offset as usize..].as_mut());//No need for field id in arrays, order is the id

            offset += item.get_size() as usize + 1;//Account for the header
        }
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        let mut size: u32 = 0;
        size.deserialize(bytes);

        let mut offset = size_of::<u32>();
        let mut iterator = self.iter_mut();

        for _ in 0..size as usize {
            if let Some(element) = iterator.next() {
                offset += 1; // Skip the field header
                element.deserialize(&bytes[offset as usize..]);
                offset += element.get_size() as usize;
            } else {
                break;
            }
        }
    }
}

#[cfg(feature = "std")]
impl<T:PrietoBuffersSerde + Default> PrietoBuffersSerde for Vec<T> {
    fn get_size(&self) -> u32 {
        let mut size: u32 = size_of::<u32>() as u32; // Size for the length prefix
        for item in self.iter() {
            size += item.get_size() + 1; // +1 for the field header
        }
        size
    }

    fn get_type(&self) -> FieldType {
        FieldType::Array
    }

    fn serialize(&self, bytes: &mut [u8]) {
        let mut offset = 0;

        let size: u32 = self.len() as u32;
        size.serialize(bytes);
        offset += size_of::<u32>();

        for item in self.iter() {
            item.serialize_with_header(0, bytes[offset as usize..].as_mut());//No need for field id in arrays, order is the id

            offset += item.get_size() as usize + 1;//Account for the header
        }
    }

    fn deserialize(&mut self, bytes: &[u8]) {
        let mut size: u32 = 0;
        size.deserialize(bytes);

        let mut offset = size_of::<u32>();

        self.clear();

        for _ in 0..size {
            offset += 1;//Skip header

            let mut element: T = Default::default();
            element.deserialize(&bytes[offset as usize..]);
            offset += element.get_size() as usize;

            self.push(element);
        }
    }
}