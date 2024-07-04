#![allow(unused_variables)]

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, FnArg, ImplItem,
    ImplItemFn, Item, ItemFn, ItemImpl, Pat, ReturnType, Signature, Token, Type,
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
    let class = format_ident!(
        "{}",
        input_ident.to_string().strip_suffix("Fields").unwrap(),
        span = input_ident.span()
    );
    let encoder_trait = quote!(octant_runtime::reexports::marshal::encode::Encoder);
    let serialize_rc_trait = quote!(octant_runtime::reexports::marshal::ser::rc::SerializeRc);
    let any_encoder = quote!(octant_runtime::reexports::marshal::encode::AnyEncoder);
    let context = quote!(octant_runtime::reexports::marshal::context::Context);
    let anyhow = quote!(octant_runtime::reexports::anyhow);
    let rc_ref = quote!(octant_runtime::reexports::marshal_pointer::rc_ref::RcRef);
    let serialize_trait = quote!(octant_runtime::reexports::marshal::ser::Serialize);
    let raw_handle = quote!(octant_runtime::handle::RawHandle);
    output = quote! {
        impl <E:#encoder_trait> #serialize_rc_trait<E> for dyn #class {
            fn serialize_rc<'w,'en>(this: &#rc_ref<Self>, e:#any_encoder<'w,'en,E>, ctx: #context)->#anyhow::Result<()>{
                <#raw_handle as #serialize_trait<E>>::serialize(&(**this).raw_handle(),e,ctx)
                // (**this).serialize
                // todo!("derive_serialize_peer_impl");
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
    let class = format_ident!(
        "{}",
        input_ident.to_string().strip_suffix("Fields").unwrap(),
        span = input_ident.span()
    );
    let decoder = quote!(::octant_runtime::reexports::marshal::decode::Decoder);
    let any_decoder = quote!(::octant_runtime::reexports::marshal::decode::AnyDecoder);
    let context = quote!(::octant_runtime::reexports::marshal::context::Context);
    let anyhow = quote!(::octant_runtime::reexports::anyhow);
    let rc = quote!(::std::rc::Rc);
    output = quote! {
        impl<D:#decoder> ::octant_runtime::reexports::marshal::de::rc::DeserializeRc<D> for dyn #class {
            fn deserialize_rc<'p,'de>(d:#any_decoder<'p,'de,D>, ctx: #context)->#anyhow::Result<#rc<Self>>{
                ::octant_runtime::deserialize_peer::<D,dyn #class>(d,ctx)
            }
        }
    };
    Ok(output)
}

struct RpcArgs {
    self_type: Option<Type>,
}

#[proc_macro_attribute]
pub fn rpc(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut self_type = None;
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("self") {
            meta.input.parse::<Token![=]>()?;
            self_type = Some(meta.input.parse()?);
            Ok(())
        } else {
            Err(syn::Error::new(meta.path.span(), "No parameters expected"))
        }
    });
    parse_macro_input!(args with parser);
    let args = RpcArgs { self_type };
    let input = parse_macro_input!(input as Item);
    proc_macro::TokenStream::from(
        rpc_item(&args, &input).unwrap_or_else(syn::Error::into_compile_error),
    )
}

fn rpc_item(args: &RpcArgs, input: &Item) -> syn::Result<TokenStream> {
    match input {
        Item::Fn(f) => rpc_fn(args, f),
        Item::Impl(i) => rpc_impl(args, i),
        _ => todo!(),
    }
}

