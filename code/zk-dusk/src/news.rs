#[cfg(feature = "zk")]
use dusk_plonk::prelude::Composer;
use dusk_plonk::prelude::Witness;
use dusk_safe::Safe;

#[cfg(feature = "zk")]

/// OK
pub trait NewableSafe<const W: usize> {
    /// OK
    type T<'b>: Safe<Witness, W>;
    /// OK
    fn new(composer: &mut Composer) -> Self::T<'_>;
}

pub trait NewableScalar {
    fn new() -> Self;
}
