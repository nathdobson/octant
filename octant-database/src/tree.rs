use std::{
    fmt,
    fmt::{Debug, Formatter},
    marker::PhantomData,
    sync::{Arc, OnceLock, Weak},
};

use parking_lot::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::{
    de::{DeserializeSeed, Error},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{
    de::{DeserializeForest, DeserializeSnapshotSeed, DeserializeUpdate},
    forest::{ForestId, Forest},
    ser::{SerializeForest, SerializeUpdate, SerializeUpdateAdapter},
    util::{
        deserialize_pair::DeserializePair,
        deserializer_proxy::DeserializerProxy,
        option_seed::OptionSeed,
        pair_struct_seed::PairStructSeed,
        serializer_proxy::SerializerProxy,
        unique_arc::{MaybeUninit2, UniqueArc},
    },
};

#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct TreeId(u64);

pub struct Tree<T: ?Sized> {
    id: OnceLock<TreeId>,
    forest: OnceLock<ForestId>,
    written: Once,
    state: RwLock<T>,
}

pub(crate) trait SerializeTree<SP: SerializerProxy> {
    fn tree_begin_stream(&mut self);
    fn tree_begin_update(&mut self) -> bool;
    fn tree_serialize_update<'up>(
        &self,
        forest: &mut Forest,
        ser_forest: &mut SerializeForest<SP>,
        s: SP::SerializerValue<'up>,
    ) -> Result<<SP::SerializerValue<'up> as Serializer>::Ok, SP::Error>;
    fn tree_end_update(&mut self);
}

impl TreeId {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl<SP: SerializerProxy, T: SerializeUpdate> SerializeTree<SP> for T {
    fn tree_begin_stream(&mut self) {
        self.begin_stream();
    }

    fn tree_begin_update(&mut self) -> bool {
        self.begin_update()
    }

    fn tree_serialize_update<'up>(
        &self,
        forest: &mut Forest,
        ser_forest: &mut SerializeForest<SP>,
        s: SP::SerializerValue<'up>,
    ) -> Result<<SP::SerializerValue<'up> as Serializer>::Ok, SP::Error> {
        self.serialize_update(forest, ser_forest, s)
    }

    fn tree_end_update(&mut self) {
        self.end_update()
    }
}

impl TreeId {
    pub fn new(x: u64) -> Self {
        TreeId(x)
    }
}

impl<T: ?Sized> Tree<T> {
    pub fn new_value(value: T) -> Self
    where
        T: Sized,
    {
        Tree {
            id: OnceLock::new(),
            forest: OnceLock::new(),
            written: Once::new(),
            state: RwLock::new(value),
        }
    }
    pub fn new_id_value(id: TreeId, value: T) -> Self
    where
        T: Sized,
    {
        Tree {
            id: id.into(),
            forest: OnceLock::new(),
            written: Once::new(),
            state: RwLock::new(value),
        }
    }
    pub fn new(value: T) -> Arc<Self>
    where
        T: Sized,
    {
        Arc::new(Self::new_value(value))
    }
    pub fn new_cyclic<F: for<'a> FnOnce(&'a Weak<Self>) -> T>(f: F) -> Arc<Self>
    where
        T: Sized,
    {
        Arc::new_cyclic(|weak| Self::new_value(f(weak)))
    }
    pub(crate) fn id(&self, forest: &Forest) -> TreeId {
        *self.id.get_or_init(|| forest.next_id())
    }
    pub(crate) fn forest(&self, forest: ForestId) -> ForestId {
        *self.forest.get_or_init(|| forest)
    }
    pub(crate) fn write(&self) -> RwLockWriteGuard<T> {
        self.state.write()
    }
    pub(crate) fn read(&self) -> RwLockReadGuard<T> {
        self.state.read()
    }
    pub(crate) fn try_write(&self) -> Option<RwLockWriteGuard<T>> {
        self.state.try_write()
    }
    pub(crate) fn try_read(&self) -> Option<RwLockReadGuard<T>> {
        self.state.try_read()
    }
}
impl<SP: SerializerProxy> Tree<dyn SerializeTree<SP>> {
    pub fn fmt_weak(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(id) = self.id.get() {
            Debug::fmt(id, f)?;
        }
        Ok(())
    }
}

impl<T: ?Sized + Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Tree");
        if let Some(id) = self.id.get() {
            f.field("id", &id);
        }
        if let Some(state) = self.state.try_read() {
            f.field("state", &&*state);
        }
        f.finish()
    }
}

impl<T: SerializeUpdate + 'static> SerializeUpdate for Arc<Tree<T>> {
    fn begin_stream(&mut self) {}

    fn begin_update(&mut self) -> bool {
        true
    }

    fn serialize_update<S: Serializer, SP: SerializerProxy>(
        &self,
        forest: &mut Forest,
        ser_forest: &mut SerializeForest<SP>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        let id = self.id(forest);
        let mut new = false;
        self.written.call_once(|| new = true);
        let mut s = s.serialize_struct("Arc", 2)?;
        s.serialize_field("id", &id)?;
        if new {
            ser_forest.trees.insert(id, self.clone());
            let mut lock = self
                .try_write()
                .expect("Could not lock tree for serialization. Is there an Arc cycle?");
            lock.begin_stream();
            assert!(lock.begin_update());
            s.serialize_field(
                "value",
                &Some(SerializeUpdateAdapter::new(&*lock, forest, ser_forest)),
            )?;
            lock.end_update();
        } else {
            s.serialize_field("value", &Option::<()>::None)?;
        }
        s.end()
    }

    fn end_update(&mut self) {}
}

