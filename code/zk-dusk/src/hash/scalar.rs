// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::news::NewableScalar;
use dusk_bls12_381::BlsScalar;
use dusk_jubjub::JubJubScalar;
use dusk_safe::{Safe, Sponge};

use super::{io_pattern, Domain};

/// Hash any given input into one or several scalar using the Hades
/// permutation strategy. The Hash can absorb multiple chunks of input but will
/// only call `squeeze` once at the finalization of the hash.
/// The output length is set to 1 element per default, but this can be
/// overridden with [`Hash::output_len`].
pub struct Hash<'a, ScalarPermutation: Safe<BlsScalar, WIDTH> + NewableScalar, const WIDTH: usize> {
    domain: Domain,
    input: Vec<&'a [BlsScalar]>,
    output_len: usize,
    phantom: PhantomData<ScalarPermutation>,
}

impl<'a, ScalarPermutation: Safe<BlsScalar, WIDTH> + NewableScalar, const WIDTH: usize>
    Hash<'a, ScalarPermutation, WIDTH>
{
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
    pub fn update(&mut self, input: &'a [BlsScalar]) {
        self.input.push(input);
    }

    /// Finalize the hash.
    ///
    /// # Panics
    /// This function panics when the io-pattern can not be created with the
    /// given domain and input, e.g. using [`Domain::Merkle4`] with an input
    /// anything other than 4 Scalar.
    pub fn finalize(&self) -> Vec<BlsScalar> {
        // Generate the hash using the sponge framework:
        // initialize the sponge
        let mut sponge = Sponge::start(
            ScalarPermutation::new(),
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

    /// Finalize the hash and output the result as a `JubJubScalar` by
    /// truncating the `BlsScalar` output to 250 bits.
    ///
    /// # Panics
    /// This function panics when the io-pattern can not be created with the
    /// given domain and input, e.g. using [`Domain::Merkle4`] with an input
    /// anything other than 4 Scalar.
    pub fn finalize_truncated(&self) -> Vec<JubJubScalar> {
        // bit-mask to 'cast' a bls-scalar result to a jubjub-scalar by
        // truncating the 6 highest bits
        const TRUNCATION_MASK: BlsScalar = BlsScalar::from_raw([
            0xffff_ffff_ffff_ffff,
            0xffff_ffff_ffff_ffff,
            0xffff_ffff_ffff_ffff,
            0x03ff_ffff_ffff_ffff,
        ]);

        // finalize the hash as bls-scalar
        let bls_output = self.finalize();

        bls_output
            .iter()
            .map(|bls| JubJubScalar::from_raw((bls & &TRUNCATION_MASK).reduce().0))
            .collect()
    }

    /// Digest an input and calculate the hash immediately
    ///
    /// # Panics
    /// This function panics when the io-pattern can not be created with the
    /// given domain and input, e.g. using [`Domain::Merkle4`] with an input
    /// anything other than 4 Scalar.
    pub fn digest(domain: Domain, input: &'a [BlsScalar]) -> Vec<BlsScalar> {
        let mut hash = Self::new(domain);
        hash.update(input);
        hash.finalize()
    }

    /// Digest an input and calculate the hash as jubjub-scalar immediately
    ///
    /// # Panics
    /// This function panics when the io-pattern can not be created with the
    /// given domain and input, e.g. using [`Domain::Merkle4`] with an input
    /// anything other than 4 Scalar.
    pub fn digest_truncated(domain: Domain, input: &'a [BlsScalar]) -> Vec<JubJubScalar> {
        let mut hash = Self::new(domain);
        hash.update(input);
        hash.finalize_truncated()
    }
}
