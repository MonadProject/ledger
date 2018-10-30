extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};



//see https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs
#[proc_macro_derive(Monad_Serializable)]
pub fn monad_serialize(input: TokenStream) -> TokenStream {

    // Parse the input tokens into a syntax tree
    //
//    let input = parse_macro_input!(input as DeriveInput);
    unimplemented!()

}

/// Data structure sent to a `proc_macro_derive` macro.
/// *This type is available if Syn is built with the `"derive"` feature.*
fn serialize(input: &DeriveInput) {
    let data = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => panic!("not struct")
    };

    //todo
}

/// consider struct and tuple struct(Fields of tuple structs have no names.)
fn serialize_field(index: usize,field: &syn::Field) {
    let indent = match field.ident {
        Some(ref ident) => ident.to_string(),
        None => index.to_string()
    };
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
