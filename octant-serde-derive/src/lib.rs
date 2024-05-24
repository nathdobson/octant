extern crate proc_macro;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Type};

#[proc_macro_derive(DeserializeWith)]
pub fn derive_deserialize_with(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output: TokenStream;
    let DeriveInput {
        attrs: _,
        vis: _,
        ident: input_ident,
        generics: _,
        data: input_data,
        ..
    } = input;
    match &input_data {
        Data::Struct(strukt) => {
            let DataStruct {
                struct_token: _,
                fields: struct_fields,
                semi_token: _,
                ..
            } = strukt;
            let field_names: Vec<&Ident>;
            let field_types: Vec<&Type>;
            match struct_fields {
                Fields::Named(struct_fields) => {
                    field_names = struct_fields
                        .named
                        .iter()
                        .map(|x| x.ident.as_ref().unwrap())
                        .collect();
                    field_types = struct_fields.named.iter().map(|x| &x.ty).collect();
                }
                Fields::Unnamed(_) => todo!("tuple struct"),
                Fields::Unit => todo!("unit struct"),
            };
            output = quote! {
                #[automatically_derived]
                impl<'de> ::octant_serde::DeserializeWith<'de> for #input_ident {
                    fn deserialize_with<
                        D: ::octant_serde::reexports::serde::Deserializer<'de>
                    >(
                        ctx: &::octant_serde::DeserializeContext,
                        d:D
                    ) -> ::std::result::Result<
                        Self,
                        <D as ::octant_serde::reexports::serde::Deserializer<'de>>::Error>
                    {
                        #[allow(non_camel_case_types)]
                        #[derive(::octant_serde::reexports::serde::Deserialize)]
                        enum Field{
                            #( #field_names ),*
                        }
                        struct V<'c>{ctx:&'c ::octant_serde::DeserializeContext}
                        impl<'c,'de> ::octant_serde::reexports::serde::de::Visitor<'de> for V<'c>{
                            type Value = #input_ident;
                            fn expecting(&self, f:&mut ::std::fmt::Formatter)->::std::fmt::Result{
                                write!(f,::std::stringify!(#input_ident))
                            }
                            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::octant_serde::reexports::serde::de::MapAccess<'de> {
                                #( let mut #field_names: Option<#field_types> = None; )*
                                while let Some(field) = map.next_key::<Field>()? {
                                    match field {
                                        #(
                                            Field::#field_names => {
                                                #field_names = Some(map.next_value_seed(::octant_serde::DeserializeWithSeed::<#field_types>::new(self.ctx))?);
                                            }
                                        )*
                                    }
                                }
                                #(
                                    let #field_names = #field_names.ok_or_else(||
                                        <A::Error as ::octant_serde::reexports::serde::de::Error>::custom(format_args!("Missing field {}",std::stringify!(#field_names)))
                                    )?;
                                )*
                                Ok(#input_ident {#(#field_names,)*})
                            }
                        }
                        d.deserialize_struct(::std::stringify!(#input_ident),&[#(::std::stringify!(#field_names)),*],V{ctx})
                    }
                }
            }
        }
        Data::Enum(_) => todo!("enum"),
        Data::Union(_) => todo!("union"),
    }
    proc_macro::TokenStream::from(output)
}
