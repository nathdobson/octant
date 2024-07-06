#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

extern crate proc_macro;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DataStruct, DeriveInput, FnArg, GenericParam
    , ItemTrait, parse_macro_input, Signature,
    spanned::Spanned, TraitItem, TraitItemFn, TypeParamBound,
};

struct Args {}

#[proc_macro_attribute]
pub fn class(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parser =
        syn::meta::parser(|meta| Err(syn::Error::new(meta.path.span(), "No parameters expected")));
    parse_macro_input!(args with parser);
    let args = Args {};
    let input = parse_macro_input!(input as ItemTrait);
    proc_macro::TokenStream::from(
        derive_class_impl(args, &input).unwrap_or_else(syn::Error::into_compile_error),
    )
}

fn derive_class_impl(args: Args, input: &ItemTrait) -> syn::Result<TokenStream> {
    let output: TokenStream;
    let ItemTrait {
        attrs,
        vis,
        unsafety,
        auto_token,
        restriction,
        trait_token,
        ident,
        generics,
        colon_token,
        supertraits,
        brace_token,
        items,
    } = input;

    let parent = match supertraits.iter().next().unwrap() {
        TypeParamBound::Trait(x) => &x.path,
        _ => todo!(),
    };

    let class = ident;
    let rc_class = format_ident!("Rc{}", ident);
    let fields = format_ident!("{}Fields", class);

    let get_ref = format_ident!(
        "{}",
        class
            .to_string()
            .from_case(Case::Pascal)
            .to_case(Case::Snake),
        span = class.span()
    );
    let get_mut = format_ident!("{}_mut", get_ref);

    let mut signatures: Vec<TokenStream> = vec![];
    let mut stubs: Vec<TokenStream> = vec![];
    let mut impls: Vec<TokenStream> = vec![];
    for item in items {
        match item {
            TraitItem::Fn(f) => {
                let TraitItemFn {
                    attrs,
                    sig,
                    default,
                    semi_token,
                } = f;
                let stub_name = format_ident!("_stub_{}", sig.ident);
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
                let mut parameters: Vec<TokenStream> = vec![];
                let mut args: Vec<TokenStream> = vec![];
                for (i, input) in inputs.iter().enumerate() {
                    match input {
                        FnArg::Receiver(receiver) => {
                            parameters.push(quote! {#input});
                            let s = receiver.self_token;
                            args.push(quote! {#s});
                        }
                        FnArg::Typed(typed) => {
                            let ident = format_ident!("_param_{}", i, span = typed.span());
                            let colon = &typed.colon_token;
                            let ty = &typed.ty;
                            parameters.push(quote! { #ident #colon #ty});
                            args.push(quote! {#ident});
                        }
                    }
                }
                signatures.push(quote! {
                    #(#attrs)*
                    #fn_token #ident #generics (#(#parameters),*) #output;
                });
                stubs.push(quote! {
                    #(#attrs)*
                    #fn_token #ident #generics (#(#parameters),*) #output {
                        <dyn #class>::#stub_name(#(#args),*)
                    }
                });
                impls.push(quote! {
                    #(#attrs)*
                    #fn_token #stub_name #generics ( #inputs ) #output #default
                })
            }
            _ => todo!(),
        }
    }

    let generic_params = &generics.params;
    let generic_where = &generics.where_clause;

    let generic_args = generics
        .params
        .iter()
        .map(|x| match x {
            GenericParam::Lifetime(x) => todo!(),
            GenericParam::Type(x) => &x.ident,
            GenericParam::Const(_) => todo!(),
        })
        .collect::<Vec<_>>();
    let output = quote! {
        #(#attrs)*
        #vis #unsafety #auto_token #trait_token #ident <#generic_params> #colon_token #supertraits {
            fn #get_ref(&self) -> &#fields <#(#generic_args),*>;
            fn #get_mut(&mut self) -> &mut #fields <#(#generic_args),*>;
            #(#signatures)*
        }
        pub type #rc_class <#(#generic_args),*> = ::octant_object::reexports::marshal_pointer::Rcf<dyn 'static + #class<#(#generic_args),*>>;
        impl<__super_secret__T, #generic_params> #unsafety #class<#(#generic_args),*> for __super_secret__T where
            __super_secret__T: #supertraits,
            __super_secret__T: octant_object::class::Ranked,
            __super_secret__T: octant_object::class::DerefRanked<
                __super_secret__T::Rank,
                <#fields<#(#generic_args),*> as octant_object::class::Ranked>::Rank, TargetRanked = #fields<#(#generic_args),*>>,
        {
            fn #get_ref(&self) -> &#fields<#(#generic_args),*> {
                self.deref_ranked()
            }
            fn #get_mut(&mut self) -> &mut #fields<#(#generic_args),*>{
                self.deref_mut_ranked()
            }
            #(#stubs)*
        }
        impl<#generic_params> dyn #class<#(#generic_args),*> {
            #(#impls)*
        }
        impl<#generic_params> ::std::ops::Deref for dyn #class<#(#generic_args),*> {
            type Target = #fields<#(#generic_args),*>;
            fn deref(&self) -> &Self::Target {
                self.#get_ref()
            }
        }
        impl<#generic_params> ::std::ops::DerefMut for dyn #class<#(#generic_args),*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.#get_mut()
            }
        }
        impl<#generic_params> ::std::ops::Deref for #fields<#(#generic_args),*> {
            type Target = <dyn #parent as ::octant_object::class::Class>::Fields;
            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }
        impl<#generic_params> ::std::ops::DerefMut for #fields<#(#generic_args),*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.parent
            }
        }
        impl<#generic_params> ::octant_object::class::Class for dyn #class<#(#generic_args),*> {
            type Fields = #fields<#(#generic_args),*>;
        }
        impl<#generic_params> ::octant_object::class::ClassValue for #fields<#(#generic_args),*> {
            type Dyn = dyn #class<#(#generic_args),*>;
        }
        impl<#generic_params> octant_object::class::Subclass for dyn #class<#(#generic_args),*> {
            type Parent = dyn #parent;
        }
        impl<#generic_params> octant_object::class::Ranked for #fields<#(#generic_args),*> {
            type Rank = ::octant_object::class::Succ<<<dyn #parent as ::octant_object::class::Class>::Fields as octant_object::class::Ranked>::Rank>;
        }
    };
    Ok(output)
}

