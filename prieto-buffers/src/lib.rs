#![cfg_attr(not(feature = "std"), no_std)]

pub mod features;
pub mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SerializeOptions {
    pub is_zero_ended_string: bool,
}

impl Default for SerializeOptions {
    fn default() -> Self {
        SerializeOptions {
            is_zero_ended_string: false,
        }
    }
}

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

#[cfg(feature = "derive")]
pub use prieto_buffers_derive::PrietoBuffersSerde;

pub trait PrietoBuffersSerde {
    fn get_size_with_options(&self, options: SerializeOptions) -> u32;
    fn get_size(&self) -> u32 {
        self.get_size_with_options(SerializeOptions::default())
    }

    fn get_type(&self) -> FieldType;
    fn serialize_with_options(&self, bytes: &mut [u8], options: SerializeOptions);
    fn serialize(&self, bytes: &mut [u8]) {
        self.serialize_with_options(bytes, SerializeOptions::default());
    }

    fn should_serialize(&self) -> bool {
        true
    }

    fn is_zero_end(&self) -> bool {
        false
    }

    fn serialize_with_header(
        &self,
        field_id: u32,
        bytes: &mut [u8],
        options: Option<SerializeOptions>,
    ) {
        let offset = utils::serialize_struct_field_header(field_id, self.get_type(), bytes);
        self.serialize_with_options(
            &mut bytes[offset..],
            options.unwrap_or(SerializeOptions::default()),
        );
    }
    fn deserialize_with_options(&mut self, bytes: &[u8], options: SerializeOptions) -> u32;
    fn deserialize(&mut self, bytes: &[u8]) -> u32 {
        self.deserialize_with_options(bytes, SerializeOptions::default())
    }
}

impl PrietoBuffersSerde for u8 {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<u8>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::SingleByte
    }

    fn serialize_with_options(&self, bytes: &mut [u8], _options: SerializeOptions) {
        bytes[0] = *self;
    }

    fn is_zero_end(&self) -> bool {
        *self == 0
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], _options: SerializeOptions) -> u32 {
        *self = bytes[0];
        size_of::<u8>() as u32
    }
}

impl PrietoBuffersSerde for i8 {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<i8>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::SingleByte
    }

    fn is_zero_end(&self) -> bool {
        *self == 0
    }

    fn serialize_with_options(&self, bytes: &mut [u8], _options: SerializeOptions) {
        bytes[0] = *self as u8;
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], _options: SerializeOptions) -> u32 {
        *self = bytes[0] as i8;
        size_of::<i8>() as u32
    }
}

impl PrietoBuffersSerde for bool {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<bool>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::SingleByte
    }

    fn is_zero_end(&self) -> bool {
        *self == false
    }

    fn serialize_with_options(&self, bytes: &mut [u8], _options: SerializeOptions) {
        bytes[0] = if *self { 1 } else { 0 };
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], _options: SerializeOptions) -> u32 {
        *self = bytes[0] != 0;
        size_of::<bool>() as u32
    }
}

impl PrietoBuffersSerde for u16 {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<u16>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::TwoBytes
    }

    fn serialize_with_options(&self, bytes: &mut [u8], _options: SerializeOptions) {
        bytes[0] = (*self & 0xFF) as u8;
        bytes[1] = ((*self >> 8) & 0xFF) as u8;
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], _options: SerializeOptions) -> u32 {
        *self = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
        size_of::<u16>() as u32
    }
}

impl PrietoBuffersSerde for i16 {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<i16>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::TwoBytes
    }

    fn serialize_with_options(&self, bytes: &mut [u8], _options: SerializeOptions) {
        bytes[0] = (*self & 0xFF) as u8;
        bytes[1] = ((*self >> 8) & 0xFF) as u8;
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], _options: SerializeOptions) -> u32 {
        *self = (bytes[0] as i16) | ((bytes[1] as i16) << 8);
        size_of::<i16>() as u32
    }
}

impl PrietoBuffersSerde for u32 {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<u32>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::FourBytes
    }

    fn serialize_with_options(&self, bytes: &mut [u8], _options: SerializeOptions) {
        bytes[0] = (*self & 0xFF) as u8;
        bytes[1] = ((*self >> 8) & 0xFF) as u8;
        bytes[2] = ((*self >> 16) & 0xFF) as u8;
        bytes[3] = ((*self >> 24) & 0xFF) as u8;
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], _options: SerializeOptions) -> u32 {
        *self = (bytes[0] as u32)
            | ((bytes[1] as u32) << 8)
            | ((bytes[2] as u32) << 16)
            | ((bytes[3] as u32) << 24);
        size_of::<u32>() as u32
    }
}

