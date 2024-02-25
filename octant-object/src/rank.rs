pub trait Nat {}

pub struct Zero;

pub struct Succ<N>(N);

impl Nat for Zero {}

impl<N> Nat for Succ<N> {}

pub trait Ranked {
    type Rank: Nat;
}