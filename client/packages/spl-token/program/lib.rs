// This file is autogenerated with https://github.com/nxpkg/native-to-mainstay

use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod spl_token {
    use super::*;

    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        decimals: u8,
        mint_authority: Pubkey,
        freeze_authority: COption<Pubkey>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_multisig(ctx: Context<InitializeMultisig>, m: u8) -> Result<()> {
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn approve(ctx: Context<Approve>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn revoke(ctx: Context<Revoke>) -> Result<()> {
        Ok(())
    }

    pub fn set_authority(
        ctx: Context<SetAuthority>,
        authority_type: AuthorityType,
        new_authority: COption<Pubkey>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn mint_to(ctx: Context<MintTo>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        Ok(())
    }

    pub fn freeze_account(ctx: Context<FreezeAccount>) -> Result<()> {
        Ok(())
    }

    pub fn thaw_account(ctx: Context<ThawAccount>) -> Result<()> {
        Ok(())
    }

    pub fn transfer_checked(
        ctx: Context<TransferChecked>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub fn approve_checked(ctx: Context<ApproveChecked>, amount: u64, decimals: u8) -> Result<()> {
        Ok(())
    }

    pub fn mint_to_checked(ctx: Context<MintToChecked>, amount: u64, decimals: u8) -> Result<()> {
        Ok(())
    }

    pub fn burn_checked(ctx: Context<BurnChecked>, amount: u64, decimals: u8) -> Result<()> {
        Ok(())
    }

    pub fn initialize_account2(ctx: Context<InitializeAccount2>, owner: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn sync_native(ctx: Context<SyncNative>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_account3(ctx: Context<InitializeAccount3>, owner: Pubkey) -> Result<()> {
        Ok(())
    }

    pub fn initialize_multisig2(ctx: Context<InitializeMultisig2>, m: u8) -> Result<()> {
        Ok(())
    }

    pub fn initialize_mint2(
        ctx: Context<InitializeMint2>,
        decimals: u8,
        mint_authority: Pubkey,
        freeze_authority: COption<Pubkey>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn get_account_data_size(ctx: Context<GetAccountDataSize>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_immutable_owner(ctx: Context<InitializeImmutableOwner>) -> Result<()> {
        Ok(())
    }

    pub fn amount_to_ui_amount(ctx: Context<AmountToUiAmount>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn ui_amount_to_amount(ctx: Context<UiAmountToAmount>, ui_amount: &'a str) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    mint: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(mut)]
    account: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    owner: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeMultisig<'info> {
    #[account(mut)]
    multisig: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
    // optional_signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    source: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    authority: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut)]
    source: AccountInfo<'info>,
    delegate: AccountInfo<'info>,
    owner: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Revoke<'info> {
    #[account(mut)]
    source: AccountInfo<'info>,
    owner: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut)]
    owned: AccountInfo<'info>,
    owner: Signer<'info>,
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account(mut)]
    mint: AccountInfo<'info>,
    #[account(mut)]
    account: AccountInfo<'info>,
    owner: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    account: AccountInfo<'info>,
    #[account(mut)]
    mint: AccountInfo<'info>,
    authority: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut)]
    account: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    owner: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct FreezeAccount<'info> {
    #[account(mut)]
    account: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    owner: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ThawAccount<'info> {
    #[account(mut)]
    account: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    owner: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferChecked<'info> {
    #[account(mut)]
    source: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    #[account(mut)]
    destination: AccountInfo<'info>,
    authority: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ApproveChecked<'info> {
    #[account(mut)]
    source: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    delegate: AccountInfo<'info>,
    owner: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct MintToChecked<'info> {
    #[account(mut)]
    mint: AccountInfo<'info>,
    #[account(mut)]
    account: AccountInfo<'info>,
    owner: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct BurnChecked<'info> {
    #[account(mut)]
    account: AccountInfo<'info>,
    #[account(mut)]
    mint: AccountInfo<'info>,
    authority: Signer<'info>,
    // optional_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeAccount2<'info> {
    #[account(mut)]
    account: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SyncNative<'info> {
    #[account(mut)]
    account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeAccount3<'info> {
    #[account(mut)]
    account: AccountInfo<'info>,
    mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMultisig2<'info> {
    #[account(mut)]
    multisig: AccountInfo<'info>,
    signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMint2<'info> {
    #[account(mut)]
    mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetAccountDataSize<'info> {
    mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeImmutableOwner<'info> {
    #[account(mut)]
    account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AmountToUiAmount<'info> {
    mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UiAmountToAmount<'info> {
    mint: AccountInfo<'info>,
}

#[account]
pub struct Mint {
    /// Optional authority used to mint new tokens. The mint authority may only be provided during
    /// mint creation. If no mint authority is present then the mint has a fixed supply and no
    /// further tokens may be minted.
    pub mint_authority: COption<Pubkey>,
    /// Total supply of tokens.
    pub supply: u64,
    /// Number of base 10 digits to the right of the decimal place.
    pub decimals: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Optional authority to freeze token accounts.
    pub freeze_authority: COption<Pubkey>,
}

#[account]
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u64,
    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate
    pub delegate: COption<Pubkey>,
    /// The account's state
    pub state: AccountState,
    /// If is_native.is_some, this is a native token, and the value logs the rent-exempt reserve. An
    /// Account is required to be rent-exempt, so the value is used by the Processor to ensure that
    /// wrapped SOL accounts do not drop below this threshold.
    pub is_native: COption<u64>,
    /// The amount delegated
    pub delegated_amount: u64,
    /// Optional authority to close the account.
    pub close_authority: COption<Pubkey>,
}

#[account]
pub struct Multisig {
    /// Number of signers required
    pub m: u8,
    /// Number of valid signers
    pub n: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Signer public keys
    pub signers: [Pubkey; 11],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AccountState {
    /// Account is not yet initialized
    Uninitialized,
    /// Account is initialized; the account owner and/or delegate may perform permitted operations
    /// on this account
    Initialized,
    /// Account has been frozen by the mint freeze authority. Neither the account owner nor
    /// the delegate are able to perform operations on this account.
    Frozen,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AuthorityType {
    /// Authority to mint new tokens
    MintTokens,
    /// Authority to freeze any account associated with the Mint
    FreezeAccount,
    /// Owner of a given token account
    AccountOwner,
    /// Authority to close a token account
    CloseAccount,
}

#[error_code]
pub enum TokenError {
    // 0
    /// Lamport balance below rent-exempt threshold.
    #[msg("Lamport balance below rent-exempt threshold")]
    NotRentExempt,
    /// Insufficient funds for the operation requested.
    #[msg("Insufficient funds")]
    InsufficientFunds,
    /// Invalid Mint.
    #[msg("Invalid Mint")]
    InvalidMint,
    /// Account not associated with this Mint.
    #[msg("Account not associated with this Mint")]
    MintMismatch,
    /// Owner does not match.
    #[msg("Owner does not match")]
    OwnerMismatch,

    // 5
    /// This token's supply is fixed and new tokens cannot be minted.
    #[msg("Fixed supply")]
    FixedSupply,
    /// The account cannot be initialized because it is already being used.
    #[msg("Already in use")]
    AlreadyInUse,
    /// Invalid number of provided signers.
    #[msg("Invalid number of provided signers")]
    InvalidNumberOfProvidedSigners,
    /// Invalid number of required signers.
    #[msg("Invalid number of required signers")]
    InvalidNumberOfRequiredSigners,
    /// State is uninitialized.
    #[msg("State is unititialized")]
    UninitializedState,

    // 10
    /// Instruction does not support native tokens
    #[msg("Instruction does not support native tokens")]
    NativeNotSupported,
    /// Non-native account can only be closed if its balance is zero
    #[msg("Non-native account can only be closed if its balance is zero")]
    NonNativeHasBalance,
    /// Invalid instruction
    #[msg("Invalid instruction")]
    InvalidInstruction,
    /// State is invalid for requested operation.
    #[msg("State is invalid for requested operation")]
    InvalidState,
    /// Operation overflowed
    #[msg("Operation overflowed")]
    Overflow,

    // 15
    /// Account does not support specified authority type.
    #[msg("Account does not support specified authority type")]
    AuthorityTypeNotSupported,
    /// This token mint cannot freeze accounts.
    #[msg("This token mint cannot freeze accounts")]
    MintCannotFreeze,
    /// Account is frozen; all account operations will fail
    #[msg("Account is frozen")]
    AccountFrozen,
    /// Mint decimals mismatch between the client and mint
    #[msg("The provided decimals value different from the Mint decimals")]
    MintDecimalsMismatch,
    /// Instruction does not support non-native tokens
    #[msg("Instruction does not support non-native tokens")]
    NonNativeNotSupported,
}