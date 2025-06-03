use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::Hash;

#[test]
fn transaction_count_resets_on_new_block() {
    new_test_ext().execute_with(|| {
        // Verifica se o contador de transações começa em zero
        assert_eq!(ContractsPallet::transaction_count(), 0);
        
        // Avança para o próximo bloco
        run_to_block(1);
        
        // Verifica se o contador de transações foi resetado
        assert_eq!(ContractsPallet::transaction_count(), 0);
    });
}

#[test]
fn deploy_contract_works() {
    new_test_ext().execute_with(|| {
        // Código de contrato de exemplo (vazio para simplificar o teste)
        let contract_code = vec![1, 2, 3, 4];
        
        // Implanta o contrato
        assert_ok!(ContractsPallet::deploy_contract(
            RuntimeOrigin::signed(1),
            contract_code.clone(),
            10,
            Weight::from_parts(1000000, 0),
            None,
            vec![]
        ));
        
        // Verifica se o contador de transações foi incrementado
        assert_eq!(ContractsPallet::transaction_count(), 1);
    });
}

#[test]
fn call_contract_works() {
    new_test_ext().execute_with(|| {
        // Chama um contrato (simulado)
        assert_ok!(ContractsPallet::call_contract(
            RuntimeOrigin::signed(1),
            2, // endereço do contrato
            10, // valor (maior que a taxa mínima)
            Weight::from_parts(1000000, 0),
            None,
            vec![]
        ));
        
        // Verifica se o contador de transações foi incrementado
        assert_eq!(ContractsPallet::transaction_count(), 1);
    });
}

#[test]
fn fee_too_low_is_rejected() {
    new_test_ext().execute_with(|| {
        // Tenta chamar um contrato com taxa muito baixa
        assert_noop!(
            ContractsPallet::call_contract(
                RuntimeOrigin::signed(1),
                2, // endereço do contrato
                0, // valor (menor que a taxa mínima)
                Weight::from_parts(1000000, 0),
                None,
                vec![]
            ),
            Error::<Test>::FeeTooLow
        );
        
        // Verifica se o contador de transações não foi incrementado
        assert_eq!(ContractsPallet::transaction_count(), 0);
    });
}

#[test]
fn transaction_limit_is_enforced() {
    new_test_ext().execute_with(|| {
        // Define o contador de transações para o limite máximo
        ContractsPallet::TransactionCount::<Test>::put(MaxTransactionsPerBlock::get());
        
        // Tenta implantar um contrato
        assert_noop!(
            ContractsPallet::deploy_contract(
                RuntimeOrigin::signed(1),
                vec![1, 2, 3, 4],
                10,
                Weight::from_parts(1000000, 0),
                None,
                vec![]
            ),
            Error::<Test>::TransactionLimitExceeded
        );
        
        // Tenta chamar um contrato
        assert_noop!(
            ContractsPallet::call_contract(
                RuntimeOrigin::signed(1),
                2,
                10,
                Weight::from_parts(1000000, 0),
                None,
                vec![]
            ),
            Error::<Test>::TransactionLimitExceeded
        );
    });
}

#[test]
fn multiple_transactions_in_same_block() {
    new_test_ext().execute_with(|| {
        // Primeira transação
        assert_ok!(ContractsPallet::call_contract(
            RuntimeOrigin::signed(1),
            2,
            10,
            Weight::from_parts(1000000, 0),
            None,
            vec![]
        ));
        
        // Verifica se o contador de transações foi incrementado
        assert_eq!(ContractsPallet::transaction_count(), 1);
        
        // Segunda transação
        assert_ok!(ContractsPallet::call_contract(
            RuntimeOrigin::signed(1),
            2,
            10,
            Weight::from_parts(1000000, 0),
            None,
            vec![]
        ));
        
        // Verifica se o contador de transações foi incrementado novamente
        assert_eq!(ContractsPallet::transaction_count(), 2);
    });
}
