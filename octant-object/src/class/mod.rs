//! Traits for specifying classes.
//!

use std::any::Any;
use std::marker::Unsize;
use std::ptr::{DynMetadata, Pointee};

pub trait Class: Any + Unsize<dyn Any> + Pointee<Metadata = DynMetadata<Self>> {
    type Value: ClassValue<Dyn = Self>;
}

pub trait Subclass: Class + Unsize<Self::Parent> {
    type Parent: ?Sized + Class;
}

pub trait ClassValue: Sized + Any + Unsize<Self::Dyn> {
    type Dyn: ?Sized + Class<Value = Self>;
}

pub trait Nat {}

pub struct Zero;

pub struct Succ<N>(N);

impl Nat for Zero {}

impl<N> Nat for Succ<N> {}

pub trait Ranked {
    type Rank: Nat;
}

pub trait DerefRanked<N1: Nat, N2: Nat> {
    type TargetRanked;
    fn deref_ranked(&self) -> &Self::TargetRanked;
}

impl<T> DerefRanked<Zero, Zero> for T {
    type TargetRanked = T;

    fn deref_ranked(&self) -> &Self::TargetRanked {
        self
    }
}

impl<N, T> DerefRanked<Succ<N>, Zero> for T
    where
        N: Nat,
        T: ::std::ops::Deref,
        <T as ::std::ops::Deref>::Target: DerefRanked<N, Zero>,
{
    type TargetRanked = <<T as ::std::ops::Deref>::Target as DerefRanked<N, Zero>>::TargetRanked;

    fn deref_ranked(&self) -> &Self::TargetRanked {
        T::deref(self).deref_ranked()
    }
}

impl<N1, N2, T> DerefRanked<Succ<N1>, Succ<N2>> for T
    where
        N1: Nat,
        N2: Nat,
        T: DerefRanked<N1, N2>,
{
    type TargetRanked = <T as DerefRanked<N1, N2>>::TargetRanked;

    fn deref_ranked(&self) -> &Self::TargetRanked {
        DerefRanked::<N1, N2>::deref_ranked(self)
    }
}

