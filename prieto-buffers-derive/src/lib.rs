use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{Lit, Meta};

fn parse_u32_from_attr(attr: &syn::Attribute) -> Option<u32> {
    match &attr.meta {
        Meta::List(meta_list) => {
            if let Some(_first) = meta_list.tokens.clone().into_iter().next() {
                let lit: Lit = syn::parse2(meta_list.tokens.clone()).ok()?;
                if let Lit::Int(lit_int) = lit {
                    if let Ok(value) = lit_int.base10_parse::<u32>() {
                        if value > u32::MAX {
                            panic!("Field ID must be between 0 and {}", u32::MAX);
                        }

                        return Some(value);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn generate_non_defined_ids(v: Vec<Option<u32>>) -> Option<Vec<u32>> {
    let mut used = HashSet::new();

    for x in v.iter().flatten() {
        used.insert(*x);
    }

    let mut result = Vec::with_capacity(v.len());
    let mut next_id: u32 = 0;

    for slot in v {
        match slot {
            Some(x) => {
                used.insert(x);
                result.push(x);
            }
            None => {
                while next_id <= u32::MAX && used.contains(&next_id) {
                    if next_id == u32::MAX {
                        return None;
                    }
                    next_id += 1;
                }

                if used.contains(&next_id) {
                    return None;
                }

                used.insert(next_id);
                result.push(next_id);
            }
        }
    }

    Some(result)
}

#[proc_macro_derive(PrietoBuffersSerde, attributes(field_id, zero_ended))]
pub fn derive_prieto_buffer_serde(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let attrs = &input.attrs;

    match &input.data {
        syn::Data::Struct(data) => derive_prieto_buffers_serde_struct(name,data),
        syn::Data::Enum(data) => derive_prieto_buffers_serde_enum(name,data, attrs),
        _ => panic!("PrietoBuffersSerde can only be derived for structs and enums"),
    }
}

fn derive_prieto_buffers_serde_struct(struct_name: &proc_macro2::Ident, data: &syn::DataStruct) -> TokenStream {
    let fields = data.fields.iter().collect::<Vec<_>>();

    let field_names: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().expect("Fields must be named"))
        .collect();

    let defined_field_ids = fields
        .iter()
        .map(|field| {
            for attr in &field.attrs {
                if attr.path().is_ident("field_id") {
                    return parse_u32_from_attr(attr);
                }
            }

            return None;
        })
        .collect::<Vec<_>>();

    let is_zero_ended_str = fields
        .iter()
        .map(|field| {
            for attr in &field.attrs {
                if attr.path().is_ident("zero_ended") {
                    return true;
                }
            }
            false
        })
        .collect::<Vec<_>>();

    let field_ids = generate_non_defined_ids(defined_field_ids)
        .expect("Too many fields, not enough IDs available");

    quote! {
        impl #struct_name {
            pub fn get_size_field_with_options(&self, field_id: u32, options: prieto_buffers::SerializeOptions) -> u32 {
                match field_id {
                    #(#field_ids => {
                        let mut options = options.clone();
                        if #is_zero_ended_str {
                            options.is_zero_ended_string = true;
                        }
                        self.#field_names.get_size_with_options(options) + (prieto_buffers::utils::get_struct_header_size() + prieto_buffers::utils::get_struct_len_size()) as u32
                    })*
                    _=> {
                        0//If the field does not exists, return 0
                    }
                }
            }

            pub fn get_size_field(&self, field_id: u32) -> u32 {
                self.get_size_field_with_options(field_id, prieto_buffers::SerializeOptions::default())
            }

            pub fn serialize_field_with_options(&self, field_id: u32, buffer: & mut [u8], options: prieto_buffers::SerializeOptions) {
                let offset = prieto_buffers::utils::serialize_struct_len(1, buffer);
                match field_id {
                    #(#field_ids => {
                        let mut options = options.clone();
                        if #is_zero_ended_str {
                            options.is_zero_ended_string = true;
                        }
                        self.#field_names.serialize_with_header(field_id, & mut buffer[offset as usize..], Some(options));
                    })*
                    _=> {
                        // If the field does not exist, do nothing
                    }
                }
            }

            pub fn serialize_field(&self, field_id: u32, buffer: & mut[u8]) {
                self.serialize_field_with_options(field_id, buffer, prieto_buffers::SerializeOptions::default());
            }

            fn skip_field(bytes: &[u8], field_type: prieto_buffers::FieldType) -> u32 {
                match field_type {
                    prieto_buffers::FieldType::Struct => {
                        let (field_count, offset) = prieto_buffers::utils::deserialize_struct_len(bytes);
                        let mut offset = offset as u32;
                        for _ in 0..field_count {
                            let (_field_id, field_type, field_header_offset) = prieto_buffers::utils::deserialize_struct_field_header(&bytes[offset as usize..]);
                            offset += field_header_offset as u32;
                            offset += #struct_name::skip_field(&bytes[offset as usize..], field_type);
                        }

                        offset
                    }
                    prieto_buffers::FieldType::Array => {
                        let (size, field_type, offset) = prieto_buffers::utils::deserialize_array_len(bytes);
                        let mut offset = offset as u32;

                        for _ in 0..size {
                            offset += #struct_name::skip_field(&bytes[offset as usize..], field_type);
                        }
                        offset
                    }
                    _ => {
                        field_type.get_size() as u32
                    }
                }
            }
        }

        impl PrietoBuffersSerde for #struct_name {
            fn get_size_with_options(&self, options: prieto_buffers::SerializeOptions) -> u32 {
                let mut size = prieto_buffers::utils::get_struct_len_size() as u32;
                #(if self.#field_names.should_serialize() {
                    let mut options = options.clone();
                    if #is_zero_ended_str {
                        options.is_zero_ended_string = true;
                    }
                    size += self.#field_names.get_size_with_options(options) + prieto_buffers::utils::get_struct_header_size() as u32;
                })*
                size
            }

            fn get_type(&self) -> prieto_buffers::FieldType {
                prieto_buffers::FieldType::Struct
            }

            fn serialize_with_options(&self, bytes: &mut [u8], options: prieto_buffers::SerializeOptions) {
                let mut offset = 0;
                let mut field_amount:u32 = 0;
                #(if self.#field_names.should_serialize() {
                    field_amount += 1;
                })*

                offset += prieto_buffers::utils::serialize_struct_len(field_amount, &mut bytes[offset..]);

                #(
                    {
                        let mut options = options.clone();
                        if #is_zero_ended_str {
                            options.is_zero_ended_string = true;
                        }
                        if self.#field_names.should_serialize() {
                            self.#field_names.serialize_with_header(#field_ids, &mut bytes[offset..], Some(options));
                            offset += self.#field_names.get_size_with_options(options) as usize + prieto_buffers::utils::get_struct_header_size();
                        }
                    }
                )*
            }

            fn deserialize_with_options(&mut self, bytes: &[u8], options: prieto_buffers::SerializeOptions) -> u32 {
                let mut counter:u8 = 0;
                let (field_count, mut offset) = prieto_buffers::utils::deserialize_struct_len(bytes);

                for _ in 0..field_count {
                    let (field_id, field_type, header_offset) = prieto_buffers::utils::deserialize_struct_field_header(&bytes[offset as usize..]);
                    offset += header_offset;

                    let field_size = match field_id {
                        #(
                            #field_ids => {
                                if self.#field_names.get_type() == field_type {
                                    let mut options = options.clone();
                                    if #is_zero_ended_str {
                                        options.is_zero_ended_string = true;
                                    }
                                    self.#field_names.deserialize_with_options(&bytes[offset..], options)
                                }
                                else {
                                    #struct_name::skip_field(&bytes[offset..], field_type)
                                }
                            }
                        )*
                        _ => {
                            #struct_name::skip_field(&bytes[offset..], field_type)
                        }
                    };

                    offset += field_size as usize;
                }

                offset as u32
            }
        }
    }.into()
}