impl<T: ?Sized> SerializeUpdate for Weak<Tree<T>> {
    fn begin_stream(&mut self) {}

    fn begin_update(&mut self) -> bool {
        true
    }

    fn serialize_update<S: Serializer, SP: SerializerProxy>(
        &self,
        forest: &mut Forest,
        ser_forest: &mut SerializeForest<SP>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(this) = self.upgrade() {
            s.serialize_some(&this.id(forest))
        } else {
            s.serialize_none()
        }
    }

    fn end_update(&mut self) {}
}

impl<'de, T: 'static + Sync + Send + for<'de2> DeserializeUpdate<'de2>> DeserializeUpdate<'de>
    for Arc<Tree<T>>
{
    fn deserialize_snapshot<D: Deserializer<'de>, DP: DeserializerProxy>(
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<Self, D::Error> {
        struct V<'a, T, DP: DeserializerProxy> {
            forest: &'a mut DeserializeForest<DP>,
            phantom: PhantomData<T>,
        }
        impl<
                'a,
                'de,
                T: 'static + Sync + Send + for<'de2> DeserializeUpdate<'de2>,
                DP: DeserializerProxy,
            > DeserializePair<'de> for V<'a, T, DP>
        {
            type First = TreeId;
            type Second = Arc<Tree<T>>;

            fn deserialize_first<D: Deserializer<'de>>(
                &mut self,
                d: D,
            ) -> Result<Self::First, D::Error> {
                TreeId::deserialize(d)
            }

            fn deserialize_second<D: Deserializer<'de>>(
                &mut self,
                key: Self::First,
                d: D,
            ) -> Result<Self::Second, D::Error> {
                if let Some(value) = self.forest.values.get(&key) {
                    Option::<!>::deserialize(d)?;
                    return Ok(Arc::downcast(value.clone())
                        .map_err(|_| D::Error::custom(format_args!("downcast failed")))?);
                }
                self.forest
                    .holes
                    .entry(key)
                    .or_insert_with(|| UniqueArc::<MaybeUninit2<Tree<T>>>::new_uninit());
                let value = OptionSeed::new(DeserializeSnapshotSeed::<T, DP>::new(self.forest))
                    .deserialize(d)?
                    .ok_or_else(|| D::Error::custom(format_args!("Missing value")))?;
                let hole = self
                    .forest
                    .holes
                    .remove(&key)
                    .ok_or_else(|| D::Error::custom(format_args!("Duplicate value")))?;
                let hole = UniqueArc::downcast::<MaybeUninit2<Tree<T>>>(hole)
                    .map_err(|_| D::Error::custom(format_args!("downcast failed")))?;
                let value = hole.init(Tree::new_id_value(key, value));
                self.forest.values.insert(key, value.clone());
                self.forest.updaters.insert(key, value.clone());
                Ok(value)
            }
        }
        PairStructSeed::new(
            "Arc",
            &["id", "value"],
            V {
                forest,
                phantom: PhantomData,
            },
        )
        .deserialize(d)
    }

    fn deserialize_update<D: Deserializer<'de>, DP: DeserializerProxy>(
        &mut self,
        table: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<(), D::Error> {
        *self = Self::deserialize_snapshot(table, d)?;
        Ok(())
    }
}

impl<'de, T: 'static + Sync + Send> DeserializeUpdate<'de> for Weak<Tree<T>> {
    fn deserialize_snapshot<D: Deserializer<'de>, DP: DeserializerProxy>(
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<Self, D::Error> {
        let key = Option::<TreeId>::deserialize(d)?;
        if let Some(key) = key {
            if let Some(value) = forest.values.get(&key) {
                Ok(Arc::downgrade(
                    &Arc::downcast(value.clone())
                        .map_err(|_| D::Error::custom("downcast failed"))?,
                ))
            } else {
                Ok(UniqueArc::downcast_downgrade_uninit::<Tree<T>>(
                    forest
                        .holes
                        .entry(key)
                        .or_insert_with(|| UniqueArc::<MaybeUninit2<Tree<T>>>::new_uninit()),
                )
                .ok_or_else(|| D::Error::custom("downcast failed"))?)
            }
        } else {
            Ok(Weak::new())
        }
        // Ok(forest
        //     .entries
        //     .entry(key)
        //     .or_insert_with(|| ArcOrEmpty::Empty(UniqueArc::new_uninit()))
        //     .weak())
    }

    fn deserialize_update<D: Deserializer<'de>, DP: DeserializerProxy>(
        &mut self,
        table: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<(), D::Error> {
        *self = Self::deserialize_snapshot(table, d)?;
        Ok(())
    }
}

impl Debug for TreeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}
