use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use spl_tlv_account_resolution::state::ExtraAccountMetaList;

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

declare_id!("BgabMDLaxsyB7eGMBt9L22MSk9KMrL4zY2iNe14kyFP5");

/// Index of the sender token account in the accounts passed to the transfer hook
pub const SENDER_TOKEN_ACCOUNT_INDEX: u8 = 0;
/// Index of the mint account in the accounts passed to the transfer hook
pub const MINT_ACCOUNT_INDEX: u8 = 1;
/// Index of the destination token account in the accounts passed to the transfer hook
pub const DESTINATION_TOKEN_ACCOUNT_INDEX: u8 = 2;
/// Index of the authority account in the accounts passed to the transfer hook
pub const AUTHORITY_ACCOUNT_INDEX: u8 = 3;

/// Number of extra accounts in the ExtraAccountMetaList account
pub const EXTRA_ACCOUNTS_LEN: u8 = 2;

#[program]
pub mod dummy_transfer_hook {
    use spl_tlv_account_resolution::{
        account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
    };
    use spl_transfer_hook_interface::instruction::{ExecuteInstruction, TransferHookInstruction};

    use super::*;

    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        let account_metas = vec![
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal {
                        bytes: "dummy_account".as_bytes().to_vec(),
                    },
                    // owner field of the sender token account
                    Seed::AccountData {
                        account_index: SENDER_TOKEN_ACCOUNT_INDEX,
                        data_index: 32,
                        length: 32,
                    },
                ],
                false, // is_signer
                false, // is_writable
            )?,
            ExtraAccountMeta::new_with_seeds(
                &[Seed::Literal {
                    bytes: "counter".as_bytes().to_vec(),
                }],
                false, // is_signer
                true,  // is_writable
            )?,
        ];

        assert_eq!(EXTRA_ACCOUNTS_LEN as usize, account_metas.len());

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &account_metas,
        )?;

        Ok(())
    }

    pub fn transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        ctx.accounts.counter.count += 1;
        Ok(())
    }

    // NOTE: the CPI call makes that the token2022 program makes (naturally) does not
    // follow the anchor calling convention, so we need to implement a fallback
    // instruction to handle the custom instruction
    pub fn fallback<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        data: &[u8],
    ) -> Result<()> {
        let instruction = TransferHookInstruction::unpack(data)?;

        // match instruction discriminator to transfer hook interface execute instruction
        // token2022 program CPIs this instruction on token transfer
        match instruction {
            TransferHookInstruction::Execute { amount } => {
                let amount_bytes = amount.to_le_bytes();

                // invoke custom transfer hook instruction on our program
                __private::__global::transfer_hook(program_id, accounts, &amount_bytes)
            }
            _ => Err(ProgramError::InvalidInstructionData.into()),
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        payer = payer,
        space = ExtraAccountMetaList::size_of(EXTRA_ACCOUNTS_LEN as usize)?,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(
        init,
        payer = payer,
        space = 8 + Counter::INIT_SPACE,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
/// NOTE: this is just a dummy transfer hook to test that the accounts are
/// passed in correctly. Do NOT use this as a starting point in a real
/// application, as it's not secure.
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account authority, can be SystemAccount or PDA owned by another program
    pub authority: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account,
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        seeds = [b"dummy_account", source_token.owner.as_ref()],
        bump
    )]
    /// CHECK: dummy account. It just tests that the off-chain code correctly
    /// computes and the on-chain code correctly passes on the PDA.
    pub dummy_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,
}
