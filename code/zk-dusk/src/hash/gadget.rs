// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use core::marker::PhantomData;

use alloc::vec::Vec;

use dusk_plonk::prelude::{Composer, Witness};
use dusk_safe::Sponge;

use crate::{news::NewableSafe, Domain};

use super::io_pattern;

pub trait HashableGadget {
    fn digest(domain: Domain, composer: &mut Composer, input: &[Witness]) -> Vec<Witness>;
}

/// OK
pub struct HashGadget<'a, GadPerm, const WIDTH: usize> {
    domain: Domain,
    input: Vec<&'a [Witness]>,
    output_len: usize,
    phantom: PhantomData<GadPerm>,
}

impl<'a, GadPerm: NewableSafe<WIDTH>, const WIDTH: usize> HashGadget<'a, GadPerm, WIDTH> {
    /// Create a new hash.
    pub fn new(domain: Domain) -> Self {
        Self {
            domain,
            input: Vec::new(),
            output_len: 1,
            phantom: PhantomData,
        }
    }

    /// Override the length of the hash output (default value is 1) when using
    /// the hash for anything other than hashing a merkle tree or
    /// encryption.
    pub fn output_len(&mut self, output_len: usize) {
        if self.domain == Domain::Other && output_len > 0 {
            self.output_len = output_len;
        }
    }

    /// Update the hash input.
    pub fn update(&mut self, input: &'a [Witness]) {
        self.input.push(input);
    }

    /// Finalize the hash.
    ///
    /// # Panics
    /// This function panics when the io-pattern can not be created with the
    /// given domain and input, e.g. using [`Domain::Merkle4`] with an input
    /// anything other than 4 Scalar.
    pub fn finalize(&self, composer: &mut Composer) -> Vec<Witness> {
        // Generate the hash using the sponge framework:
        let mut sponge = Sponge::start(
            GadPerm::new(composer),
            io_pattern(self.domain, &self.input, self.output_len)
                .expect("io-pattern should be valid"),
            self.domain.into(),
        )
        .expect("at this point the io-pattern is valid");

        // absorb the input
        for input in self.input.iter() {
            sponge
                .absorb(input.len(), input)
                .expect("at this point the io-pattern is valid");
        }

        // squeeze output_len elements
        sponge
            .squeeze(self.output_len)
            .expect("at this point the io-pattern is valid");

        // return the result
        sponge
            .finish()
            .expect("at this point the io-pattern is valid")
    }

    /// Finalize the hash and output JubJubScalar.
    ///
    /// # Panics
    /// This function panics when the io-pattern can not be created with the
    /// given domain and input, e.g. using [`Domain::Merkle4`] with an input
    /// anything other than 4 Scalar.
    pub fn finalize_truncated(&self, composer: &mut Composer) -> Vec<Witness> {
        // finalize the hash as bls-scalar witnesses
        let bls_output = self.finalize(composer);

        // truncate the bls witnesses to 250 bits
        bls_output
            .iter()
            .map(|bls: &Witness| composer.append_logic_xor::<125>(*bls, Composer::ZERO))
            .collect()
    }

    /// Digest an input and calculate the hash as jubjub-scalar immediately
    ///
    /// # Panics
    /// This function panics when the io-pattern can not be created with the
    /// given domain and input, e.g. using [`Domain::Merkle4`] with an input
    /// anything other than 4 Scalar.
    pub fn digest_truncated(
        domain: Domain,
        composer: &mut Composer,
        input: &'a [Witness],
    ) -> Vec<Witness> {
        let mut hash = Self::new(domain);
        hash.update(input);
        hash.finalize_truncated(composer)
    }
}

impl<'a, GadPerm: NewableSafe<WIDTH>, const WIDTH: usize> HashableGadget
    for HashGadget<'a, GadPerm, WIDTH>
{
    /// Digest an input and calculate the hash immediately
    ///
    /// # Panics
    /// This function panics when the io-pattern can not be created with the
    /// given domain and input, e.g. using [`Domain::Merkle4`] with an input
    /// anything other than 4 Scalar.
    fn digest(domain: Domain, composer: &mut Composer, input: &[Witness]) -> Vec<Witness> {
        let mut hash = Self::new(domain);
        hash.update(input);
        hash.finalize(composer)
    }
}
