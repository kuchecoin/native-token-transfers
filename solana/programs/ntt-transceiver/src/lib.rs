use anchor_lang::prelude::*;
use solana_security_txt::security_txt;
#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
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

pub mod messages;
pub mod peer;
pub mod wormhole;

use wormhole::instructions::*;

declare_id!("Ee6jpX9oq2EsGuqGb6iZZxvtcpmMGZk8SAUbnQy4jcHR");

pub const TRANSCEIVER_TYPE: &str = "wormhole";

#[program]
pub mod ntt_transceiver {

    use super::*;

    pub fn transceiver_type(_ctx: Context<TransceiverType>) -> Result<String> {
        Ok(TRANSCEIVER_TYPE.to_string())
    }

    pub fn set_wormhole_peer(
        ctx: Context<SetTransceiverPeer>,
        args: SetTransceiverPeerArgs,
    ) -> Result<()> {
        set_transceiver_peer(ctx, args)
    }

    pub fn receive_wormhole_message(ctx: Context<ReceiveMessage>) -> Result<()> {
        wormhole::instructions::receive_message(ctx)
    }

    pub fn release_wormhole_outbound(
        ctx: Context<ReleaseOutbound>,
        args: ReleaseOutboundArgs,
    ) -> Result<()> {
        wormhole::instructions::release_outbound(ctx, args)
    }

    pub fn broadcast_wormhole_id(ctx: Context<BroadcastId>) -> Result<()> {
        wormhole::instructions::broadcast_id(ctx)
    }

    pub fn broadcast_wormhole_peer(
        ctx: Context<BroadcastPeer>,
        args: BroadcastPeerArgs,
    ) -> Result<()> {
        wormhole::instructions::broadcast_peer(ctx, args)
    }
}

#[derive(Accounts)]
pub struct TransceiverType {}
