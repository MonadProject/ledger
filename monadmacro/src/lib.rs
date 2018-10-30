//see https://github.com/rust-lang/book/blob/master/first-edition/src/procedural-macros.md

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use syn::VariantData;


#[proc_macro_derive(Monad_Serializable)]
pub fn monad_serialize(input: TokenStream) -> TokenStream {

    // Construct a string representation of the type definition
//    let s = input.to_string();

    // Parse the string representation
//    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
//    let gen = impl_serialize(&ast);

    unimplemented!()
}

/// Data structure sent to a `proc_macro_derive` macro.
/// *This type is available if Syn is built with the `"derive"` feature.*
fn impl_serialize(ast: &syn::DeriveInput) -> quote::Tokens {
//    let body: &VariantData = match ast.body {
//        syn::Body::Struct(ref s) => s,
//        _ => println!("not struct"),
//    };
//
//
//    let a = *body;
//
//    let bbb = match *body {
//        syn::VariantData::Struct(ref fields) => fields.iter().enumerate().map().collect(),
//    };
    unimplemented!()
}

fn serialize_field_map(pairs: (usize, &syn::Field)) -> quote::Tokens {
    serialize_field(pairs.0, pairs.1)
}

fn serialize_field(index: usize, field: &syn::Field) -> quote::Tokens {
    let ident = match field.ident {
        Some(ref ident) => ident.to_string(),
        None => index.to_string()
    };

    let id = syn::Ident::new(format!("self.{}", ident));

    if "Vec" == &ident.to_string() {
        quote! {
            stream.write_list(&#id);
        }
    } else {
        quote! {
            stream.write(&#id);
        }
    }
}


#[proc_macro_derive(Monad_Deserializable)]
pub fn monad_deserialize(input: TokenStream) -> TokenStream {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
