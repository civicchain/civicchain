#![cfg_attr(not(feature = "std"), no_std, no_main)]

//! # Contrato SimpleStorage
//!
//! Este é um contrato simples de armazenamento para a CivicChain.
//! Ele demonstra como armazenar e recuperar um valor numérico.
//!
//! ## Visão Geral
//!
//! O contrato implementa:
//! * Armazenamento de um valor numérico
//! * Recuperação do valor armazenado
//! * Verificação de assinatura e saldo
//! * Função de pausa de emergência
//! * Proteção contra overflow

#[ink::contract]
mod simple_storage {
    use ink::storage::Mapping;

    /// Define os erros que podem ocorrer no contrato
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Valor excede o limite
        ValueTooLarge,
        /// Chamador não é o proprietário
        NotOwner,
        /// Contrato está pausado
        ContractPaused,
        /// Saldo insuficiente
        InsufficientBalance,
    }

    /// Tipo de resultado para as operações do contrato
    pub type Result<T> = core::result::Result<T, Error>;

    /// Estrutura do contrato SimpleStorage
    #[ink(storage)]
    pub struct SimpleStorage {
        /// Valor armazenado
        value: u32,
        /// Proprietário do contrato
        owner: AccountId,
        /// Estado de pausa do contrato
        paused: bool,
        /// Mapeamento de valores por conta
        values_by_account: Mapping<AccountId, u32>,
        /// Timestamp de bloqueio para operações
        time_locks: Mapping<AccountId, BlockNumber>,
    }

    /// Eventos emitidos pelo contrato
    #[ink(event)]
    pub struct ValueChanged {
        #[ink(topic)]
        by: AccountId,
        old_value: u32,
        new_value: u32,
    }

    #[ink(event)]
    pub struct ContractPaused {
        #[ink(topic)]
        by: AccountId,
    }

    #[ink(event)]
    pub struct ContractUnpaused {
        #[ink(topic)]
        by: AccountId,
    }

    #[ink(event)]
    pub struct TimeLockSet {
        #[ink(topic)]
        account: AccountId,
        unlock_at: BlockNumber,
    }

    impl SimpleStorage {
        /// Construtor que inicializa o contrato com valor zero
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            Self {
                value: 0,
                owner,
                paused: false,
                values_by_account: Mapping::default(),
                time_locks: Mapping::default(),
            }
        }

        /// Construtor que inicializa o contrato com um valor específico
        #[ink(constructor)]
        pub fn with_value(init_value: u32) -> Self {
            let owner = Self::env().caller();
            Self {
                value: init_value,
                owner,
                paused: false,
                values_by_account: Mapping::default(),
                time_locks: Mapping::default(),
            }
        }

        /// Retorna o valor armazenado
        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }

        /// Define um novo valor
        #[ink(message)]
        pub fn set(&mut self, new_value: u32) -> Result<()> {
            // Verifica se o contrato está pausado
            if self.paused {
                return Err(Error::ContractPaused);
            }

            // Verifica se o valor não é muito grande (proteção contra overflow)
            if new_value > 1_000_000 {
                return Err(Error::ValueTooLarge);
            }

            // Verifica se o chamador tem saldo suficiente
            let caller = self.env().caller();
            let caller_balance = self.env().balance_of(caller);
            if caller_balance < 1 {
                return Err(Error::InsufficientBalance);
            }

            // Verifica se o timelock expirou
            if let Some(locked_until) = self.time_locks.get(caller) {
                if self.env().block_number() < locked_until {
                    return Err(Error::ContractPaused);
                }
            }

            // Armazena o valor antigo para o evento
            let old_value = self.value;
            
            // Atualiza o valor
            self.value = new_value;
            
            // Armazena o valor para a conta do chamador
            self.values_by_account.insert(caller, &new_value);
            
            // Emite evento de valor alterado
            self.env().emit_event(ValueChanged {
                by: caller,
                old_value,
                new_value,
            });
            
            Ok(())
        }

        /// Retorna o valor armazenado para uma conta específica
        #[ink(message)]
        pub fn get_for_account(&self, account: AccountId) -> u32 {
            self.values_by_account.get(account).unwrap_or(0)
        }

        /// Pausa o contrato (apenas o proprietário)
        #[ink(message)]
        pub fn pause(&mut self) -> Result<()> {
            // Verifica se o chamador é o proprietário
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            
            self.paused = true;
            
            // Emite evento de contrato pausado
            self.env().emit_event(ContractPaused { by: caller });
            
            Ok(())
        }

        /// Despausa o contrato (apenas o proprietário)
        #[ink(message)]
        pub fn unpause(&mut self) -> Result<()> {
            // Verifica se o chamador é o proprietário
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            
            self.paused = false;
            
            // Emite evento de contrato despausado
            self.env().emit_event(ContractUnpaused { by: caller });
            
            Ok(())
        }

        /// Define um timelock para uma conta
        #[ink(message)]
        pub fn set_timelock(&mut self, account: AccountId, blocks: BlockNumber) -> Result<()> {
            // Verifica se o chamador é o proprietário
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            
            let current_block = self.env().block_number();
            let unlock_at = current_block + blocks;
            
            // Define o timelock
            self.time_locks.insert(account, &unlock_at);
            
            // Emite evento de timelock definido
            self.env().emit_event(TimeLockSet {
                account,
                unlock_at,
            });
            
            Ok(())
        }

        /// Verifica se o contrato está pausado
        #[ink(message)]
        pub fn is_paused(&self) -> bool {
            self.paused
        }

        /// Retorna o proprietário do contrato
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }

    /// Testes unitários em ambiente de contrato
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test, DefaultEnvironment};

        #[ink::test]
        fn default_works() {
            let contract = SimpleStorage::new();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = SimpleStorage::new();
            assert_eq!(contract.get(), 0);
            assert_eq!(contract.set(42), Ok(()));
            assert_eq!(contract.get(), 42);
        }

        #[ink::test]
        fn value_too_large_fails() {
            let mut contract = SimpleStorage::new();
            assert_eq!(contract.set(2_000_000), Err(Error::ValueTooLarge));
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn pause_works() {
            let mut contract = SimpleStorage::new();
            
            // Pausa o contrato
            assert_eq!(contract.pause(), Ok(()));
            assert!(contract.is_paused());
            
            // Tenta definir um valor enquanto pausado
            assert_eq!(contract.set(42), Err(Error::ContractPaused));
            
            // Despausa o contrato
            assert_eq!(contract.unpause(), Ok(()));
            assert!(!contract.is_paused());
            
            // Agora deve funcionar
            assert_eq!(contract.set(42), Ok(()));
            assert_eq!(contract.get(), 42);
        }

        #[ink::test]
        fn only_owner_can_pause() {
            let mut contract = SimpleStorage::new();
            
            // Configura uma conta diferente
            let accounts = test::default_accounts::<DefaultEnvironment>();
            
            // Tenta pausar com outra conta
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            assert_eq!(contract.pause(), Err(Error::NotOwner));
            
            // O proprietário deve conseguir pausar
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(contract.pause(), Ok(()));
        }

        #[ink::test]
        fn account_specific_values_work() {
            let mut contract = SimpleStorage::new();
            
            // Configura contas
            let accounts = test::default_accounts::<DefaultEnvironment>();
            
            // Alice define um valor
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(contract.set(42), Ok(()));
            
            // Bob define outro valor
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            assert_eq!(contract.set(84), Ok(()));
            
            // Verifica os valores específicos
            assert_eq!(contract.get_for_account(accounts.alice), 42);
            assert_eq!(contract.get_for_account(accounts.bob), 84);
            
            // O valor global é o último definido
            assert_eq!(contract.get(), 84);
        }

        #[ink::test]
        fn timelock_works() {
            let mut contract = SimpleStorage::new();
            
            // Configura contas
            let accounts = test::default_accounts::<DefaultEnvironment>();
            
            // Define um timelock para Bob
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            assert_eq!(contract.set_timelock(accounts.bob, 100), Ok(()));
            
            // Bob tenta definir um valor
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            assert_eq!(contract.set(42), Err(Error::ContractPaused));
            
            // Avança o tempo além do timelock
            test::advance_block::<DefaultEnvironment>();
            test::set_block_number::<DefaultEnvironment>(101);
            
            // Agora Bob deve conseguir definir um valor
            assert_eq!(contract.set(42), Ok(()));
            assert_eq!(contract.get(), 42);
        }
    }
}
