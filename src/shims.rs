//! Solana type shims for v1.16/v3 cross-compatibility.
//!
//! These types replace direct `solana-sdk` imports, providing a client-side
//! `Account` struct that is ABI-compatible with both solana-sdk 1.16 and
//! solana-account v3. Fields removed or deprecated in v3 (e.g. `rent_epoch`)
//! use default values.
//!
//! All types use `#[repr(C)]` to guarantee a stable field layout across
//! different Rust compiler versions (the FFI dylib and the caller may be
//! compiled with different toolchains).

use anchor_lang::solana_program::{
    account_info::Account as AccountTrait, clock::Epoch, pubkey::Pubkey,
};

/// Client-side account representation.
///
/// Layout-identical to both `solana_sdk::account::Account` (v1.16) and
/// `solana_account::Account` (v3), but with `#[repr(C)]` to guarantee
/// consistent field ordering across compiler versions.
///
/// `rent_epoch` is retained for compatibility but defaults to `0` matching
/// the v3 convention.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Account {
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    /// Deprecated in solana v3; retained for ABI layout compatibility.
    pub rent_epoch: Epoch,
}

impl AccountTrait for Account {
    fn get(&mut self) -> (&mut u64, &mut [u8], &Pubkey, bool) {
        (
            &mut self.lamports,
            &mut self.data,
            &self.owner,
            self.executable,
        )
    }
}
pub type Slot = u64;
