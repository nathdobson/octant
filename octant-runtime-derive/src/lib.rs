#![allow(unused_variables)]

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

#[proc_macro_derive(PeerNewClient)]
pub fn derive_peer_new_client(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(
        derive_peer_new_client_impl(input).unwrap_or_else(syn::Error::into_compile_error),
    )
}

fn derive_peer_new_client_impl(input: DeriveInput) -> syn::Result<TokenStream> {
    let output: TokenStream;
    let DeriveInput {
        attrs: input_attrs,
        vis: input_vis,
        ident: input_ident,
        generics: input_generics,
        data: input_data,
    } = &input;
    match input_data {
        Data::Struct(strukt) => {
            let DataStruct {
                struct_token,
                fields,
                semi_token,
            } = strukt;
            let parent_field = fields
                .iter()
                .nth(0)
                .expect("first field")
                .ident
                .as_ref()
                .unwrap();
            let parent_type = &fields.iter().nth(0).expect("first field").ty;
            let native_field = fields
                .iter()
                .nth(1)
                .expect("second field")
                .ident
                .as_ref()
                .unwrap();
            let native_type = &fields.iter().nth(1).expect("second field").ty;
            let field_names: Vec<_> = fields.iter().skip(2).map(|x| &x.ident).collect();
            output = quote! {
                impl ::octant_runtime_client::PeerNew for #input_ident {
                    type Builder = #native_type;
                    fn peer_new(native: #native_type) -> Self {
                        #input_ident {
                            #parent_field: <#parent_type as ::octant_runtime_client::PeerNew>::peer_new(::std::clone::Clone::clone(&native).into()),
                            #native_field: native,
                            #( #field_names : ::std::default::Default::default()),*
                        }
                    }
                }
                impl ::octant_runtime_client::peer::AsNative for <#input_ident as ::octant_object::class::ClassValue>::Dyn {
                    type Native = #native_type;
                    fn native(&self)->&#native_type{
                        &self.#native_field
                    }
                }
            };
        }
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
    Ok(output)
}

#[proc_macro_derive(PeerNewServer)]
pub fn derive_peer_new_server(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(
        crate::derive_peer_new_server_impl(input).unwrap_or_else(syn::Error::into_compile_error),
    )
}

fn derive_peer_new_server_impl(input: DeriveInput) -> syn::Result<TokenStream> {
    let output: TokenStream;
    let DeriveInput {
        attrs: input_attrs,
        vis: input_vis,
        ident: input_ident,
        generics: input_generics,
        data: input_data,
    } = &input;
    match input_data {
        Data::Struct(strukt) => {
            let DataStruct {
                struct_token,
                fields,
                semi_token,
            } = strukt;
            let parent_field = fields
                .iter()
                .nth(0)
                .expect("first field")
                .ident
                .as_ref()
                .unwrap();
            let parent_type = &fields.iter().nth(0).expect("first field").ty;
            let field_names: Vec<_> = fields.iter().skip(1).map(|x| &x.ident).collect();
            output = quote! {
                impl ::octant_runtime_server::PeerNew for #input_ident {
                    type Builder = ::octant_runtime::peer::PeerValue;
                    fn peer_new(peer: ::octant_runtime::peer::PeerValue) -> Self {
                        #input_ident {
                            #parent_field: <#parent_type as ::octant_runtime_server::PeerNew>::peer_new(peer),
                            #( #field_names : ::std::default::Default::default()),*
                        }
                    }
                }
            };
        }
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
    Ok(output)
}

#[proc_macro_derive(SerializePeer)]
pub fn derive_serialize_peer(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(
        crate::derive_serialize_peer_impl(input).unwrap_or_else(syn::Error::into_compile_error),
    )
}

fn derive_serialize_peer_impl(input: DeriveInput) -> syn::Result<TokenStream> {
    let output: TokenStream;
    let DeriveInput {
        attrs: input_attrs,
        vis: input_vis,
        ident: input_ident,
        generics: input_generics,
        data: input_data,
    } = &input;
    let value = input_ident;
    output = quote! {
        impl octant_runtime::reexports::serde::Serialize for <#value as ::octant_object::class::ClassValue>::Dyn {
            fn serialize<S>(&self, s: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                return self.raw_handle().serialize(s);
            }
        }
    };
    Ok(output)
}

#[proc_macro_derive(DeserializePeer)]
pub fn derive_deserialize_peer(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(
        crate::derive_deserialize_peer_impl(input).unwrap_or_else(syn::Error::into_compile_error),
    )
}

fn derive_deserialize_peer_impl(input: DeriveInput) -> syn::Result<TokenStream> {
    let output: TokenStream;
    let DeriveInput {
        attrs: input_attrs,
        vis: input_vis,
        ident: input_ident,
        generics: input_generics,
        data: input_data,
    } = &input;
    let value = input_ident;
    output = quote! {
        impl<'de> ::octant_serde::DeserializeRcWith<'de> for <#value as ::octant_object::class::ClassValue>::Dyn {
            fn deserialize_rc_with<
                D: ::serde::Deserializer<'de>
            >(
                ctx: &::octant_serde::DeserializeContext,
                d: D
            ) -> ::std::result::Result<::octant_reffed::rc::Rc2<Self>, D::Error>{
                octant_runtime::deserialize_object_with(ctx, d)
            }
        }
    };
    Ok(output)
}