fn rpc_fn(args: &RpcArgs, input: &ItemFn) -> syn::Result<TokenStream> {
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
    let mut seen_runtime = false;
    let mut server_runtime_param = vec![];
    let mut self_param = vec![];
    let mut runtime_lookup = vec![];
    let mut this_capture = vec![];
    let mut this_field = vec![];
    let mut self_callee = vec![];
    for (i, input) in inputs.iter().enumerate() {
        match input {
            FnArg::Receiver(rec) => {
                self_param.push(quote! { #rec });
            }
            FnArg::Typed(pat_type) => {
                let colon = &pat_type.colon_token;
                let ty = Some(&pat_type.ty);
                if !seen_runtime {
                    seen_runtime = true;
                    let ident = Some(format_ident!("runtime", span = pat_type.pat.span()));
                    if self_param.is_empty() {
                        server_runtime_param.push(quote! {#ident #colon #ty });
                    } else {
                        let self_type = args.self_type.as_ref().ok_or_else(|| {
                            syn::Error::new(
                                pat_type.span(),
                                "Must specify [rpc(self=TheCurrentClass)] for methods",
                            )
                        })?;
                        runtime_lookup.push(quote! {
                            let runtime = self.runtime();
                        });
                        this_capture.push(quote! {
                            this: ::octant_runtime::reexports::marshal_pointer::rcf::Rcf::from(self.rc())
                        });
                        this_field.push(quote! {
                            this: ::octant_runtime::reexports::marshal_pointer::rcf::Rcf<#self_type>
                        });
                        self_callee.push(quote! {
                            //<::std::rc::Rc<_> as ::octant_runtime::reexports::marshal_pointer::AsFlatRef>::as_flat_ref(&self.this).
                            self.this.
                        });
                    }
                } else {
                    let param_name;
                    match &*pat_type.pat {
                        Pat::Ident(ident) => {
                            param_name = format_ident!("_param_{}", ident.ident);
                        }
                        _ => {
                            param_name = format_ident!("_param_{}", i, span = pat_type.pat.span());
                        }
                    }
                    server_params.push(quote! { #param_name #colon #ty });
                    param_names.push(quote! {#param_name});
                }
            }
        };
    }
    let output_type;
    let output_type_arrow;
    match output {
        ReturnType::Default => {
            output_type = quote! { () };
            output_type_arrow = quote! { -> };
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
            .to_case(Case::Pascal),
        span = ident.span()
    );
    let request_type_def = quote! {
        #[derive(::std::fmt::Debug, ::octant_runtime::reexports::marshal::Serialize, ::octant_runtime::reexports::marshal::Deserialize)]
        struct #request_type {
            #(#this_field,)*
            #(#server_params,)*
            down: <#output_type as ::octant_runtime::immediate_return::ImmediateReturn>::Down
        }
        ::octant_runtime::reexports::marshal_object::derive_variant!(::octant_runtime::proto::BoxDownMessage, #request_type);
    };
    output_tokens = quote! {
        #[cfg(side = "server")]
        #vis #fn_token #ident(
            #(#self_param,)*
            #(#server_runtime_param,)*
            #(#server_params),*
        ) #output {
            #request_type_def
            impl ::octant_runtime::proto::DownMessage for #request_type {}

            #(#runtime_lookup)*
            let (output, down) = <#output_type as ::octant_runtime_server::immediate_return::ImmediateReturn>::immediate_new(runtime);
            runtime.send(Box::<#request_type>::new(#request_type {
                #(#this_capture,)*
                #(#param_names,)*
                down
            }));
            output
        }


        #[cfg(side="client")]
        #fn_token #ident(
            #inputs
        ) #output_type_arrow ::octant_runtime::reexports::octant_error::OctantResult<#output_type> {
            #request_type_def
            impl ::octant_runtime::proto::DownMessage for #request_type {
                fn run(self:Box<Self>, runtime: &::std::rc::Rc<::octant_runtime::runtime::Runtime>) -> ::octant_runtime::reexports::octant_error::OctantResult<()>{
                    let output = #(#self_callee)*#ident(runtime #(, self.#param_names)*)?;
                    ::octant_runtime::immediate_return::ImmediateReturn::immediate_return(output, runtime, self.down);
                    Ok(())
                }
            }
            #block
        }
    };
    Ok(output_tokens)
}

fn rpc_impl(args: &RpcArgs, input: &ItemImpl) -> syn::Result<TokenStream> {
    let ItemImpl {
        attrs,
        defaultness,
        unsafety,
        impl_token,
        generics,
        trait_,
        self_ty,
        brace_token,
        items,
    } = input;
    let output;
    let mut out_items = vec![];
    for item in items {
        match item {
            ImplItem::Fn(item) => {
                let ImplItemFn {
                    attrs,
                    vis,
                    defaultness,
                    sig,
                    block,
                } = item;
                let mut out_attrs: Vec<TokenStream> = vec![];
                for attr in &item.attrs {
                    if attr.meta.path().is_ident("rpc") {
                        out_attrs.push(quote! {
                            #[rpc(self=#self_ty)]
                        });
                    } else {
                        out_attrs.push(quote! {
                            #attr
                        })
                    }
                }
                out_items.push(quote! {
                    #(#out_attrs)*
                    #vis
                    #defaultness
                    #sig
                    #block
                });
            }
            _ => {
                out_items.push(quote! {#item});
            }
        }
    }
    output = quote! {
        #(#attrs)*
        #defaultness
        #unsafety
        #impl_token
        #generics
        #self_ty
        {
            #(#out_items)*
        }
    };
    Ok(output)
}
