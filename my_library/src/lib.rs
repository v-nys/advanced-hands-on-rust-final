// #![warn(missing_docs)]
//! `my_library` provides a suite of helpers to create games with Bevy.
//!
//! ## What's Included?
//!
//! `my_library` includes:
//!
//! * Random number generation facilities.
//!
//! ## Feature Flags
//!
//! The following feature flags are supported: `xorshift`, `pcg`, `locking`.
//!
//! ### Random Number Generation
//!
//! * The `locking` feature enables interior mutability inside [`RandomNumberGenerator`],
//!   allowing it to be used as a resource (`Res<RandomNumberGenerator`)
//!   rather than requiring mutability (`ResMut<RandomNumberGenerator>`)
//! * You can control which random number generation algorithm is used by
//!   specifying *one* of:
//!   * `xorshift` to use the XorShift algorithm.
//!   * `pcg` to use the PCG algorithm.

/// [`RandomNumberGenerator`] wraps the `rand` crate. The `rand` crate
/// is re-exported for your convenience.
pub use rand;

mod bevy_framework;
pub use bevy_framework::*;

#[cfg(not(feature = "locking"))]
mod random;

#[cfg(not(feature = "locking"))]
pub use random::*;

#[cfg(feature = "locking")]
mod random_locking;
#[cfg(feature = "locking")]
pub use random_locking::*;

#[cfg(test)]
mod tests {
    use super::*;
}
