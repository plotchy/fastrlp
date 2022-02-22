use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_decodable(ast: &syn::DeriveInput) -> TokenStream {
    let body = if let syn::Data::Struct(s) = &ast.data {
        s
    } else {
        panic!("#[derive(RlpDecodable)] is only defined for structs.");
    };

    let stmts: Vec<_> = body
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| decodable_field(i, field))
        .collect();
    let name = &ast.ident;

    let impl_block = quote! {
        impl fastrlp::Decodable for #name {
            fn decode(buf: &mut &[u8]) -> Result<Self, fastrlp::DecodeError> {
                let rlp_head = fastrlp::Header::decode(buf)?;

                if !rlp_head.list {
                    return Err(fastrlp::DecodeError::UnexpectedString);
                }

                let started_len = buf.len();
                let this = Self {
                    #(#stmts)*
                };

                let consumed = started_len - buf.len();
                if consumed != rlp_head.payload_length {
                    return Err(fastrlp::DecodeError::ListLengthMismatch {
                        expected: rlp_head.payload_length,
                        got: consumed,
                    });
                }

                Ok(this)
            }
        }
    };

    quote! {
        const _: () = {
            extern crate bytes;
            extern crate fastrlp;
            #impl_block
        };
    }
}

fn decodable_field(index: usize, field: &syn::Field) -> TokenStream {
    let id = if let Some(ident) = &field.ident {
        quote! { #ident }
    } else {
        let index = syn::Index::from(index);
        quote! { #index }
    };

    quote! { #id: fastrlp::Decodable::decode(buf)?, }
}