/// Derive [Debug] for a class with fields from each class flattened into one set of braces.
#[proc_macro_derive(DebugClass)]
pub fn derive_debug_class(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(
        derive_debug_class_impl(input).unwrap_or_else(syn::Error::into_compile_error),
    )
}

fn derive_debug_class_impl(input: DeriveInput) -> syn::Result<TokenStream> {
    let output: TokenStream;
    let DeriveInput {
        attrs: input_attrs,
        vis: input_vis,
        ident: input_ident,
        generics: input_generics,
        data: input_data,
    } = &input;

    let class = format_ident!(
        "{}",
        input_ident.to_string().strip_suffix("Fields").unwrap(),
        span = input_ident.span()
    );

    let generic_params = &input_generics.params;
    let generic_where = &input_generics.where_clause;

    let generic_args = input_generics
        .params
        .iter()
        .map(|x| match x {
            GenericParam::Lifetime(x) => todo!(),
            GenericParam::Type(x) => &x.ident,
            GenericParam::Const(_) => todo!(),
        })
        .collect::<Vec<_>>();

    match input_data {
        Data::Struct(strukt) => {
            let DataStruct {
                struct_token,
                fields,
                semi_token,
            } = strukt;
            let field_names: Vec<_> = fields.iter().skip(1).map(|x| &x.ident).collect();
            output = quote! {
                impl<#generic_params> ::octant_object::class::DebugClass for #input_ident<#(#generic_args),*> {
                    fn fmt_class(&self, f: &mut ::std::fmt::DebugStruct) {
                        ::octant_object::class::DebugClass::fmt_class(&self.parent, f);
                        #(
                            f.field(std::stringify!(#field_names), &self.#field_names);
                        )*
                    }
                }
                impl<#generic_params> ::std::fmt::Debug for #input_ident<#(#generic_args),*> {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        let mut f = f.debug_struct(::std::stringify!(#class));
                        ::octant_object::class::DebugClass::fmt_class(self, &mut f);
                        f.finish()
                    }
                }

            };
        }
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
    Ok(output)
}
