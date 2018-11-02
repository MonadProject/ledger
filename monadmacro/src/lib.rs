//see https://github.com/rust-lang/book/blob/master/first-edition/src/procedural-macros.md

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate serialization;
extern crate syn;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use syn::VariantData;


#[proc_macro_derive(Monad_Serializable)]
pub fn monad_serialize(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    impl_serialize(&ast)
}


fn impl_serialize(ast: &syn::DeriveInput) -> TokenStream {
    //1.首先确保打这个注解的是struct
    let data = match ast.body {
        syn::Body::Struct(ref data) => data,
        _ => panic!("only support for structs"),
    };

    //2.不支持union 类型
    let statements: Vec<_> = match *data {
        syn::VariantData::Struct(ref fields) => fields.iter().enumerate().map(serialize_field_map).collect(),
        syn::VariantData::Tuple(ref fields) => fields.iter().enumerate().map(serialize_field_map).collect(),
        _ => panic!("no union")
    };

    let size_statements: Vec<_> = match *data {
        syn::VariantData::Struct(ref fields) => fields.iter().enumerate().map(serialize_size_map).collect(),
        syn::VariantData::Tuple(ref fields) => fields.iter().enumerate().map(serialize_size_map).collect(),
        _ => panic!("no union")
    };

    let identity = &ast.ident;

    let dummy = syn::Ident::new(format!("impl_serialization_for{}", identity));

    //see https://docs.rs/quote/0.6.9/quote/macro.quote.html
    let tokens = quote! {
		#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
		const #dummy: () = {
			extern crate serialization;
			impl serialization::stream::Serializable for #identity {
			    fn serialize(&self, stream: &mut serialization::stream::Stream) {
				    #(#statements)*
			    }

			    fn serialized_size(&self) -> usize {
                    #(#size_statements)+*
			    }

            }
		};
    };
    tokens.parse().unwrap()
}

fn serialize_field_map(pair: (usize, &syn::Field)) -> quote::Tokens {
    serialize_field(pair.0, pair.1)
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

fn serialize_size_map(pair: (usize, &syn::Field)) -> quote::Tokens {
    serialize_size(pair.0, pair.1)
}

fn serialize_size(index: usize, field: &syn::Field) -> quote::Tokens {
    let ident = match field.ident {
        Some(ref ident) => ident.to_string(),
        None => index.to_string()
    };

    let id = syn::Ident::new(format!("self.{}", ident));

    if "Vec" == &ident.to_string() {
        quote! {
             serialization::stream::serialize_list_size(&#id)
        }
    } else {
        quote! {
            #id.serialized_size()
        }
    }
}


#[proc_macro_derive(Monad_Deserializable)]
pub fn monad_deserialize(input: TokenStream) -> TokenStream {
    unimplemented!()
}


pub fn deserialize_field_map(pair: (usize, &syn::Field)) -> quote::Tokens {
    deserialize_field(pair.0, pair.1)
}


pub fn deserialize_field(index: usize, field: &syn::Field) -> quote::Tokens {
    let ident = match field.ident {
        Some(ref ident) => ident.to_string(),
        None => index.to_string()
    };

    let id = syn::Ident::new(ident.to_string());

    if "Vec" == &ident.to_string() {
        quote! {
            #id: reader.read_list()?,
        }
    } else {
        quote! {
            #id: reader.read()?,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
