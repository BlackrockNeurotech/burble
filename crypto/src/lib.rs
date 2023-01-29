//! Bluetooth LE cryptographic toolbox ([Vol 3] Part H, Section 2.2).

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(non_ascii_idents)]
#![warn(single_use_lifetimes)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::inline_always)]
#![allow(clippy::module_name_repetitions)]
// #![warn(clippy::restriction)]
#![warn(clippy::assertions_on_result_states)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::default_union_representation)]
#![warn(clippy::deref_by_slicing)]
#![warn(clippy::empty_drop)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exhaustive_enums)]
#![warn(clippy::exit)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::missing_enforced_import_renames)]
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::mod_module_files)]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::pattern_type_mismatch)]
#![warn(clippy::print_stdout)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_add)]
#![warn(clippy::string_to_string)]
#![warn(clippy::suspicious_xor_used_as_pow)]
#![warn(clippy::todo)]
#![warn(clippy::try_err)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unnecessary_safety_comment)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unseparated_literal_suffix)]

use std::fmt::{Debug, Display, Formatter};
use std::mem;

use structbuf::{Packer, Unpacker};

pub use crate::{cmac::*, p256::*};

mod cmac;
mod p256;

/// Codec for packing and unpacking SMP command parameters.
pub trait Codec: Sized {
    /// Packs command parameters into an SMP PDU.
    fn pack(&self, p: &mut Packer);

    /// Unpacks command parameters from an SMP PDU.
    ///
    /// Implementations should only return [`None`] if one of the unpacked
    /// values cannot be decoded. The final [`Unpacker`] status is checked by
    /// the caller.
    #[must_use]
    fn unpack(p: &mut Unpacker) -> Option<Self>;
}

/// Implements [`Codec`] for a `u128` newtype struct.
macro_rules! u128_codec {
    ($T:ty) => {
        impl $crate::Codec for $T {
            #[inline(always)]
            fn pack(&self, p: &mut Packer) {
                p.u128(self.0);
            }

            #[inline(always)]
            fn unpack(p: &mut Unpacker) -> Option<Self> {
                Some(Self(p.u128()))
            }
        }
    };
}

/// Implements [`subtle::ConstantTimeEq`] for a newtype struct.
macro_rules! ct_newtype {
    ($T:ty) => {
        impl subtle::ConstantTimeEq for $T {
            #[inline(always)]
            fn ct_eq(&self, other: &Self) -> subtle::Choice {
                self.0.ct_eq(&other.0)
            }
        }

        impl PartialEq for $T {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                bool::from(subtle::ConstantTimeEq::ct_eq(self, other))
            }
        }
    };
}

/// 56-bit device address in big-endian byte order used by [`DHKey::f5`] and
/// [`MacKey::f6`] functions ([Vol 3] Part H, Section 2.2.7 and 2.2.8).
#[derive(Clone, Copy, Debug)]
#[must_use]
#[repr(transparent)]
pub struct Addr([u8; 7]);

impl Addr {
    /// Creates a device address from a little-endian byte array.
    #[inline]
    pub fn from_le_bytes(is_random: bool, mut v: [u8; 6]) -> Self {
        v.reverse();
        let mut a = [0; 7];
        a[0] = u8::from(is_random);
        a[1..].copy_from_slice(&v);
        Self(a)
    }
}

/// Concatenated `AuthReq`, OOB data flag, and IO capability parameters used by
/// [`MacKey::f6`] function ([Vol 3] Part H, Section 2.2.8).
#[derive(Clone, Copy, Debug)]
#[must_use]
#[repr(transparent)]
pub struct IoCap([u8; 3]);

impl IoCap {
    /// Creates new `IoCap` parameter.
    #[inline(always)]
    pub fn new(auth_req: u8, oob_data: bool, io_cap: u8) -> Self {
        Self([auth_req, u8::from(oob_data), io_cap])
    }
}

/// 128-bit random nonce value ([Vol 3] Part H, Section 2.3.5.6).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
#[repr(transparent)]
pub struct Nonce(u128);

u128_codec!(Nonce);

impl Nonce {
    /// Generates a new non-zero random nonce value from the OS CSPRNG.
    ///
    /// # Panics
    ///
    /// Panics if the OS CSPRNG is broken.
    #[allow(clippy::new_without_default)]
    #[inline]
    pub fn new() -> Self {
        use rand_core::{OsRng, RngCore};
        let mut b = [0; mem::size_of::<u128>()];
        OsRng.fill_bytes(b.as_mut_slice());
        let n = u128::from_ne_bytes(b);
        assert_ne!(n, 0);
        Self(n)
    }

    /// Generates LE Secure Connections confirm value
    /// ([Vol 3] Part H, Section 2.2.6).
    #[inline]
    pub fn f4(&self, u: &PublicKeyX, v: &PublicKeyX, z: u8) -> Confirm {
        let mut m = AesCmac::new(&Key::new(self.0));
        m.update(u.as_be_bytes())
            .update(v.as_be_bytes())
            .update([z]);
        Confirm(m.finalize())
    }

