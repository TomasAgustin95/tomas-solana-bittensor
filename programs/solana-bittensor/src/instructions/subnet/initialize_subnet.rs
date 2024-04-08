use crate::states::*;
use anchor_lang::prelude::*;

use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn initialize_subnet(ctx: Context<InitializeSubnet>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;
    ctx.accounts.subnet_epoch.load_init()?.epoch_start_timestamp = timestamp;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnet<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(
        init,
        payer = owner,
        space = 10 * 1024,
        seeds = [b"subnet_state",owner.key().as_ref()],
        bump
    )]
    pub subnet_state: AccountLoader<'info, SubnetState>,

    #[account(
        init,
        payer = owner,
        space = 10 * 1024,
        seeds = [b"subnet_epoch",subnet_state.key().as_ref()],
        bump
    )]
    pub subnet_epoch: AccountLoader<'info, SubnetEpochState>,

    // 系统奖励代币
    #[account(
        mut,
        seeds = [b"tao", bittensor_state.key().as_ref()], 
        bump,
    )]
    pub tao_mint: Box<Account<'info, Mint>>,

    // 质押代币存储账户
    #[account(
        init,
        payer = owner,
        seeds=[b"tao_stake", subnet_state.key().as_ref()],
        bump,
        token::mint = tao_mint,
        token::authority = subnet_state
    )]
    pub tao_stake: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