impl PrietoBuffersSerde for i32 {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<i32>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::FourBytes
    }

    fn serialize_with_options(&self, bytes: &mut [u8], _options: SerializeOptions) {
        bytes[0] = (*self & 0xFF) as u8;
        bytes[1] = ((*self >> 8) & 0xFF) as u8;
        bytes[2] = ((*self >> 16) & 0xFF) as u8;
        bytes[3] = ((*self >> 24) & 0xFF) as u8;
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], _options: SerializeOptions) -> u32 {
        *self = (bytes[0] as i32)
            | ((bytes[1] as i32) << 8)
            | ((bytes[2] as i32) << 16)
            | ((bytes[3] as i32) << 24);
        size_of::<i32>() as u32
    }
}

impl PrietoBuffersSerde for u64 {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<u64>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::EightBytes
    }

    fn serialize_with_options(&self, bytes: &mut [u8], _options: SerializeOptions) {
        for i in 0..size_of::<u64>() {
            bytes[i] = ((*self >> (i * 8)) & 0xFF) as u8;
        }
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], _options: SerializeOptions) -> u32 {
        *self = 0;
        for i in 0..size_of::<u64>() {
            *self |= (bytes[i] as u64) << (i * 8);
        }
        size_of::<u64>() as u32
    }
}

impl PrietoBuffersSerde for i64 {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<i64>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::EightBytes
    }

    fn serialize_with_options(&self, bytes: &mut [u8], _options: SerializeOptions) {
        for i in 0..8 {
            bytes[i] = ((*self >> (i * 8)) & 0xFF) as u8;
        }
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], _options: SerializeOptions) -> u32 {
        *self = 0;
        for i in 0..8 {
            *self |= (bytes[i] as i64) << (i * 8);
        }
        size_of::<i64>() as u32
    }
}

impl PrietoBuffersSerde for f32 {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<f32>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::FourBytes
    }

    fn serialize_with_options(&self, bytes: &mut [u8], options: SerializeOptions) {
        let integer: u32 = self.to_bits();
        integer.serialize_with_options(bytes, options);
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], options: SerializeOptions) -> u32 {
        let mut integer: u32 = 0;
        integer.deserialize_with_options(bytes, options);
        *self = f32::from_bits(integer);
        size_of::<f32>() as u32
    }
}

impl PrietoBuffersSerde for f64 {
    fn get_size_with_options(&self, _options: SerializeOptions) -> u32 {
        size_of::<f64>() as u32
    }

    fn get_type(&self) -> FieldType {
        FieldType::EightBytes
    }

    fn serialize_with_options(&self, bytes: &mut [u8], options: SerializeOptions) {
        let integer: u64 = self.to_bits();
        integer.serialize_with_options(bytes, options);
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], options: SerializeOptions) -> u32 {
        let mut integer: u64 = 0;
        integer.deserialize_with_options(bytes, options);
        *self = f64::from_bits(integer);
        size_of::<f64>() as u32
    }
}

impl<T: PrietoBuffersSerde + Default> PrietoBuffersSerde for Option<T> {
    fn get_size_with_options(&self, options: SerializeOptions) -> u32 {
        match self {
            Some(value) => value.get_size_with_options(options),
            None => 0,
        }
    }

    fn get_type(&self) -> FieldType {
        T::default().get_type()
    }

    fn serialize_with_options(&self, bytes: &mut [u8], options: SerializeOptions) {
        if let Some(value) = self {
            value.serialize_with_options(bytes, options);
        }
    }

    fn should_serialize(&self) -> bool {
        self.is_some()
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], options: SerializeOptions) -> u32 {
        let mut value = T::default();
        let size = value.deserialize_with_options(bytes, options);
        *self = Some(value);
        size
    }
}

mod zero_ended_array {
    use super::*;

    pub fn get_size_with_options<T: PrietoBuffersSerde + Default, const N: usize>(
        array: &[T; N],
        _options: SerializeOptions,
    ) -> u32 {
        let mut size: u32 = (features::ARRAY_LEN_SIZE + size_of::<u8>()) as u32; //Size for the length prefix and data type byte
        for item in array.iter() {
            size += item.get_size_with_options(_options);
            if item.is_zero_end() {
                break;
            }
        }

        size
    }

    pub fn serialize_with_options<T: PrietoBuffersSerde + Default, const N: usize>(
        array: &[T; N],
        bytes: &mut [u8],
        options: SerializeOptions,
    ) {
        let mut len: u32 = 0;
        for item in array.iter() {
            len += 1;
            if item.is_zero_end() {
                break;
            }
        }

        let mut offset = utils::serialize_array_len(len, T::default().get_type(), bytes);

        for item in array.iter() {
            item.serialize_with_options(&mut bytes[offset as usize..], options);
            offset += item.get_size_with_options(options) as usize;
            if item.is_zero_end() {
                break;
            }
        }
    }
}

impl<T: PrietoBuffersSerde + Default, const N: usize> PrietoBuffersSerde for [T; N] {
    fn get_size_with_options(&self, options: SerializeOptions) -> u32 {
        //Handle zero ended arrays
        if options.is_zero_ended_string && T::default().get_type() == FieldType::SingleByte {
            return zero_ended_array::get_size_with_options(self, options);
        }

        let mut size: u32 = (features::ARRAY_LEN_SIZE + size_of::<u8>()) as u32; // Size for the length prefix and type header
        for item in self.iter() {
            size += item.get_size_with_options(options);
        }
        size
    }

