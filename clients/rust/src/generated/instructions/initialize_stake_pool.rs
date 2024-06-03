//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

#[cfg(feature = "anchor")]
use anchor_lang::prelude::{AnchorDeserialize, AnchorSerialize};
#[cfg(not(feature = "anchor"))]
use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct InitializeStakePool {
    /// SPL Token Mint of the underlying token to be deposited for staking
    pub base_mint: solana_program::pubkey::Pubkey,

    pub x_mint: solana_program::pubkey::Pubkey,

    pub metadata: solana_program::pubkey::Pubkey,

    pub stake_pool: solana_program::pubkey::Pubkey,

    pub vault: solana_program::pubkey::Pubkey,

    pub payer: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub token_metadata_program: solana_program::pubkey::Pubkey,

    pub associated_token_program: solana_program::pubkey::Pubkey,
}

impl InitializeStakePool {
    pub fn instruction(
        &self,
        args: InitializeStakePoolInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeStakePoolInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.base_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.x_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.metadata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake_pool,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_metadata_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = InitializeStakePoolInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::SOLD_STAKING_ID,
            accounts,
            data,
        }
    }
}

#[cfg_attr(not(feature = "anchor"), derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "anchor", derive(AnchorSerialize, AnchorDeserialize))]
pub struct InitializeStakePoolInstructionData {
    discriminator: [u8; 8],
}

impl InitializeStakePoolInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [48, 189, 243, 73, 19, 67, 36, 83],
        }
    }
}

#[cfg_attr(not(feature = "anchor"), derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "anchor", derive(AnchorSerialize, AnchorDeserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeStakePoolInstructionArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub initial_exchange_rate: u64,
}

