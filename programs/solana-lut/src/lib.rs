use anchor_lang::prelude::*;

declare_id!("9Z6ES14c7GfmTQt2DwQ2tbcRG8DYDPXU749ZuUBtDQX5");

#[program]
pub mod solana_lut {
    use super::*;

    pub fn init_pdas(ctx: Context<InitPdas>) -> Result<()> {
        ctx.accounts.pda_1.value = 1;
        ctx.accounts.pda_2.value = 2;
        ctx.accounts.pda_3.value = 3;
        ctx.accounts.pda_4.value = 4;
        ctx.accounts.pda_5.value = 5;
        ctx.accounts.pda_6.value = 6;
        ctx.accounts.pda_7.value = 7;
        ctx.accounts.pda_8.value = 8;
        ctx.accounts.pda_9.value = 9;
        ctx.accounts.pda_10.value = 10;
        ctx.accounts.pda_11.value = 11;
        ctx.accounts.pda_12.value = 12;
        ctx.accounts.pda_13.value = 13;
        ctx.accounts.pda_14.value = 14;
        ctx.accounts.pda_15.value = 15;
        ctx.accounts.pda_16.value = 16;
        ctx.accounts.pda_17.value = 17;
        ctx.accounts.pda_18.value = 18;
        ctx.accounts.pda_19.value = 19;
        ctx.accounts.pda_20.value = 20;
        ctx.accounts.pda_21.value = 21;
        ctx.accounts.pda_22.value = 22;
        ctx.accounts.pda_23.value = 23;
        ctx.accounts.pda_24.value = 24;
        ctx.accounts.pda_25.value = 25;
        ctx.accounts.pda_26.value = 26;
        ctx.accounts.pda_27.value = 27;
        ctx.accounts.pda_28.value = 28;
        ctx.accounts.pda_29.value = 29;
        ctx.accounts.pda_30.value = 30;
        ctx.accounts.pda_31.value = 31;
        ctx.accounts.pda_32.value = 32;
        ctx.accounts.pda_33.value = 33;
        ctx.accounts.pda_34.value = 34;
        ctx.accounts.pda_35.value = 35;
        ctx.accounts.pda_36.value = 36;
        ctx.accounts.pda_37.value = 37;
        ctx.accounts.pda_38.value = 38;
        ctx.accounts.pda_39.value = 39;
        ctx.accounts.pda_40.value = 40;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPdas<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
    #[account(init, seeds=[b"1".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_1: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"2".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_2: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"3".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_3: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"4".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_4: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"5".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_5: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"6".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_6: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"7".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_7: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"8".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_8: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"9".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_9: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"10".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_10: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"11".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_11: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"12".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_12: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"13".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_13: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"14".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_14: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"15".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_15: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"16".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_16: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"17".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_17: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"18".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_18: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"19".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_19: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"20".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_20: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"21".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_21: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"22".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_22: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"23".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_23: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"24".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_24: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"25".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_25: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"26".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_26: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"27".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_27: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"28".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_28: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"29".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_29: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"30".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_30: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"31".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_31: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"32".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_32: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"33".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_33: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"34".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_34: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"35".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_35: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"36".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_36: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"37".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_37: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"38".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_38: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"39".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_39: Box<Account<'info, Pda>>,
    
    #[account(init, seeds=[b"40".as_ref()], bump, payer=signer, space=8+1)]
    pub pda_40: Box<Account<'info, Pda>>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pda {
    value: u8,
}
