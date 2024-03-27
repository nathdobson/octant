use crate::rank::{Nat, Succ, Zero};

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