/// Instruction builder for `InitializeStakePool`.
///
/// ### Accounts:
///
///   0. `[]` base_mint
///   1. `[writable]` x_mint
///   2. `[writable]` metadata
///   3. `[writable]` stake_pool
///   4. `[writable]` vault
///   5. `[writable, signer]` payer
///   6. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   8. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   9. `[optional]` token_metadata_program (default to `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`)
///   10. `[]` associated_token_program
#[derive(Default)]
pub struct InitializeStakePoolBuilder {
    base_mint: Option<solana_program::pubkey::Pubkey>,
    x_mint: Option<solana_program::pubkey::Pubkey>,
    metadata: Option<solana_program::pubkey::Pubkey>,
    stake_pool: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    token_metadata_program: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    decimals: Option<u8>,
    initial_exchange_rate: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeStakePoolBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// SPL Token Mint of the underlying token to be deposited for staking
    #[inline(always)]
    pub fn base_mint(&mut self, base_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.base_mint = Some(base_mint);
        self
    }
    #[inline(always)]
    pub fn x_mint(&mut self, x_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.x_mint = Some(x_mint);
        self
    }
    #[inline(always)]
    pub fn metadata(&mut self, metadata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    #[inline(always)]
    pub fn stake_pool(&mut self, stake_pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_pool = Some(stake_pool);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s']`
    #[inline(always)]
    pub fn token_metadata_program(
        &mut self,
        token_metadata_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_metadata_program = Some(token_metadata_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_token_program = Some(associated_token_program);
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.symbol = Some(symbol);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.uri = Some(uri);
        self
    }
    #[inline(always)]
    pub fn decimals(&mut self, decimals: u8) -> &mut Self {
        self.decimals = Some(decimals);
        self
    }
    #[inline(always)]
    pub fn initial_exchange_rate(&mut self, initial_exchange_rate: u64) -> &mut Self {
        self.initial_exchange_rate = Some(initial_exchange_rate);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts =
            InitializeStakePool {
                base_mint: self.base_mint.expect("base_mint is not set"),
                x_mint: self.x_mint.expect("x_mint is not set"),
                metadata: self.metadata.expect("metadata is not set"),
                stake_pool: self.stake_pool.expect("stake_pool is not set"),
                vault: self.vault.expect("vault is not set"),
                payer: self.payer.expect("payer is not set"),
                rent: self.rent.unwrap_or(solana_program::pubkey!(
                    "SysvarRent111111111111111111111111111111111"
                )),
                system_program: self
                    .system_program
                    .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                )),
                token_metadata_program: self.token_metadata_program.unwrap_or(
                    solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                ),
                associated_token_program: self
                    .associated_token_program
                    .expect("associated_token_program is not set"),
            };
        let args = InitializeStakePoolInstructionArgs {
            name: self.name.clone().expect("name is not set"),
            symbol: self.symbol.clone().expect("symbol is not set"),
            uri: self.uri.clone().expect("uri is not set"),
            decimals: self.decimals.clone().expect("decimals is not set"),
            initial_exchange_rate: self
                .initial_exchange_rate
                .clone()
                .expect("initial_exchange_rate is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_stake_pool` CPI accounts.
pub struct InitializeStakePoolCpiAccounts<'a, 'b> {
    /// SPL Token Mint of the underlying token to be deposited for staking
    pub base_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub x_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_stake_pool` CPI instruction.
pub struct InitializeStakePoolCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL Token Mint of the underlying token to be deposited for staking
    pub base_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub x_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,

    pub stake_pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeStakePoolInstructionArgs,
}

impl<'a, 'b> InitializeStakePoolCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeStakePoolCpiAccounts<'a, 'b>,
        args: InitializeStakePoolInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            base_mint: accounts.base_mint,
            x_mint: accounts.x_mint,
            metadata: accounts.metadata,
            stake_pool: accounts.stake_pool,
            vault: accounts.vault,
            payer: accounts.payer,
            rent: accounts.rent,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            token_metadata_program: accounts.token_metadata_program,
            associated_token_program: accounts.associated_token_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.base_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.x_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.metadata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake_pool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_metadata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = InitializeStakePoolInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SOLD_STAKING_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(11 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.base_mint.clone());
        account_infos.push(self.x_mint.clone());
        account_infos.push(self.metadata.clone());
        account_infos.push(self.stake_pool.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.token_metadata_program.clone());
        account_infos.push(self.associated_token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `InitializeStakePool` via CPI.
///
/// ### Accounts:
///
///   0. `[]` base_mint
///   1. `[writable]` x_mint
///   2. `[writable]` metadata
///   3. `[writable]` stake_pool
///   4. `[writable]` vault
///   5. `[writable, signer]` payer
///   6. `[]` rent
///   7. `[]` system_program
///   8. `[]` token_program
///   9. `[]` token_metadata_program
///   10. `[]` associated_token_program
pub struct InitializeStakePoolCpiBuilder<'a, 'b> {
    instruction: Box<InitializeStakePoolCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeStakePoolCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeStakePoolCpiBuilderInstruction {
            __program: program,
            base_mint: None,
            x_mint: None,
            metadata: None,
            stake_pool: None,
            vault: None,
            payer: None,
            rent: None,
            system_program: None,
            token_program: None,
            token_metadata_program: None,
            associated_token_program: None,
            name: None,
            symbol: None,
            uri: None,
            decimals: None,
            initial_exchange_rate: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// SPL Token Mint of the underlying token to be deposited for staking
    #[inline(always)]
    pub fn base_mint(
        &mut self,
        base_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.base_mint = Some(base_mint);
        self
    }
    #[inline(always)]
    pub fn x_mint(
        &mut self,
        x_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.x_mint = Some(x_mint);
        self
    }
    #[inline(always)]
    pub fn metadata(
        &mut self,
        metadata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata = Some(metadata);
        self
    }
    #[inline(always)]
    pub fn stake_pool(
        &mut self,
        stake_pool: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_pool = Some(stake_pool);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn token_metadata_program(
        &mut self,
        token_metadata_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_metadata_program = Some(token_metadata_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.associated_token_program = Some(associated_token_program);
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.instruction.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.instruction.symbol = Some(symbol);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.instruction.uri = Some(uri);
        self
    }
    #[inline(always)]
    pub fn decimals(&mut self, decimals: u8) -> &mut Self {
        self.instruction.decimals = Some(decimals);
        self
    }
    #[inline(always)]
    pub fn initial_exchange_rate(&mut self, initial_exchange_rate: u64) -> &mut Self {
        self.instruction.initial_exchange_rate = Some(initial_exchange_rate);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = InitializeStakePoolInstructionArgs {
            name: self.instruction.name.clone().expect("name is not set"),
            symbol: self.instruction.symbol.clone().expect("symbol is not set"),
            uri: self.instruction.uri.clone().expect("uri is not set"),
            decimals: self
                .instruction
                .decimals
                .clone()
                .expect("decimals is not set"),
            initial_exchange_rate: self
                .instruction
                .initial_exchange_rate
                .clone()
                .expect("initial_exchange_rate is not set"),
        };
        let instruction = InitializeStakePoolCpi {
            __program: self.instruction.__program,

            base_mint: self.instruction.base_mint.expect("base_mint is not set"),

            x_mint: self.instruction.x_mint.expect("x_mint is not set"),

            metadata: self.instruction.metadata.expect("metadata is not set"),

            stake_pool: self.instruction.stake_pool.expect("stake_pool is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            token_metadata_program: self
                .instruction
                .token_metadata_program
                .expect("token_metadata_program is not set"),

            associated_token_program: self
                .instruction
                .associated_token_program
                .expect("associated_token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct InitializeStakePoolCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    base_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    x_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    metadata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_metadata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    decimals: Option<u8>,
    initial_exchange_rate: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
