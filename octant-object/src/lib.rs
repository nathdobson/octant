#![feature(trait_upcasting)]

pub trait Nat {}

pub struct Zero;

pub struct Succ<N>(N);

impl Nat for Zero {}

impl<N> Nat for Succ<N> {}

// pub trait NatSub<N: Nat>: Nat {
//     type Output: Nat;
// }
//
// impl<N: Nat> NatSub<Zero> for N {
//     type Output = N;
// }
// impl<N1: Nat, N2: NatSub<N1>> NatSub<Succ<N1>> for Succ<N2> {
//     type Output = <N2 as NatSub<N1>>::Output;
// }

pub trait Ranked {
    type Rank: Nat;
}

#[macro_export]
macro_rules! define_class {
    (
        pub class {
            $($field:ident : $type:ty),* $(,)?
        }
    ) => {
        pub struct Value {
            $($field : $type,)*
        }
        pub trait Trait : 'static + ::std::marker::Sync + ::std::marker::Send + ::std::any::Any  {
            fn concrete(&self) -> &Value;
        }
        impl Trait for Value {
            fn concrete(&self) -> &Value{
                self
            }
        }
        impl<T: ::std::ops::Deref + 'static + Sync + Send> Trait for T where <T as ::std::ops::Deref>::Target : Trait{
            fn concrete(&self) -> &Value{
                T::deref(self).concrete()
            }
        }

        impl ::std::ops::Deref for dyn Trait {
            type Target = Value;
            fn deref(&self) -> &Self::Target {
                self.concrete()
            }
        }

        impl $crate::Ranked for Value{
            type Rank = $crate::Zero;
        }
    };
    (
        pub class : $parent:tt {
            $($field:ident : $type:ty),* $(,)?
        }
    ) => {
            pub struct Value {
                parent: super::$parent::Value,
                $($field : $type,)*
            }
            impl $crate::Ranked for Value{
                type Rank = $crate::Succ<<super::$parent::Value as $crate::Ranked>::Rank>;
            }
            trait Ranked <N1:$crate::Nat, N2:$crate::Nat> : super::$parent::Trait {
                fn concrete_ranked(&self) -> &Value;
            }
            impl Ranked<$crate::Zero, $crate::Zero> for Value {
                 fn concrete_ranked(&self) -> &Value{
                    self
                }
            }
            impl<N, T> Ranked<$crate::Succ<N>, $crate::Zero> for T where
                N: $crate::Nat,
                T: ::std::ops::Deref,
                T: super::$parent::Trait,
                <T as ::std::ops::Deref>::Target : Ranked<N,$crate::Zero>,
            {
                 fn concrete_ranked(&self) -> &Value{
                    T::deref(self).concrete_ranked()
                }
            }
            impl<N1,N2, T> Ranked<$crate::Succ<N1>, $crate::Succ<N2>> for T where
                N1: $crate::Nat,
                N2: $crate::Nat,
                T:Ranked<N1,N2>
            {
                 fn concrete_ranked(&self) -> &Value{
                    <T as Ranked<N1,N2>>::concrete_ranked(self)
                }
            }
            pub trait Trait: super::$parent::Trait {
                fn concrete(&self) -> &Value;
            }
            impl<T> Trait for T where
                T: super::$parent::Trait,
                T: $crate::Ranked,
                T: Ranked<T::Rank, <Value as $crate::Ranked>::Rank>,
            {
                fn concrete(&self) -> &Value{
                    self.concrete_ranked()
                }
            }

            impl ::std::ops::Deref for dyn Trait {
                type Target = Value;
                fn deref(&self) -> &Self::Target {
                    Trait::concrete(self)
                }
            }

            impl ::std::ops::Deref for Value {
                type Target = super::$parent::Value;
                fn deref(&self) -> &Self::Target {
                    &self.parent
                }
            }


    };
}

#[cfg(test)]
#[allow(unused_variables, dead_code)]
mod test {
    mod a {
        define_class! {
            pub class{
                x: u32,
            }
        }
        impl Value {
            pub fn new(x: u32) -> Self {
                Value { x }
            }
        }
    }

    mod b {
        use crate::test::a;

        define_class! {
            pub class : a {
                y: u32,
            }
        }
        impl Value {
            pub fn new(x: u32, y: u32) -> Self {
                Value {
                    parent: a::Value::new(x),
                    y,
                }
            }
        }
    }

    mod c {
        use crate::test::b;

        define_class! {
            pub class : b{
                z:u32,
            }
        }
        impl Value {
            pub fn new(x: u32, y: u32, z: u32) -> Self {
                Value {
                    parent: b::Value::new(x, y),
                    z,
                }
            }
        }
    }

    mod d {
        use crate::test::c;

