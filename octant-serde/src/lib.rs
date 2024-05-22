#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(trait_upcasting)]
#![feature(unsize)]
#![allow(unused_variables)]
#![deny(unused_must_use)]

mod deserialize_with_impl;

use std::{
    any::{type_name, Any},
    collections::HashMap,
    fmt::Formatter,
    marker::{PhantomData, Unsize},
    sync::Arc,
};

use catalog::{Builder, BuilderFrom, Registry};
use serde::{
    de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor},
    ser::{Error, SerializeStruct},
    Deserializer, Serialize, Serializer,
};
pub use type_map::TypeMap;

pub mod reexports {
    pub use catalog;
    pub use serde;
}

pub type OctantDeserializer<'a, 'de> =
    &'a mut serde_json::Deserializer<serde_json::de::StrRead<'de>>;
pub type OctantSerializer<'a> = &'a mut serde_json::Serializer<&'a mut Vec<u8>>;

pub fn serialize<T: ?Sized + Serialize>(x: &T) -> Result<String, serde_json::Error> {
    let mut vec = vec![];
    let mut serializer = serde_json::Serializer::new(&mut vec);
    x.serialize(&mut serializer)?;
    Ok(String::from_utf8(vec).unwrap())
}

pub fn deserialize<'de, T: DeserializeWith<'de>>(
    ctx: &TypeMap,
    de: &'de str,
) -> Result<T, serde_json::Error> {
    T::deserialize_with(
        ctx,
        &mut serde_json::Deserializer::new(serde_json::de::StrRead::new(de)),
    )
}

pub trait DeserializeWith<'de>: Sized {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error>;
}

pub trait DeserializeArcWith<'de> {
    fn deserialize_arc_with<D: Deserializer<'de>>(
        ctx: &TypeMap,
        d: D,
    ) -> Result<Arc<Self>, D::Error>;
}

impl<'de, T: ?Sized> DeserializeWith<'de> for Arc<T>
where
    T: DeserializeArcWith<'de>,
{
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Arc<T>, D::Error> {
        T::deserialize_arc_with(ctx, d)
    }
}

pub struct DeserializeWithSeed<'c, O>(&'c TypeMap, PhantomData<O>);

impl<'c, T> DeserializeWithSeed<'c, T> {
    pub fn new(c: &'c TypeMap) -> Self {
        DeserializeWithSeed(c, PhantomData)
    }
}

impl<'c, 'de, T> DeserializeSeed<'de> for DeserializeWithSeed<'c, T>
where
    T: DeserializeWith<'de>,
{
    type Value = T;
    fn deserialize<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize_with(self.0, d)
    }
}

type DeserializeFn<U> =
    for<'a, 'de> fn(
        &'a TypeMap,
        OctantDeserializer<'a, 'de>,
    )
        -> Result<Box<U>, <OctantDeserializer<'a, 'de> as Deserializer<'de>>::Error>;

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
            deserialize: |ctx, de| Ok(Box::<T>::new(T::deserialize_with(ctx, de)?)),
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

pub trait SerializeType {
    fn serialize_type(&self) -> &'static str;
}

pub trait SerializeDyn: SerializeType {
    fn serialize_dyn(
        &self,
        s: OctantSerializer,
    ) -> Result<<OctantSerializer as Serializer>::Ok, <OctantSerializer as Serializer>::Error>;
}

impl<T: Serialize + SerializeType> SerializeDyn for T {
    fn serialize_dyn(
        &self,
        s: OctantSerializer,
    ) -> Result<<OctantSerializer as Serializer>::Ok, <OctantSerializer as Serializer>::Error> {
        self.serialize(s)
    }
}

struct DeserializeValue<'a, U: ?Sized> {
    typ: String,
    ctx: &'a TypeMap,
    phantom: PhantomData<U>,
}

trait DeserializeSpec<'de, U: ?Sized, D: Deserializer<'de>> {
    fn deserialize_spec(self, d: D) -> Result<Box<U>, D::Error>;
}

impl<'a, 'de, U: 'static + ?Sized, D: Deserializer<'de>> DeserializeSpec<'de, U, D>
    for DeserializeValue<'a, U>
{
    default fn deserialize_spec(self, d: D) -> Result<Box<U>, D::Error> {
        let expected = type_name::<OctantDeserializer>();
        let found = type_name::<D>();
        Err(D::Error::custom(format_args!(
            "missing deserialize specialization (expected {}, found {})",
            expected, found
        )))
    }
}

impl<'a, 'de, U: 'static + ?Sized> DeserializeSpec<'de, U, OctantDeserializer<'a, 'de>>
    for DeserializeValue<'a, U>
{
    fn deserialize_spec(
        self,
        d: OctantDeserializer<'a, 'de>,
    ) -> Result<Box<U>, <OctantDeserializer<'a, 'de> as Deserializer<'de>>::Error> {
        let imp = *DESERIALIZE_REGISTRY
            .deserializers
            .get(&self.typ)
            .ok_or_else(|| {
                <<OctantDeserializer<'a, 'de> as Deserializer<'de>>::Error as serde::de::Error>::custom(
                    format_args!("Missing deserializer for {}", self.typ),
                )
            })?;
        let imp = imp.downcast_ref::<DeserializeFn<U>>().unwrap();
        imp(self.ctx, d)
    }
}