    fn get_type(&self) -> FieldType {
        FieldType::Array
    }

    fn serialize_with_options(&self, bytes: &mut [u8], options: SerializeOptions) {
        //Handled zero ended arrays
        if options.is_zero_ended_string && T::default().get_type() == FieldType::SingleByte {
            zero_ended_array::serialize_with_options(self, bytes, options);
            return;
        }

        let mut offset = utils::serialize_array_len(N as u32, T::default().get_type(), bytes);

        for item in self.iter() {
            item.serialize_with_options(bytes[offset as usize..].as_mut(), options);

            offset += item.get_size_with_options(options) as usize;
        }
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], options: SerializeOptions) -> u32 {
        let (size, _field_type, offset) = utils::deserialize_array_len(bytes);

        let mut offset = offset as u32;

        let mut iterator = self.iter_mut();

        for _ in 0..size as usize {
            if let Some(element) = iterator.next() {
                offset += element.deserialize_with_options(&bytes[offset as usize..], options);
            } else {
                break;
            }
        }

        offset
    }
}

#[cfg(feature = "std")]
mod zero_ended_vec {

    use super::*;

    pub fn get_size_with_options<T: PrietoBuffersSerde + Default>(
        vec: &Vec<T>,
        _options: SerializeOptions,
    ) -> u32 {
        let mut size: u32 = (features::ARRAY_LEN_SIZE + size_of::<u8>()) as u32; // Size for the length prefix and type header
        for item in vec.iter() {
            size += item.get_size_with_options(_options);
            if item.is_zero_end() {
                break;
            }
        }

        size
    }

    pub fn serialize_with_options<T: PrietoBuffersSerde + Default>(
        vec: &Vec<T>,
        bytes: &mut [u8],
        options: SerializeOptions,
    ) {
        let mut len: u32 = 0;
        for item in vec.iter() {
            len += 1;
            if item.is_zero_end() {
                break;
            }
        }

        let mut offset = utils::serialize_array_len(len, T::default().get_type(), bytes);

        for item in vec.iter() {
            item.serialize_with_options(&mut bytes[offset as usize..], options);
            offset += item.get_size_with_options(options) as usize;
            if item.is_zero_end() {
                break;
            }
        }
    }
}
#[cfg(feature = "std")]
impl<T: PrietoBuffersSerde + Default> PrietoBuffersSerde for Vec<T> {
    fn get_size_with_options(&self, options: SerializeOptions) -> u32 {
        if options.is_zero_ended_string && T::default().get_type() == FieldType::SingleByte {
            return zero_ended_vec::get_size_with_options(self, options);
        }

        let mut size: u32 = (features::ARRAY_LEN_SIZE + size_of::<u8>()) as u32; // size of the prefix length(4) + type(1)
        for item in self.iter() {
            size += item.get_size_with_options(options);
        }
        size
    }

    fn get_type(&self) -> FieldType {
        FieldType::Array
    }

    fn serialize_with_options(&self, bytes: &mut [u8], options: SerializeOptions) {
        //Handled zero ended arrays
        if options.is_zero_ended_string && T::default().get_type() == FieldType::SingleByte {
            zero_ended_vec::serialize_with_options(self, bytes, options);
            return;
        }

        let mut offset =
            utils::serialize_array_len(self.len() as u32, T::default().get_type(), bytes);

        for item in self.iter() {
            item.serialize_with_options(&mut bytes[offset as usize..], options);

            offset += item.get_size_with_options(options) as usize;
        }
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], options: SerializeOptions) -> u32 {
        let (size, _field_type, offset) = utils::deserialize_array_len(bytes);

        let mut offset = offset as u32;

        self.clear();

        for _ in 0..size {
            let mut element: T = Default::default();
            offset += element.deserialize_with_options(&bytes[offset as usize..], options);

            self.push(element);
        }

        offset
    }
}

#[cfg(feature = "std")]
impl PrietoBuffersSerde for String {
    fn get_size_with_options(&self, options: SerializeOptions) -> u32 {
        let self_vec: Vec<u8> = self.bytes().collect();
        self_vec.get_size_with_options(options)
    }

    fn get_type(&self) -> FieldType {
        FieldType::Array
    }

    fn serialize_with_options(&self, bytes: &mut [u8], options: SerializeOptions) {
        let self_vec: Vec<u8> = self.bytes().collect();
        self_vec.serialize_with_options(bytes, options);
    }

    fn deserialize_with_options(&mut self, bytes: &[u8], options: SerializeOptions) -> u32 {
        let mut self_vec: Vec<u8> = Vec::new();
        let size = self_vec.deserialize_with_options(bytes, options);
        *self = String::from_utf8(self_vec).expect("Invalid UTF-8");
        size
    }
}
