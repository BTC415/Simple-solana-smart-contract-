use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::mint_to;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::create_metadata_accounts_v2;

declare_id!("9FKLho9AUYScrrKgJbG1mExt5nSgEfk1CNEbR8qBwKTZ");

#[program]
pub mod nft_minting_contract {

    use super::*;

    pub fn nft_format(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        msg!("Minting NFT:");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = mint_to(cpi_ctx, 1);
        if let Err(_) = result {
            return Err(error!(ErrorCode::MintFailed));
        }
        msg!("NFT has been minted!");
        msg!("Metadata account is being created:");
        let accounts = vec![
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        let creators = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        let result = invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                name,
                symbol,
                uri,
                Some(creators),
                1,
                true,
                false,
                None,
                None,
            ),
            &accounts
        );
        if let Err(_) = result {
            return Err(error!(ErrorCode::MetadataCreateFailed));
        }
        msg!("Metadata account has been created");
        Ok(())
    }
}
    }
}