impl<'a, 'de, U: 'static + ?Sized> DeserializeSeed<'de> for DeserializeValue<'a, U> {
    type Value = Box<U>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        <Self as DeserializeSpec<U, D>>::deserialize_spec(self, deserializer)
    }
}

pub fn deserialize_box<'de, U: 'static + ?Sized, D: Deserializer<'de>>(
    ctx: &TypeMap,
    d: D,
) -> Result<Box<U>, D::Error> {
    struct V<'a, U: 'static + ?Sized> {
        ctx: &'a TypeMap,
        phantom: PhantomData<U>,
    }
    impl<'a, 'de, U: ?Sized> Visitor<'de> for V<'a, U> {
        type Value = Box<U>;

        fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", type_name::<U>())
        }
        fn visit_seq<A>(self, mut d: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let typ = d
                .next_element::<String>()?
                .ok_or_else(|| A::Error::custom("missing type"))?;
            let value = d
                .next_element_seed(DeserializeValue {
                    typ,
                    ctx: self.ctx,
                    phantom: PhantomData,
                })?
                .ok_or_else(|| A::Error::custom("missing value"))?;
            Ok(value)
        }
        fn visit_map<A: MapAccess<'de>>(self, mut d: A) -> Result<Self::Value, A::Error> {
            let typ = d
                .next_key::<String>()?
                .ok_or_else(|| A::Error::custom("missing type"))?;
            if typ != "type" {
                return Err(A::Error::custom("first field should be `type`"));
            }
            let typ = d.next_value::<String>()?;
            let value = d
                .next_key::<String>()?
                .ok_or_else(|| A::Error::custom("missing type"))?;
            if value != "value" {
                return Err(A::Error::custom("second field should be `value`"));
            }
            let value = d.next_value_seed(DeserializeValue {
                typ,
                ctx: self.ctx,
                phantom: PhantomData,
            })?;
            Ok(value)
        }
    }
    d.deserialize_struct(
        type_name::<U>(),
        &["type", "value"],
        V::<U> {
            ctx,
            phantom: PhantomData,
        },
    )
}

trait SerializeSpec<S: Serializer> {
    fn serialize_spec(&self, s: S) -> Result<S::Ok, S::Error>;
}

impl<T: ?Sized + SerializeDyn, S: Serializer> SerializeSpec<S> for T {
    default fn serialize_spec(&self, s: S) -> Result<S::Ok, S::Error> {
        Err(S::Error::custom("Specialization failed."))
    }
}

impl<T: ?Sized + SerializeDyn> SerializeSpec<OctantSerializer<'_>> for T {
    fn serialize_spec(
        &self,
        s: OctantSerializer,
    ) -> Result<<OctantSerializer as Serializer>::Ok, <OctantSerializer as Serializer>::Error> {
        self.serialize_dyn(s)
    }
}

impl Serialize for dyn SerializeDyn {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <dyn SerializeDyn as SerializeSpec<S>>::serialize_spec(self, s)
    }
}

pub fn serialize_box<U: ?Sized + SerializeType + Unsize<dyn SerializeDyn>, S: Serializer>(
    this: &U,
    s: S,
) -> Result<S::Ok, S::Error> {
    let mut s = s.serialize_struct(type_name::<U>(), 2)?;
    s.serialize_field("type", this.serialize_type())?;
    s.serialize_field("value", this as &dyn SerializeDyn)?;
    s.end()
}

pub static DESERIALIZE_REGISTRY: Registry<DeserializeRegistry> = Registry::new();

#[macro_export]
macro_rules! define_serde_trait {
    ($trait:path) => {
        impl<'de> $crate::DeserializeWith<'de> for ::std::boxed::Box<dyn $trait> {
            fn deserialize_with<D>(
                ctx: &$crate::TypeMap,
                d: D,
            ) -> ::std::result::Result<Self, D::Error>
            where
                D: $crate::reexports::serde::Deserializer<'de>,
            {
                $crate::deserialize_box(ctx, d)
            }
        }

        impl $crate::reexports::serde::Serialize for dyn $trait {
            fn serialize<S>(&self, s: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::reexports::serde::Serializer,
            {
                $crate::serialize_box(self, s)
            }
        }
    };
}

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
            $($field:ident: $type:ty, )*
        }
    } => {
        impl<'de> $crate::DeserializeWith<'de> for $struct {
            fn deserialize_with<D:$crate::reexports::serde::Deserializer<'de>>(ctx:&$crate::TypeMap,d:D)->::std::result::Result<Self, D::Error>{
                #[allow(non_camel_case_types)]
                #[derive($crate::reexports::serde::Deserialize)]
                enum Field{
                    $( $field ),*
                }
                struct V<'c>{ctx:&'c $crate::TypeMap}
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