        define_class! {
            pub class :c {
                w:u32,
            }
        }
        impl Value {
            pub fn new(x: u32, y: u32, z: u32, w: u32) -> Self {
                Value {
                    parent: c::Value::new(x, y, z),
                    w,
                }
            }
        }
    }

    fn test_a_a(x: &a::Value) -> &dyn a::Trait {
        x
    }

    fn test_b_a(x: &b::Value) -> &dyn a::Trait {
        x
    }

    fn test_c_a(x: &c::Value) -> &dyn a::Trait {
        x
    }

    fn test_d_a(x: &d::Value) -> &dyn a::Trait {
        x
    }

    fn test_b_b(x: &b::Value) -> &dyn b::Trait {
        x
    }

    fn test_c_b(x: &c::Value) -> &dyn b::Trait {
        x
    }

    fn test_d_b(x: &d::Value) -> &dyn b::Trait {
        x
    }

    fn test_c_c(x: &c::Value) -> &dyn c::Trait {
        x
    }

    fn test_d_c(x: &d::Value) -> &dyn c::Trait {
        x
    }

    fn test_d_d(x: &d::Value) -> &dyn d::Trait {
        x
    }

    fn test_d_b_up(x: &dyn d::Trait) -> &dyn b::Trait {
        x
    }

    #[test]
    fn test() {
        let x = d::Value::new(1, 2, 3, 4);
        let y: &dyn a::Trait = &x;
    }

    // // #[test]
    // // fn test() {
    // //     let a = A::new(1);
    // //     let b = A::new(2);
    // // }
    // //
    // // fn a(x: A) -> ArcA {
    // //     Arc::new(x)
    // // }
    //
    // use std::ops::Deref;
    // use std::ops::Deref;
    //
    // struct A;
    // struct B(A);
    // struct C(B);
    // struct D(C);
    //
    // impl Deref for B {
    //     type Target = A;
    //     fn deref(&self) -> &Self::Target {
    //         &self.0
    //     }
    // }
    // impl Deref for C {
    //     type Target = B;
    //     fn deref(&self) -> &Self::Target {
    //         &self.0
    //     }
    // }
    // impl Deref for D {
    //     type Target = C;
    //     fn deref(&self) -> &Self::Target {
    //         &self.0
    //     }
    // }
    //
    //
    // pub trait Ranked {
    //     type Rank: Nat;
    // }
    // impl Ranked for A {
    //     type Rank = Zero;
    // }
    // impl Ranked for B {
    //     type Rank = Succ<<A as Ranked>::Rank>;
    // }
    // impl Ranked for C {
    //     type Rank = Succ<<B as Ranked>::Rank>;
    // }
    // impl Ranked for D {
    //     type Rank = Succ<<C as Ranked>::Rank>;
    // }
    //
    // trait TransDerefN<N: Nat, T> {
    //     fn trans_deref_n(&self) -> &T;
    // }
    // impl<T> TransDerefN<Zero, T> for T {
    //     fn trans_deref_n(&self) -> &T {
    //         self
    //     }
    // }
    // impl<N: Nat, T, S> TransDerefN<Succ<N>, S> for T
    // where
    //     T: Deref,
    //     T::Target: TransDerefN<N, S>,
    // {
    //     fn trans_deref_n(&self) -> &S {
    //         T::deref(self).trans_deref_n()
    //     }
    // }
    // trait TransDeref<T> {
    //     fn trans_deref(&self) -> &T;
    // }
    // impl<T, S> TransDeref<S> for T
    // where
    //     T: Ranked,
    //     S: Ranked,
    //     T::Rank: NatSub<S::Rank>,
    //     T: TransDerefN<<T::Rank as NatSub<S::Rank>>::Output, S>,
    // {
    //     fn trans_deref(&self) -> &S {
    //         self.trans_deref_n()
    //     }
    // }
    //
    // fn test_a_a(x: A) -> impl TransDeref<A> {
    //     x
    // }
    // fn test_a_b(x: B) -> impl TransDeref<A> {
    //     x
    // }
    // fn test_a_c(x: C) -> impl TransDeref<A> {
    //     x
    // }
    // fn test_a_d(x: D) -> impl TransDeref<A> {
    //     x
    // }
    // fn test_b_b(x: B) -> impl TransDeref<B> {
    //     x
    // }
    // fn test_b_c(x: C) -> impl TransDeref<B> {
    //     x
    // }
    // fn test_b_d(x: D) -> impl TransDeref<B> {
    //     x
    // }
    // fn test_c_c(x: C) -> impl TransDeref<C> {
    //     x
    // }
    // fn test_c_d(x: D) -> impl TransDeref<C> {
    //     x
    // }
    // fn test_d_d(x: D) -> impl TransDeref<D> {
    //     x
    // }
}
