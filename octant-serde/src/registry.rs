use crate::{DeserializeContext, DeserializeWith, Error, Format, RawEncoded};
use anyhow::anyhow;
use catalog::{Builder, BuilderFrom, Registry};
use serde::Serialize;
use std::{
    any::Any,
    collections::HashMap,
    marker::{PhantomData, Unsize},
};

type DeserializeFn<U> =
    for<'c, 'de> fn(&'c DeserializeContext, &'de RawEncoded) -> Result<Box<U>, Error>;

pub struct DeserializeImp<U: ?Sized, T> {
    pub name: &'static str,
    pub deserialize: DeserializeFn<U>,
    phantom: PhantomData<fn() -> T>,
}

impl<U: ?Sized, T: 'static + Unsize<U> + for<'de> DeserializeWith<'de>> DeserializeImp<U, T> {
    pub fn new(package_name: &str, type_name: &str) -> Self {
        let name = Box::leak(Box::new(format!(
            "{}::{}",
            package_name,
            type_name.split_once("::").unwrap().1,
        )));
        DeserializeImp {
            name,
            deserialize: |ctx, de: &RawEncoded| {
                Ok(Box::<T>::new(de.deserialize_as_with::<T>(ctx)?))
            },
            phantom: PhantomData,
        }
    }
}

pub struct DeserializeRegistry {
    deserializers: HashMap<String, &'static (dyn Sync + Send + Any)>,
}

impl Builder for DeserializeRegistry {
    type Output = DeserializeRegistry;

    fn new() -> Self {
        DeserializeRegistry {
            deserializers: HashMap::new(),
        }
    }

    fn build(self) -> Self::Output {
        self
    }
}

impl<U: ?Sized, T> BuilderFrom<&'static DeserializeImp<U, T>> for DeserializeRegistry {
    fn insert(&mut self, element: &'static DeserializeImp<U, T>) {
        assert!(self
            .deserializers
            .insert(element.name.to_string(), &element.deserialize)
            .is_none());
    }
}

impl DeserializeRegistry {
    pub fn deserialize_box<U: 'static + ?Sized>(
        &self,
        ctx: &DeserializeContext,
        typ: &str,
        value: &RawEncoded,
    ) -> Result<Box<U>, Error> {
        let dfn: &dyn Any = *self
            .deserializers
            .get(typ)
            .ok_or_else(|| anyhow!("Could not find deserializer"))?;
        let dfn: &DeserializeFn<U> = dfn
            .downcast_ref()
            .ok_or_else(|| anyhow!("Could not downcast deserializer"))?;
        (*dfn)(ctx, value)
    }
}

pub trait SerializeType {
    fn serialize_type(&self) -> &'static str;
}

pub trait SerializeDyn: SerializeType {
    fn serialize_dyn(&self, format: Format) -> Result<RawEncoded, Error>;
}

impl<T: Serialize + SerializeType> SerializeDyn for T {
    fn serialize_dyn(&self, format: Format) -> Result<RawEncoded, Error> {
        format.serialize_raw(self)
    }
}

pub static DESERIALIZE_REGISTRY: Registry<DeserializeRegistry> = Registry::new();

#[macro_export]
macro_rules! define_serde_impl {
    ($type:ty: $trait:path) => {
        const _: () = {
            #[$crate::reexports::catalog::register($crate::DESERIALIZE_REGISTRY, lazy = true, crate=$crate::reexports::catalog)]
            static IMP: $crate::DeserializeImp<dyn $trait, $type> = $crate::DeserializeImp::new(
                option_env!("OCTANT_SERDE_SHARED_NAME").unwrap_or(env!("CARGO_CRATE_NAME")),
                ::std::any::type_name::<$type>()
            );
            impl $crate::SerializeType for $type {
                fn serialize_type(&self) -> &'static str {
                    IMP.name
                }
            }
        };
    };
}

#[macro_export]
macro_rules! derive_deserialize_with_for_struct {
    {
        struct $struct:ident {
            $($field:ident: $type:ty ),* $(,)?
        }
    } => {
        impl<'de> $crate::DeserializeWith<'de> for $struct {
            fn deserialize_with<D:$crate::reexports::serde::Deserializer<'de>>(ctx:&$crate::DeserializeContext,d:D)->::std::result::Result<Self, D::Error>{
                #[allow(non_camel_case_types)]
                #[derive($crate::reexports::serde::Deserialize)]
                enum Field{
                    $( $field ),*
                }
                struct V<'c>{ctx:&'c $crate::DeserializeContext}
                impl<'c,'de> $crate::reexports::serde::de::Visitor<'de> for V<'c>{
                    type Value = $struct;
                    fn expecting(&self, f:&mut ::std::fmt::Formatter)->::std::fmt::Result{
                        write!(f,::std::stringify!($struct))
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: $crate::reexports::serde::de::MapAccess<'de> {
                        $( let mut $field: Option<$type> = None; )*
                        while let Some(field) = map.next_key::<Field>()? {
                            match field {
                                $(
                                    Field::$field => {
                                        $field = Some(map.next_value_seed($crate::DeserializeWithSeed::<$type>::new(self.ctx))?);
                                    }
                                )*
                            }
                        }
                        $(
                            let $field = $field.ok_or_else(||
                                <A::Error as $crate::reexports::serde::de::Error>::custom(format_args!("Missing field {}",std::stringify!($field)))
                            )?;
                        )*
                        Ok($struct {$($field,)*})
                    }
                }
                d.deserialize_struct(::std::stringify!($struct),&[$(::std::stringify!($field)),*],V{ctx})
            }
        }
    }
}
