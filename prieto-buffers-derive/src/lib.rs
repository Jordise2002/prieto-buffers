use proc_macro::TokenStream;
use quote::quote;

use syn::{Meta, Lit};

fn parse_u8_from_attr(attr: &syn::Attribute) -> Option<u8> {
    match &attr.meta {
        Meta::List(meta_list) => {
            if let Some(_first) = meta_list.tokens.clone().into_iter().next() {
                let lit: Lit = syn::parse2(meta_list.tokens.clone()).ok()?;
                if let Lit::Int(lit_int) = lit {
                    if let Ok(value) = lit_int.base10_parse::<u8>() {
                        if value > 31 {
                            panic!("Field ID must be between 0 and 31");
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

use std::collections::HashSet;

fn generate_non_defined_ids(v: Vec<Option<u8>>) -> Option<Vec<u8>> {
    let mut used = HashSet::new();

    // recoger los usados
    for x in v.iter().flatten() {
        used.insert(*x);
    }

    let mut result = Vec::with_capacity(v.len());
    let mut next_id: u8 = 0;

    for slot in v {
        match slot {
            Some(x) => {
                used.insert(x);
                result.push(x);
            }
            None => {
                while next_id <= 31 && used.contains(&next_id) {
                    if next_id == 31 {
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

#[proc_macro_derive(PrietoBuffersSerde, attributes(field_id))]
pub fn derive_prieto_buffer_serde(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let struct_name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("PrietoBuffersSerde can only be derived for structs"),
    };

    let field_names: Vec<_>= fields.iter().map(|field| {
        field.ident.as_ref().expect("Fields must be named")
    }).collect();

    let defined_field_ids = fields.iter().map(|field| {
        for attr in &field.attrs {
            if attr.path().is_ident("field_id") {
                return parse_u8_from_attr(attr);
            }
        }

        return None
    }).collect::<Vec<_>>();


    let field_ids = generate_non_defined_ids(defined_field_ids).expect("Too many fields, not enough IDs available");

    quote! {
        impl #struct_name {
            pub fn skip_field(bytes: &[u8], field_type: prieto_buffers::FieldType) -> u32 {
                if let prieto_buffers::FieldType::Struct = field_type {
                    let field_count = bytes[0] as u32;
                    let mut offset = 1;

                    for _ in 0..field_count {
                        let field_header = bytes[offset as usize];
                        offset += 1;

                        let field_type = prieto_buffers::FieldType::from_u8(field_header >> 5).unwrap();
                        offset += #struct_name::skip_field(&bytes[offset as usize..], field_type);
                    }

                    offset as u32
                }
                else {
                    field_type.get_size() as u32
                }
            }
        }

        impl PrietoBuffersSerde for #struct_name {
            fn get_size(&self) -> u32 {
                let mut size = 1;
                #(if self.#field_names.should_serialize() {
                    size += self.#field_names.get_size() + 1;
                })*
                size
            }

            fn get_type(&self) -> prieto_buffers::FieldType {
                prieto_buffers::FieldType::Struct
            }

            fn serialize(&self, bytes: &mut [u8]) {
                let mut offset:u32 = 0;
                let mut field_amount:u8 = 0;
    
                #(if self.#field_names.should_serialize() {
                    field_amount += 1;
                })*
                bytes[offset as usize] = field_amount;
                offset += 1;

                #(
                    if self.#field_names.should_serialize() {
                        self.#field_names.serialize_with_header(#field_ids, &mut bytes[offset as usize..]);
                        offset += self.#field_names.get_size() + 1;
                    }
                )*
            }

            fn deserialize(&mut self, bytes: &[u8]) {
                let mut offset:u32 = 0;
                let mut counter:u8 = 0;
                
                let field_count = bytes[offset as usize];
                offset += 1;

                for _ in 0..field_count {
                    let field_header = bytes[offset as usize];
                    offset += 1;

                    let field_id = field_header & 0b00011111;
                    let field_type = prieto_buffers::FieldType::from_u8(field_header >> 5).unwrap();

                    let field_size = match field_id {
                        #(
                            #field_ids => {
                                if self.#field_names.get_type() == field_type {
                                    self.#field_names.deserialize(&bytes[offset as usize..]);
                                    self.#field_names.get_size()
                                }
                                else {
                                    #struct_name::skip_field(&bytes[offset as usize..], field_type)
                                }
                            }
                        )*
                        _ => {
                            #struct_name::skip_field(&bytes[offset as usize..], field_type)
                        }
                    };

                    offset += field_size;
                }
            }
        }
    }.into()
}