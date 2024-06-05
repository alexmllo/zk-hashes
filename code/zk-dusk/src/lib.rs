// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![no_std]
#![doc = include_str!("../README.md")]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

extern crate alloc;

mod error;
pub use error::Error;

pub mod hades;
pub mod rescue;
pub mod griffin;
pub mod anemoi;
pub mod arion;

mod hash;
/// OK
pub mod news;

#[cfg(feature = "zk")]
pub use hash::gadget::HashGadget;
pub use hash::gadget::HashableGadget;
pub use hash::scalar::Hash;
pub use hash::Domain;

#[cfg(feature = "encryption")]
mod encryption;

#[cfg(feature = "encryption")]
#[cfg(feature = "zk")]
pub use encryption::gadget::{decrypt_gadget, encrypt_gadget};
#[cfg(feature = "encryption")]
pub use encryption::{decrypt, encrypt};
