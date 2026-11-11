use crate::{
    constants::{STATE, VAULT_SEED},
    state::VaultState,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [STATE, user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [VAULT_SEED, user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let rent = Rent::get()?.minimum_balance(self.vault.data_len());

        let balance = self.vault.lamports();

        let amount = balance
            .checked_sub(rent)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        **self.vault.to_account_info().try_borrow_mut_lamports()? -= amount;

        **self.user.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}