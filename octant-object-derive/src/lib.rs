#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

extern crate proc_macro;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DataStruct, DeriveInput, Field, Fields,
    GenericArgument, Item, ItemStruct, ItemTrait, Path, PathArguments, Token, TraitItem,
    TraitItemFn, Type, TypeParamBound,
};

struct Args {}

#[proc_macro_attribute]
pub fn class(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parser = syn::meta::parser(|meta| Err(syn::Error::new(meta.path.span(), "No parameters expected")));
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

    let signatures: Vec<_> = items
        .iter()
        .map(|i| match i {
            TraitItem::Fn(f) => {
                let TraitItemFn {
                    attrs,
                    sig,
                    default,
                    semi_token,
                } = f;
                quote! {
                    #(#attrs)*
                    #sig ;
                }
            }
            _ => todo!(),
        })
        .collect();
    let output = quote! {
        #(#attrs)*
        #vis #unsafety #auto_token #trait_token #ident #generics #colon_token #supertraits {
            fn #get_ref(&self) -> &#value;
            fn #get_mut(&mut self) -> &mut #value;
            #(#signatures)*
        }
        pub type #rc_class = ::octant_object::reexports::octant_reffed::rc::Rc2<dyn 'static + #class>;
        impl<__super_secret__T> #unsafety #class #generics for __super_secret__T where
            __super_secret__T: #supertraits,
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
            #(#items)*
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
                impl ::std::fmt::Debug for #input_ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        let mut f = f.debug_struct(::std::stringify!(#input_ident));
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