    /// Generates LE Secure Connections numeric comparison value
    /// ([Vol 3] Part H, Section 2.2.9).
    #[inline]
    pub fn g2(&self, u: &PublicKeyX, v: &PublicKeyX, y: &Self) -> NumCompare {
        let mut m = AesCmac::new(&Key::new(self.0));
        m.update(u.as_be_bytes())
            .update(v.as_be_bytes())
            .update(y.0.to_be_bytes());
        #[allow(clippy::cast_possible_truncation)]
        NumCompare(m.finalize() as u32 % 1_000_000)
    }
}

/// LE Secure Connections confirm value generated by [`Nonce::f4`].
#[derive(Clone, Copy, Debug, Eq)]
#[must_use]
#[repr(transparent)]
pub struct Confirm(u128);

u128_codec!(Confirm);
ct_newtype!(Confirm);

/// LE Secure Connections numeric comparison value generated by [`Nonce::g2`].
#[derive(Clone, Copy, Eq, PartialEq)]
#[must_use]
#[repr(transparent)]
pub struct NumCompare(u32);

impl Debug for NumCompare {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NumCompare")
            .field(&format_args!("{:06}", self.0))
            .finish()
    }
}

impl Display for NumCompare {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:06}", self.0)
    }
}

/// 128-bit key used to compute LE Secure Connections check value
/// ([Vol 3] Part H, Section 2.2.8).
#[must_use]
#[repr(transparent)]
pub struct MacKey(Key);

debug_secret!(MacKey);

impl MacKey {
    /// Generates LE Secure Connections check value
    /// ([Vol 3] Part H, Section 2.2.8).
    #[inline]
    pub fn f6(&self, n1: Nonce, n2: Nonce, r: u128, io_cap: IoCap, a1: Addr, a2: Addr) -> Check {
        let mut m = AesCmac::new(&self.0);
        m.update(n1.0.to_be_bytes())
            .update(n2.0.to_be_bytes())
            .update(r.to_be_bytes())
            .update(io_cap.0)
            .update(a1.0)
            .update(a2.0);
        Check(m.finalize())
    }
}

/// LE Secure Connections check value generated by [`MacKey::f6`].
#[derive(Clone, Copy, Debug, Eq)]
#[must_use]
#[repr(transparent)]
pub struct Check(u128);

u128_codec!(Check);
ct_newtype!(Check);

#[allow(clippy::unusual_byte_groupings)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonce() {
        // No fair dice rolls for us!
        assert_ne!(Nonce::new(), Nonce::new());
    }

    /// Confirm value generation function ([Vol 3] Part H, Section D.2).
    #[test]
    fn nonce_f4() {
        let u = PublicKeyX::from_be_bytes(u256(
            0x20b003d2_f297be2c_5e2c83a7_e9f9a5b9,
            0xeff49111_acf4fddb_cc030148_0e359de6,
        ));
        let v = PublicKeyX::from_be_bytes(u256(
            0x55188b3d_32f6bb9a_900afcfb_eed4e72a,
            0x59cb9ac2_f19d7cfb_6b4fdd49_f47fc5fd,
        ));
        let x = Nonce(0xd5cb8454_d177733e_ffffb2ec_712baeab);
        assert_eq!(x.f4(&u, &v, 0).0, 0xf2c916f1_07a9bd1c_f1eda1be_a974872d);
    }

    /// Numeric comparison generation function ([Vol 3] Part H, Section D.5).
    #[allow(clippy::unreadable_literal)]
    #[test]
    fn nonce_g2() {
        let u = PublicKeyX::from_be_bytes(u256(
            0x20b003d2_f297be2c_5e2c83a7_e9f9a5b9,
            0xeff49111_acf4fddb_cc030148_0e359de6,
        ));
        let v = PublicKeyX::from_be_bytes(u256(
            0x55188b3d_32f6bb9a_900afcfb_eed4e72a,
            0x59cb9ac2_f19d7cfb_6b4fdd49_f47fc5fd,
        ));
        let x = Nonce(0xd5cb8454_d177733e_ffffb2ec_712baeab);
        let y = Nonce(0xa6e8e7cc_25a75f6e_216583f7_ff3dc4cf);
        assert_eq!(x.g2(&u, &v, &y), NumCompare(0x2f9ed5ba % 1_000_000));
    }

    /// Check value generation function ([Vol 3] Part H, Section D.4).
    #[test]
    fn mac_key_f6() {
        let k = MacKey(Key::new(0x2965f176_a1084a02_fd3f6a20_ce636e20));
        let n1 = Nonce(0xd5cb8454_d177733e_ffffb2ec_712baeab);
        let n2 = Nonce(0xa6e8e7cc_25a75f6e_216583f7_ff3dc4cf);
        let r = 0x12a3343b_b453bb54_08da42d2_0c2d0fc8;
        let io_cap = IoCap([0x01, 0x01, 0x02]);
        let a1 = Addr([0x00, 0x56, 0x12, 0x37, 0x37, 0xbf, 0xce]);
        let a2 = Addr([0x00, 0xa7, 0x13, 0x70, 0x2d, 0xcf, 0xc1]);
        let c = k.f6(n1, n2, r, io_cap, a1, a2);
        assert_eq!(c.0, 0xe3c47398_9cd0e8c5_d26c0b09_da958f61);
    }
}
