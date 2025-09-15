use anchor_lang::prelude::*;

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    // Required fields
    name: "ASSDAQ",
    project_url: "https://assdaqbridge.com",
    contacts: "email:todor91@gmail.com",
    policy: "https://github.com/kuchecoin/native-token-transfers/blob/main/SECURITY.md",

    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/kuchecoin/native-token-transfers/tree/main/",
    encryption: "
-----BEGIN PGP PUBLIC KEY BLOCK-----

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
=or/e
-----END PGP PUBLIC KEY BLOCK-----
",
    auditors: "https://github.com/kuchecoin/native-token-transfers/tree/main/audits/",
    acknowledgements: "kuchecoin"
}

// TODO: is there a more elegant way of checking that these 3 features are mutually exclusive?

#[cfg(all(feature = "mainnet", feature = "solana-devnet"))]
compile_error!("Cannot enable both mainnet and solana-devnet features at the same time");

#[cfg(all(feature = "mainnet", feature = "tilt-devnet"))]
compile_error!("Cannot enable both mainnet and tilt-devnet features at the same time");

#[cfg(all(feature = "solana-devnet", feature = "tilt-devnet"))]
compile_error!("Cannot enable both solana-devnet and tilt-devnet features at the same time");

pub mod bitmap;
pub mod clock;
pub mod config;
pub mod error;
pub mod instructions;
pub mod messages;
pub mod peer;
pub mod pending_token_authority;
pub mod queue;
pub mod registered_transceiver;
pub mod spl_multisig;
pub mod transceivers;
pub mod transfer;

use transceivers::wormhole::instructions::*;

use instructions::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "tilt-devnet2")] {
        declare_id!("NTTManager222222222222222222222222222222222");
    } else if #[cfg(feature = "tilt-devnet")] {
        declare_id!("NTTManager111111111111111111111111111111111");
    } else {
        declare_id!("NTTkYAiVKitnNvZjS6MJr6JEjR42movctE4i5i9E38t");
    }
}

pub const TOKEN_AUTHORITY_SEED: &[u8] = b"token_authority";

/// The seed for the session authority account.
///
/// These accounts are used in the `transfer_*` instructions. The user first
/// approves the session authority to spend the tokens, and then the session
/// authority burns or locks the tokens.
/// This is to avoid the user having to pass their own authority to the program,
/// which in general is dangerous, especially for upgradeable programs.
///
/// There is a session authority associated with each transfer, and is seeded by
/// the sender's pubkey, and (the hash of) all the transfer arguments.
/// These seeds essentially encode the user's intent when approving the
/// spending.
///
/// In practice, the approve instruction is going to be atomically bundled with
/// the transfer instruction, so this encoding makes no difference.
/// However, it does allow it to be done in a separate transaction without the
/// risk of a malicious actor redirecting the funds by frontrunning the transfer
/// instruction.
/// In other words, the transfer instruction has no degrees of freedom; all the
/// arguments are determined in the approval step. Then transfer can be
/// permissionlessly invoked by anyone (even if in practice it's going to be the
/// user, atomically).
pub const SESSION_AUTHORITY_SEED: &[u8] = b"session_authority";

pub const VERSION: &str = "3.0.0";

