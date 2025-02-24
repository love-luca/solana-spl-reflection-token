#![feature(trivial_bounds)]

use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, TokenAccount, Transfer, Token},
};
use anchor_lang::solana_program::instruction::Instruction; // Correct import for Instruction

declare_id!("AeDRnzQh9VJswa71LWQrcP1R1AhT2bdtKmkjsCvyUzq6");

#[program]
pub mod solami_rewards {
    use super::*;

    /// Initialize the treasury account
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing treasury account");
        ctx.accounts.treasury_account.taxed_amount = 0;
        Ok(())
    }

    /// Transfer with 10% tax
    pub fn transfer_with_tax(ctx: Context<TransferWithTax>, amount: u64) -> Result<()> {
        msg!("Transferring with 10% tax");
        let tax = amount / 10; // 10% tax
        let net_amount = amount - tax;

        // Transfer net amount to recipient
        let transfer_cpi = Transfer {
            from: ctx.accounts.sender_token.to_account_info(),
            to: ctx.accounts.recipient_token.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_cpi), net_amount)?;

        // Distribute tax to various pools
        let rewards_pool_tax = tax * 4 / 10;
        let liquidity_growth_tax = tax * 3 / 10;
        let team_fee_tax = tax * 1 / 10;
        let treasury_reserve_tax = tax * 1 / 10;
        let marketing_growth_tax = tax * 5 / 100;
        let development_fund_tax = tax * 5 / 100;

        // Transfer tax portions to respective accounts
        let tax_cpi = Transfer {
            from: ctx.accounts.sender_token.to_account_info(),
            to: ctx.accounts.treasury_token.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tax_cpi), rewards_pool_tax)?;
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tax_cpi), liquidity_growth_tax)?;
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tax_cpi), team_fee_tax)?;
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tax_cpi), treasury_reserve_tax)?;
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tax_cpi), marketing_growth_tax)?;
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tax_cpi), development_fund_tax)?;

        // Track tax collected
        ctx.accounts.treasury_account.taxed_amount += tax;
        Ok(())
    }

    /// Swap taxed SOLAMI for SOL via Jupiter
    pub fn swap_taxed_tokens(ctx: Context<SwapTaxedTokens>, swap_instructions: Vec<Instruction>) -> Result<()> {
        msg!("Executing swap instructions");
        // Execute the swap instructions prepared by the off-chain service
        for instruction in swap_instructions {
            anchor_lang::solana_program::program::invoke(
                &instruction,
                &ctx.accounts.to_account_infos(),
            )?;
        }

        // Reset taxed amount after successful swap
        ctx.accounts.treasury_account.taxed_amount = 0;
        Ok(())
    }

    /// Allow users to claim rewards
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        msg!("Claiming rewards");
        let user_rewards = ctx.accounts.user_rewards.pending_rewards;
        require!(user_rewards > 0, CustomError::NoRewards);

        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += user_rewards;
        ctx.accounts.user_rewards.pending_rewards = 0;

        Ok(())
    }
}

/// **Accounts**
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 8)]
    pub treasury_account: Account<'info, TreasuryAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferWithTax<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_account: Account<'info, TreasuryAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SwapTaxedTokens<'info> {
    #[account(mut)]
    pub treasury_account: Account<'info, TreasuryAccount>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_rewards: Account<'info, UserRewards>,
}

#[account]
pub struct TreasuryAccount {
    pub taxed_amount: u64,
}

#[account]
pub struct UserRewards {
    pub pending_rewards: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("No rewards to claim")]
    NoRewards,
}
