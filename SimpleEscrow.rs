use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("FILL_WITH_YOUR_PROGRAM_ID");

#[program]
pub mod escrow {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_account;
        escrow.buyer = *ctx.accounts.buyer.key;
        escrow.seller = *ctx.accounts.seller.key;
        escrow.amount = amount;
        msg!("Escrow created for {} SOL", amount);
        Ok(())
    }

    pub fn release_funds(ctx: Context<ReleaseFunds>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_account;
        require!(escrow.is_approved, ErrorCode::NotApproved);

        let transfer_instruction = Transfer {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.escrow_vault.to_account_info(),
        };

        token::transfer(ctx.accounts.into_transfer_context(), escrow.amount)?;
        msg!("Funds released to seller");
        Ok(())
    }

    pub fn approve_escrow(ctx: Context<ApproveEscrow>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_account;
        escrow.is_approved = true;
        msg!("Escrow approved");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(init, payer = buyer, space = 256)]
    pub escrow_account: Account<'info, Escrow>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub seller: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, Escrow>,
    #[account(mut)]
    pub escrow_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct ApproveEscrow<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, Escrow>,
    #[account(mut)]
    pub buyer: Signer<'info>,
}

#[account]
pub struct Escrow {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub is_approved: bool,
}