fn get_repr(attrs: &[syn::Attribute]) -> syn::Result<syn::Type> {
    for attr in attrs {
        if attr.path().is_ident("repr") {
            let ty: syn::Type = attr.parse_args()?;
            return Ok(ty);
        }
    }

    panic!("Enum must have a repr attribute to derive PrietoBuffersSerde");
    
}

fn derive_prieto_buffers_serde_enum(enum_name: &proc_macro2::Ident, data: &syn::DataEnum, attrs: &Vec<syn::Attribute>) -> TokenStream {
    let is_simple = data.variants.iter().all(|variant| {
        matches!(&variant.fields, syn::Fields::Unit)
    });

    if ! is_simple {
        panic!("PrietoBuffersSerde can only be derived for simple enums");
    }

    let ty = get_repr(attrs).expect("Enum must have a repr attribute to derive PrietoBuffersSerde");

    let variants = &data.variants.iter().map(|variant| {
        variant.ident.clone()
    }).collect::<Vec<_>>();

    quote! {
        impl #enum_name {
    
            fn from_raw(&mut self, raw: #ty) {
                *self = match raw {
                    #(
                        raw_value if raw_value == #enum_name::#variants as #ty => #enum_name::#variants,
                    )*
                    _ => panic!("Invalid enum variant"),
                }
            }

            fn to_raw(&self) -> #ty {
                match self {
                    #(
                        #enum_name::#variants => #enum_name::#variants as #ty,
                    )*
                }
            }
        }

        impl PrietoBuffersSerde for #enum_name {
            fn get_size_with_options(&self, _options: prieto_buffers::SerializeOptions) -> u32 {
                self.to_raw().get_size_with_options(_options)
            }

            fn get_type(&self) -> prieto_buffers::FieldType {
                self.to_raw().get_type()
            }

            fn serialize_with_options(&self, bytes: &mut [u8], options: prieto_buffers::SerializeOptions) {
                let raw = self.to_raw();
                prieto_buffers::PrietoBuffersSerde::serialize_with_options(&raw, bytes, options);
            }

            fn deserialize_with_options(&mut self, bytes: &[u8], options: prieto_buffers::SerializeOptions) -> u32 {
                let mut raw: #ty = 0;
                let size = prieto_buffers::PrietoBuffersSerde::deserialize_with_options(&mut raw, bytes, options);
                self.from_raw(raw);
                size
            }
        }

    }.into()
}