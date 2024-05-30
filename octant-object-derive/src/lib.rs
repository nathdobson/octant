#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

extern crate proc_macro;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Field, Fields,
    GenericArgument, ItemStruct, Path, PathArguments, Token, Type, TypeParamBound,
};

#[proc_macro_attribute]
pub fn class(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    proc_macro::TokenStream::from(
        derive_class_impl(input).unwrap_or_else(syn::Error::into_compile_error),
    )
}
fn derive_class_impl(input: ItemStruct) -> syn::Result<TokenStream> {
    let output: TokenStream;
    let ItemStruct {
        attrs,
        vis,
        struct_token,
        ident: class,
        generics,
        fields,
        semi_token,
    } = &input;

    let parent: &Type;
    let field_defs: Vec<&Field>;
    let field_names: Vec<&Ident>;
    match fields {
        Fields::Named(struct_fields) => {
            let parent_token = &struct_fields.named[0].ident.as_ref().unwrap();
            if parent_token.to_string() != "parent" {
                return Err(syn::Error::new(
                    parent_token.span(),
                    "first field must be named `parent'",
                ));
            }
            parent = &struct_fields.named[0].ty;
            field_defs = struct_fields.named.iter().skip(1).collect();
            field_names = fields.iter().map(|x| x.ident.as_ref().unwrap()).collect();
        }
        Fields::Unnamed(_) => todo!("tuple struct"),
        Fields::Unit => todo!("unit struct"),
    };
    let parent = match parent {
        Type::TraitObject(x) => {
            if x.bounds.len() != 1 {
                return Err(syn::Error::new(
                    parent.span(),
                    "parent must be a trait object with one bound",
                ));
            }
            match &x.bounds[0] {
                TypeParamBound::Trait(x) => &x.path,
                _ => {
                    return Err(syn::Error::new(
                        parent.span(),
                        "parent must be a trait object with one bound for the parent class",
                    ));
                }
            }
        }
        _ => {
            return Err(syn::Error::new(
                parent.span(),
                "parent must be a trait object",
            ))
        }
    };
    // let mut class: Option<Path> = None;
    // let mut parent: Option<Path> = None;
    // for attr in &input_attrs {
    //     if attr.path().is_ident("octant") {
    //         attr.parse_nested_meta(|meta| {
    //             if meta.path.is_ident("class") {
    //                 meta.input.parse::<Token![=]>()?;
    //                 class = Some(meta.input.parse()?);
    //             } else if meta.path.is_ident("extends") {
    //                 meta.input.parse::<Token![=]>()?;
    //                 parent = Some(meta.input.parse()?);
    //             } else {
    //                 return Err(syn::Error::new(
    //                     meta.path.span(),
    //                     "expected `class' or `extends'",
    //                 ));
    //             }
    //
    //             Ok(())
    //         })?;
    //     }
    // }
    // let class = class.ok_or_else(|| {
    //     syn::Error::new(input_ident.span(), "missing #[octant(class=...)]")
    // })?;

    let rc_class = format_ident!("Rc{}", class);
    let as_class = format_ident!("As{}", class);
    let value = format_ident!("{}Value", class);
    let get_ref = format_ident!(
        "{}",
        class
            .to_string()
            .from_case(Case::Pascal)
            .to_case(Case::Snake),
        span = class.span()
    );
    let get_mut = format_ident!("{}_mut", get_ref);
    output = quote! {
        #[derive(::octant_object::DebugClass)]
        #( #attrs )*
        pub struct #value {
            parent: <dyn #parent as ::octant_object::class::Class>::Value,
            #(#field_defs),*
        }
        pub trait #as_class : #parent {
            fn #get_ref(&self) -> &#value;
            fn #get_mut(&mut self) -> &mut #value;
        }
        pub type #rc_class = ::octant_object::reexports::octant_reffed::rc::Rc2<dyn 'static + #class>;
        impl<__super_secret__T> #as_class for __super_secret__T where
            __super_secret__T: #parent,
            __super_secret__T: octant_object::class::Ranked,
            __super_secret__T: octant_object::class::DerefRanked<
                __super_secret__T::Rank,
                <#value as octant_object::class::Ranked>::Rank, TargetRanked = #value>,
        {
            fn #get_ref(&self) -> &#value {
                self.deref_ranked()
            }
            fn #get_mut(&mut self) -> &mut #value{
                self.deref_mut_ranked()
            }
        }
        impl ::std::ops::Deref for dyn #class {
            type Target = #value;
            fn deref(&self) -> &Self::Target {
                self.#get_ref()
            }
        }
        impl ::std::ops::DerefMut for dyn #class {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.#get_mut()
            }
        }
        impl ::std::ops::Deref for #value {
            type Target = <dyn #parent as ::octant_object::class::Class>::Value;
            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }
        impl ::std::ops::DerefMut for #value {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.parent
            }
        }
        impl ::octant_object::class::Class for dyn #class {
            type Value = #value;
        }
        impl ::octant_object::class::ClassValue for #value {
            type Dyn = dyn #class;
        }
        impl octant_object::class::Subclass for dyn #class {
            type Parent = dyn #parent;
        }
        impl octant_object::class::Ranked for #value {
            type Rank = ::octant_object::class::Succ<<<dyn #parent as ::octant_object::class::Class>::Value as octant_object::class::Ranked>::Rank>;
        }
        impl ::std::fmt::Debug for #value {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut f = f.debug_struct(::std::stringify!(#class));
                ::octant_object::class::DebugClass::fmt_class(self, &mut f);
                f.finish()
            }
        }
    };
    Ok(output)
}

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
    match input_data {
        Data::Struct(strukt) => {
            let DataStruct {
                struct_token,
                fields,
                semi_token,
            } = strukt;
            let field_names: Vec<_> = fields.iter().skip(1).map(|x| &x.ident).collect();
            output = quote! {
                impl ::octant_object::class::DebugClass for #input_ident {
                    fn fmt_class(&self, f: &mut ::std::fmt::DebugStruct) {
                        ::octant_object::class::DebugClass::fmt_class(&self.parent, f);
                        #(
                            f.field(std::stringify!(#field_names), &self.#field_names);
                        )*
                    }
                }
            };
        }
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
    Ok(output)
}
