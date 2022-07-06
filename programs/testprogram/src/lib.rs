use anchor_lang::prelude::*;

declare_id!("AxYLd6wScMy7bu3qMzEQpyGeNeqBe2sEtz6wkZxFtwZ1");

#[program]
pub mod testprogram {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = String::from("Empty");
        Ok(())
    }

    pub fn write(ctx: Context<Update>, new_string: String) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = new_string;
        Ok(())
    }

    #[derive(Accounts)]
    // what do we need to pass in to initialize? Context for the initialize function
    pub struct Initialize<'info> {
        #[account(init, payer = user, space = 100)]
        pub my_account: Account<'info, MyAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    // what do we need to pass in to write? Context for the write function
    pub struct Update<'info> {
        #[account(mut)]
        pub my_account: Account<'info, MyAccount>,
    }

    #[account]
    // this defines the contents of the account
    pub struct MyAccount {
        pub data: String,
    }
}
