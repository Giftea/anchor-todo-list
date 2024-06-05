use anchor_lang::prelude::*;

declare_id!("BAxXaiartNHmbeDvXLaqCWGUUGvAxtv22FZwbVjGAMku");

#[program]
pub mod anchor_todo_list1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
