//! # CivicChain Contracts Pallet
//!
//! Este pallet estende o pallet de contratos padrão do Substrate para fornecer
//! funcionalidades específicas para a CivicChain, como taxas de transação próximas de zero
//! e limites para evitar spam na rede.
//!
//! ## Visão Geral
//!
//! O pallet de contratos da CivicChain fornece:
//! * Suporte a contratos inteligentes com ink! (Rust para WebAssembly)
//! * Taxas de transação próximas de zero (micro-fees)
//! * Mecanismos para evitar spam na rede

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, Get, Randomness},
        weights::Weight,
    };
    use frame_system::pallet_prelude::*;
    use pallet_contracts::{chain_extension::Environment, Config as ContractsConfig};
    use sp_runtime::traits::Hash;
    use sp_std::prelude::*;

    // Definição do tipo de moeda para o pallet
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // Definição do pallet
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // Configuração do pallet
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_contracts::Config {
        /// O tipo de evento.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A moeda em que as taxas são pagas.
        type Currency: Currency<Self::AccountId>;

        /// Taxa mínima para transações de contratos (micro-fee).
        #[pallet::constant]
        type MinContractFee: Get<BalanceOf<Self>>;

        /// Limite de transações por bloco para evitar spam.
        #[pallet::constant]
        type MaxTransactionsPerBlock: Get<u32>;
    }

    // Eventos emitidos pelo pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Um contrato foi implantado.
        /// [criador, endereço_contrato, código_hash]
        ContractDeployed(T::AccountId, T::AccountId, T::Hash),

        /// Uma chamada de contrato foi executada.
        /// [chamador, endereço_contrato, valor]
        ContractCalled(T::AccountId, T::AccountId, BalanceOf<T>),

        /// Uma transação foi rejeitada devido ao limite de transações por bloco.
        /// [chamador, endereço_contrato]
        TransactionRejected(T::AccountId, T::AccountId),
    }

    // Erros que podem ocorrer no pallet
    #[pallet::error]
    pub enum Error<T> {
        /// Taxa muito baixa.
        FeeTooLow,
        /// Limite de transações por bloco excedido.
        TransactionLimitExceeded,
    }

    // Armazenamento para o contador de transações no bloco atual
    #[pallet::storage]
    #[pallet::getter(fn transaction_count)]
    pub type TransactionCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    // Hooks do pallet
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_block_number: BlockNumberFor<T>) -> Weight {
            // Reseta o contador de transações no início de cada bloco
            <TransactionCount<T>>::put(0);
            Weight::zero()
        }
    }

    // Chamadas externas do pallet
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Implanta um novo contrato.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn deploy_contract(
            origin: OriginFor<T>,
            code: Vec<u8>,
            value: BalanceOf<T>,
            gas_limit: Weight,
            storage_deposit_limit: Option<<T as pallet_contracts::Config>::BalanceOf<T>>,
            data: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;

            // Verifica se o limite de transações por bloco foi excedido
            let tx_count = Self::transaction_count();
            ensure!(
                tx_count < T::MaxTransactionsPerBlock::get(),
                Error::<T>::TransactionLimitExceeded
            );

            // Incrementa o contador de transações
            <TransactionCount<T>>::put(tx_count + 1);

            // Calcula o endereço do contrato
            let salt = (
                frame_system::Pallet::<T>::block_number(),
                frame_system::Pallet::<T>::extrinsic_index(),
                tx_count,
                who.clone(),
            );
            let code_hash = T::Hashing::hash(&code);
            let contract_addr = T::Hashing::hash_of(&(code_hash, salt))
                .as_ref()
                .to_vec();
            let contract_addr = T::AccountId::decode(&mut &contract_addr[..])
                .unwrap_or_else(|_| Default::default());

            // Chama o pallet de contratos para implantar o contrato
            pallet_contracts::Pallet::<T>::instantiate_with_code(
                origin,
                value,
                gas_limit,
                storage_deposit_limit,
                code,
                data,
                vec![],
            )?;

            // Emite evento de implantação de contrato
            Self::deposit_event(Event::ContractDeployed(who, contract_addr, code_hash));

            Ok(())
        }

        /// Chama um contrato existente.
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn call_contract(
            origin: OriginFor<T>,
            dest: T::AccountId,
            value: BalanceOf<T>,
            gas_limit: Weight,
            storage_deposit_limit: Option<<T as pallet_contracts::Config>::BalanceOf<T>>,
            data: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;

            // Verifica se o limite de transações por bloco foi excedido
            let tx_count = Self::transaction_count();
            ensure!(
                tx_count < T::MaxTransactionsPerBlock::get(),
                Error::<T>::TransactionLimitExceeded
            );

            // Incrementa o contador de transações
            <TransactionCount<T>>::put(tx_count + 1);

            // Verifica se a taxa é suficiente
            ensure!(
                value >= T::MinContractFee::get(),
                Error::<T>::FeeTooLow
            );

            // Chama o pallet de contratos para executar a chamada
            pallet_contracts::Pallet::<T>::call(
                origin,
                dest.clone(),
                value,
                gas_limit,
                storage_deposit_limit,
                data,
            )?;

            // Emite evento de chamada de contrato
            Self::deposit_event(Event::ContractCalled(who, dest, value));

            Ok(())
        }
    }
}