#[program]
pub mod example_native_token_transfers {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
        instructions::initialize(ctx, args)
    }

    pub fn initialize_lut(ctx: Context<InitializeLUT>, recent_slot: u64) -> Result<()> {
        instructions::initialize_lut(ctx, recent_slot)
    }

    pub fn version(_ctx: Context<Version>) -> Result<String> {
        Ok(VERSION.to_string())
    }

    pub fn transfer_burn<'info>(
        ctx: Context<'_, '_, '_, 'info, TransferBurn<'info>>,
        args: TransferArgs,
    ) -> Result<()> {
        instructions::transfer_burn(ctx, args)
    }

    pub fn transfer_lock<'info>(
        ctx: Context<'_, '_, '_, 'info, TransferLock<'info>>,
        args: TransferArgs,
    ) -> Result<()> {
        instructions::transfer_lock(ctx, args)
    }

    pub fn redeem(ctx: Context<Redeem>, args: RedeemArgs) -> Result<()> {
        instructions::redeem(ctx, args)
    }

    pub fn release_inbound_mint<'info>(
        ctx: Context<'_, '_, '_, 'info, ReleaseInboundMint<'info>>,
        args: ReleaseInboundArgs,
    ) -> Result<()> {
        instructions::release_inbound_mint(ctx, args)
    }

    pub fn release_inbound_unlock<'info>(
        ctx: Context<'_, '_, '_, 'info, ReleaseInboundUnlock<'info>>,
        args: ReleaseInboundArgs,
    ) -> Result<()> {
        instructions::release_inbound_unlock(ctx, args)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        instructions::transfer_ownership(ctx)
    }

    pub fn transfer_ownership_one_step_unchecked(ctx: Context<TransferOwnership>) -> Result<()> {
        instructions::transfer_ownership_one_step_unchecked(ctx)
    }

    pub fn claim_ownership(ctx: Context<ClaimOwnership>) -> Result<()> {
        instructions::claim_ownership(ctx)
    }

    pub fn accept_token_authority(ctx: Context<AcceptTokenAuthority>) -> Result<()> {
        instructions::accept_token_authority(ctx)
    }

    pub fn accept_token_authority_from_multisig<'info>(
        ctx: Context<'_, '_, '_, 'info, AcceptTokenAuthorityFromMultisig<'info>>,
    ) -> Result<()> {
        instructions::accept_token_authority_from_multisig(ctx)
    }

    pub fn set_token_authority_one_step_unchecked(
        ctx: Context<SetTokenAuthorityUnchecked>,
    ) -> Result<()> {
        instructions::set_token_authority_one_step_unchecked(ctx)
    }

    pub fn set_token_authority(ctx: Context<SetTokenAuthorityChecked>) -> Result<()> {
        instructions::set_token_authority(ctx)
    }

    pub fn revert_token_authority(ctx: Context<RevertTokenAuthority>) -> Result<()> {
        instructions::revert_token_authority(ctx)
    }

    pub fn claim_token_authority(ctx: Context<ClaimTokenAuthority>) -> Result<()> {
        instructions::claim_token_authority(ctx)
    }

    pub fn claim_token_authority_to_multisig(
        ctx: Context<ClaimTokenAuthorityToMultisig>,
    ) -> Result<()> {
        instructions::claim_token_authority_to_multisig(ctx)
    }

    pub fn set_paused(ctx: Context<SetPaused>, pause: bool) -> Result<()> {
        instructions::set_paused(ctx, pause)
    }

    pub fn set_peer(ctx: Context<SetPeer>, args: SetPeerArgs) -> Result<()> {
        instructions::set_peer(ctx, args)
    }

    pub fn register_transceiver(ctx: Context<RegisterTransceiver>) -> Result<()> {
        instructions::register_transceiver(ctx)
    }

    pub fn deregister_transceiver(ctx: Context<DeregisterTransceiver>) -> Result<()> {
        instructions::deregister_transceiver(ctx)
    }

    pub fn set_outbound_limit(
        ctx: Context<SetOutboundLimit>,
        args: SetOutboundLimitArgs,
    ) -> Result<()> {
        instructions::set_outbound_limit(ctx, args)
    }

    pub fn set_inbound_limit(
        ctx: Context<SetInboundLimit>,
        args: SetInboundLimitArgs,
    ) -> Result<()> {
        instructions::set_inbound_limit(ctx, args)
    }

    pub fn mark_outbox_item_as_released(ctx: Context<MarkOutboxItemAsReleased>) -> Result<bool> {
        instructions::mark_outbox_item_as_released(ctx)
    }

    pub fn set_threshold(ctx: Context<SetThreshold>, threshold: u8) -> Result<()> {
        instructions::set_threshold(ctx, threshold)
    }

    // standalone transceiver stuff

    pub fn set_wormhole_peer(
        ctx: Context<SetTransceiverPeer>,
        args: SetTransceiverPeerArgs,
    ) -> Result<()> {
        transceivers::wormhole::instructions::set_transceiver_peer(ctx, args)
    }

    pub fn receive_wormhole_message(ctx: Context<ReceiveMessage>) -> Result<()> {
        transceivers::wormhole::instructions::receive_message(ctx)
    }

    pub fn release_wormhole_outbound(
        ctx: Context<ReleaseOutbound>,
        args: ReleaseOutboundArgs,
    ) -> Result<()> {
        transceivers::wormhole::instructions::release_outbound(ctx, args)
    }

    pub fn broadcast_wormhole_id(ctx: Context<BroadcastId>) -> Result<()> {
        transceivers::wormhole::instructions::broadcast_id(ctx)
    }

    pub fn broadcast_wormhole_peer(
        ctx: Context<BroadcastPeer>,
        args: BroadcastPeerArgs,
    ) -> Result<()> {
        transceivers::wormhole::instructions::broadcast_peer(ctx, args)
    }
}

// The Version struct is just a dummy type because anchor needs every function
// to have a context. When compiled in CPI mode, anchor generates code that
// assumes that the struct has a lifetime parameter. So in that mode, we bind a
// dummy lifetime parameter (and use it in a dummy account).
// When compiling normally, we don't do this, and just use an empty struct, which anchor is happy with.
#[cfg(feature = "cpi")]
#[derive(Accounts)]
pub struct Version<'info> {
    /// CHECK: refer to comment above
    pub dummy: UncheckedAccount<'info>,
}

#[cfg(not(feature = "cpi"))]
#[derive(Accounts)]
pub struct Version {}
