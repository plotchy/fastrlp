use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_encodable(ast: &syn::DeriveInput) -> TokenStream {
    let body = if let syn::Data::Struct(s) = &ast.data {
        s
    } else {
        panic!("#[derive(RlpEncodable)] is only defined for structs.");
    };

    let length_stmts: Vec<_> = body
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| encodable_length(i, field))
        .collect();

    let stmts: Vec<_> = body
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| encodable_field(i, field))
        .collect();
    let name = &ast.ident;

    let impl_block = quote! {
        trait E {
            fn rlp_header(&self) -> fastrlp::Header;
        }

        impl E for #name {
            fn rlp_header(&self) -> fastrlp::Header {
                let mut rlp_head = fastrlp::Header { list: true, payload_length: 0 };
                #(#length_stmts)*
                rlp_head
            }
        }

        impl fastrlp::Encodable for #name {
            fn length(&self) -> usize {
                let rlp_head = E::rlp_header(self);
                return fastrlp::length_of_length(rlp_head.payload_length) + rlp_head.payload_length;
            }
            fn encode(&self, out: &mut dyn bytes::BufMut) {
                E::rlp_header(self).encode(out);
                #(#stmts)*
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

pub fn impl_max_encoded_len(ast: &syn::DeriveInput) -> TokenStream {
    let body = if let syn::Data::Struct(s) = &ast.data {
        s
    } else {
        panic!("#[derive(RlpEncodable)] is only defined for structs.");
    };

    let stmts: Vec<_> = body
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| encodable_max_length(index, field))
        .collect();
    let name = &ast.ident;

    let impl_block = quote! {
        unsafe impl fastrlp::MaxEncodedLen<{ fastrlp::const_add(fastrlp::length_of_length(#(#stmts)*), #(#stmts)*) }> for #name {}
        unsafe impl fastrlp::MaxEncodedLenAssoc for #name {
            const LEN: usize = { fastrlp::const_add(fastrlp::length_of_length(#(#stmts)*), { #(#stmts)* }) };
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

fn encodable_length(index: usize, field: &syn::Field) -> TokenStream {
    let ident = if let Some(ident) = &field.ident {
        quote! { #ident }
    } else {
        let index = syn::Index::from(index);
        quote! { #index }
    };

    quote! { rlp_head.payload_length += fastrlp::Encodable::length(&self.#ident); }
}

fn encodable_max_length(index: usize, field: &syn::Field) -> TokenStream {
    let fieldtype = &field.ty;

    if index == 0 {
        quote! { <#fieldtype as fastrlp::MaxEncodedLenAssoc>::LEN }
    } else {
        quote! { + <#fieldtype as fastrlp::MaxEncodedLenAssoc>::LEN }
    }
}

fn encodable_field(index: usize, field: &syn::Field) -> TokenStream {
    let ident = if let Some(ident) = &field.ident {
        quote! { #ident }
    } else {
        let index = syn::Index::from(index);
        quote! { #index }
    };

    let id = quote! { self.#ident };

    quote! { fastrlp::Encodable::encode(&#id, out); }
}
