use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::instruction::Instruction;
use std::convert::Into;
use std::ops::Deref;

declare_id!("6HXUaG3yZXWhbwq4V2iW6pcpi9wVSfV6g9hRaDHF7CBJ");
// A shared wallet is just another name for a multisig wallet
#[program]
pub mod shared_wallet {
    use super::*;

    pub fn create_wallet(
        ctx: Context<CreateWallet>,
        owners: Vec<Pubkey>,
        threshold: u64,
        nonce: u8,) -> Result<()> {
        check_unique_owners(&owners)?;

        require!(threshold > 0 && threshold <= owners.len() as u64, InvalidThreshold);
        

        require!(owners.is_empty(), InvalidOwnerslen);
        let wallet = &mut ctx.accounts.shared_wallet;
        wallet.owners = owners;
        wallet.threshold_owners = threshold;
        wallet.nonce = nonce;
        wallet.owner_sequence = 0;
        Ok(())
    }
    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        pid: Pubkey,
        accs: Vec<TransactionAccount>,
        data: Vec<u8>,
    ) -> Result<()> {
        let owner_index = ctx
            .accounts
            .shared_wallet
            .owners
            .iter()
            .position(|a| a == ctx.accounts.initiator.key)
            .ok_or(ErrorCode::InvalidOwner)?;

        let mut signers = Vec::new();
        signers.resize(ctx.accounts.shared_wallet.owners.len(), false);
        signers[owner_index] = true;

        let tx = &mut ctx.accounts.transaction;
        tx.program_id = pid;
        tx.accounts = accs;
        tx.data = data;
        tx.signers = signers;
        tx.shared_wallet = ctx.accounts.shared_wallet.key();
        tx.did_execute = false;
        tx.owner_sequence = ctx.accounts.shared_wallet.owner_sequence;

        Ok(())

    }    
    pub fn approve_transaction(ctx: Context<ApproveTransaction>) -> Result<()> {
        let owner_index = ctx
            .accounts
            .shared_wallet
            .owners
            .iter()
            .position(|a| a == ctx.accounts.owner.key)
            .ok_or(ErrorCode::InvalidOwner)?;

        ctx.accounts.transaction.signers[owner_index] = true;

        Ok(())
    }
    pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
           
            if ctx.accounts.transaction.did_execute {
                return Err(ErrorCode::AlreadyExecuted.into());
            }
    
            // Checking if we have enough signers.
            let sig_count = ctx
                .accounts
                .transaction
                .signers
                .iter()
                .filter(|&did_sign| *did_sign)
                .count() as u64;
            if sig_count < ctx.accounts.shared_wallet.threshold_owners {
                 error!(ErrorCode::NotEnoughSigners);
            }
    
            let mut ix: Instruction = (*ctx.accounts.transaction).deref().into();
            ix.accounts = ix
                .accounts
                .iter()
                .map(|acc| {
                    let mut acc = acc.clone();
                    if &acc.pubkey == ctx.accounts.shared_wallet_signer.key {
                        acc.is_signer = true;
                    }
                    acc
                })
                .collect();
            let shared_wallet_key = ctx.accounts.shared_wallet.key();
            let seeds = &[shared_wallet_key.as_ref(), &[ctx.accounts.shared_wallet.nonce]];
            let signer = &[&seeds[..]];
            let accounts = ctx.remaining_accounts;
            solana_program::program::invoke_signed(&ix, accounts, signer)?;
    
            // Burn the transaction to ensure one time use.
            ctx.accounts.transaction.did_execute = true;
    
            Ok(())
        }
    
}

#[derive(Accounts)]
pub struct CreateWallet<'info> {
    #[account(zero, signer)]
    shared_wallet: Box<Account<'info, SharedWallet>>,
}
#[derive(Accounts)]
pub struct CreateTransaction<'info> {
    shared_wallet: Box<Account<'info, SharedWallet>>,
    #[account(zero, signer)]
    transaction: Box<Account<'info, Transaction>>,
    initiator: Signer<'info>,
}
#[derive(Accounts)]
pub struct ApproveTransaction<'info> {
    #[account(constraint = shared_wallet.owner_sequence == transaction.owner_sequence )]
    shared_wallet: Box<Account<'info, SharedWallet>>,
    #[account(mut, has_one = shared_wallet)]
    transaction: Box<Account<'info, Transaction>>,
    owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(constraint = shared_wallet.owner_sequence == transaction.owner_sequence)]
    shared_wallet: Box<Account<'info, SharedWallet>>,
    #[account(mut,has_one = shared_wallet)]
    transaction: Box<Account<'info, Transaction>>,
    #[account(seeds = [shared_wallet.key().as_ref()], bump = shared_wallet.nonce)]
    shared_wallet_signer: Signer<'info>,
}




#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl From<&TransactionAccount> for AccountMeta {
    fn from(account: &TransactionAccount) -> AccountMeta {
        match account.is_writable {
            false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
            true => AccountMeta::new(account.pubkey, account.is_signer),
        }
    }
}

impl From<&AccountMeta> for TransactionAccount {
    fn from(account_meta: &AccountMeta) -> TransactionAccount {
        TransactionAccount {
            pubkey: account_meta.pubkey,
            is_signer: account_meta.is_signer,
            is_writable: account_meta.is_writable,
        }
    }
}
fn check_unique_owners(owners: &[Pubkey]) -> Result<()> {
    for(i, owner) in owners.iter().enumerate() {
        require!(
            !owners.iter().skip(i + 1).any(|o| o == owner),
            UniqueOwners
        )
    }
    Ok(())
}

#[account]
pub struct SharedWallet {
    pub owners: Vec<Pubkey>,
    pub threshold_owners: u64,
    pub nonce: u8,
    pub owner_sequence: u32,
}
#[account]
pub struct Transaction {
    pub shared_wallet: Pubkey,
    pub program_id: Pubkey,
    pub accounts: Vec<TransactionAccount>,
    pub data: Vec<u8>,
    pub signers: Vec<bool>,
    pub did_execute: bool,
    pub owner_sequence: u32,
}
impl From<&Transaction> for Instruction {
    fn from(tx: &Transaction) -> Instruction {
        Instruction {
            program_id: tx.program_id,
            accounts: tx.accounts.iter().map(Into::into).collect(),
            data: tx.data.clone(),
        }
    }
}
#[error_code]
pub enum ErrorCode
{   
    InvalidOwner,
    InvalidOwnerslen,
    InvalidThreshold,
    UniqueOwners,
    AlreadyExecuted,
    NotEnoughSigners,
}