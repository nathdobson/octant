#![allow(unused_variables)]

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, FnArg, ItemFn, ReturnType,
    Signature,
};

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
                    type Builder = ::octant_runtime::peer::PeerFields;
                    fn peer_new(peer: ::octant_runtime::peer::PeerFields) -> Self {
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

struct RpcArgs {}

#[proc_macro_attribute]
pub fn rpc(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parser =
        syn::meta::parser(|meta| Err(syn::Error::new(meta.path.span(), "No parameters expected")));
    parse_macro_input!(args with parser);
    let args = RpcArgs {};
    let input = parse_macro_input!(input as ItemFn);
    proc_macro::TokenStream::from(
        rpc_impl(args, &input).unwrap_or_else(syn::Error::into_compile_error),
    )
}

fn rpc_impl(args: RpcArgs, input: &ItemFn) -> syn::Result<TokenStream> {
    let output_tokens: TokenStream;
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;
    let Signature {
        constness,
        asyncness,
        unsafety,
        abi,
        fn_token,
        ident,
        generics,
        paren_token,
        inputs,
        variadic,
        output,
    } = sig;
    let mut server_params: Vec<TokenStream> = vec![];
    let mut param_names: Vec<TokenStream> = vec![];
    let mut server_runtime_param = None;
    for (i, input) in inputs.iter().enumerate() {
        match input {
            FnArg::Receiver(_) => todo!(),
            FnArg::Typed(pat_type) => {
                let colon = &pat_type.colon_token;
                let ty = Some(&pat_type.ty);
                if server_runtime_param.is_none() {
                    let ident = Some(format_ident!("runtime", span = pat_type.pat.span()));
                    server_runtime_param = Some(quote! {#ident #colon #ty });
                } else {
                    let ident = format_ident!("_param_{}", i, span = pat_type.pat.span());
                    server_params.push(quote! { #ident #colon #ty });
                    param_names.push(quote! {#ident});
                }
            }
        };
    }
    let server_runtime_param = server_runtime_param.unwrap();
    let output_type;
    let output_type_arrow;
    match output {
        ReturnType::Default => {
            output_type = quote! { () };
            output_type_arrow = quote!{ -> };
        }
        ReturnType::Type(arrow, ty) => {
            output_type_arrow = quote! { #arrow };
            output_type = quote! { #ty };
        }
    };
    let request_type = format_ident!(
        "{}Request",
        format!("{}", ident)
            .from_case(Case::Snake)
            .to_case(Case::Pascal)
    );
    output_tokens = quote! {
        #[cfg(side = "server")]
        #vis #fn_token #ident(
            #server_runtime_param,
            #(#server_params),*
        ) #output {
            let (output, down) = <#output_type as ::octant_runtime_server::immediate_return::ImmediateReturn>::immediate_new(runtime);
            runtime.send(Box::<#request_type>::new(#request_type {
                #(#param_names,)*
                down
            }));
            output
        }

        #[derive(::std::fmt::Debug, ::octant_runtime::reexports::serde::Serialize, ::octant_runtime::reexports::octant_serde::DeserializeWith)]
        pub struct #request_type {
            #(#server_params,)*
            down: <#output_type as ::octant_runtime::immediate_return::ImmediateReturn>::Down
        }

        ::octant_runtime::reexports::octant_serde::define_serde_impl!(#request_type: ::octant_runtime::proto::DownMessage);

        impl ::octant_runtime::proto::DownMessage for #request_type {
            #[cfg(side="client")]
            fn run(self:Box<Self>, runtime: &::std::rc::Rc<::octant_runtime::runtime::Runtime>) -> ::octant_runtime::reexports::anyhow::Result<()>{
                let output=#ident(runtime #(, self.#param_names)*)?;
                ::octant_runtime::immediate_return::ImmediateReturn::immediate_return(output, runtime, self.down);
                Ok(())
            }
        }

        #[cfg(side="client")]
        #fn_token #ident(
            #inputs
        ) #output_type_arrow ::octant_runtime::reexports::anyhow::Result<#output_type> {
            #block
        }
    };
    Ok(output_tokens)
}
