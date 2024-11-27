use anchor_lang::prelude::*;
use anchor_spl::{
    token::{TokenAccount, Mint, Token},
    associated_token::AssociatedToken,
    metadata::{
        CreateMetadataAccountsV3, Metadata, mpl_token_metadata,
        create_metadata_accounts_v3,
    },
};
declare_id!("CYEX79QtyVPz99LtKWamTvmMmGpMkKGH3mBF2tpRGmEs");
#[program]
pub mod testing_program {
    use super::*;
    pub fn create_token_mint(
        ctx: Context<CreateTokenMintContext>,
        key: Pubkey,
        token_decimals: u8,
        token_name: String,
        token_symbol: String,
        token_uri: String,
        seller_fee_basis_points: u16,
    ) -> Result<()> {
        let token_data: mpl_token_metadata::types::DataV2 = mpl_token_metadata::types::DataV2 {
            name: token_name,
            symbol: token_symbol,
            uri: token_uri,
            seller_fee_basis_points: seller_fee_basis_points,
            creators: None,
            collection: None,
            uses: None,
        };
        let cpi_accounts = CreateMetadataAccountsV3 {
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.mint_account.to_account_info(),
            metadata: ctx.accounts.metadata_account.to_account_info(),
            mint_authority: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            cpi_accounts,
        );
        create_metadata_accounts_v3(cpi_ctx, token_data, true, false, None)?;
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(token_decimals:u8)]
pub struct CreateTokenMintContext<'info> {
    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata",
        token_metadata_program.key().as_ref(),
        mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = token_decimals,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint_account: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub hel: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}
