use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_core::{H256, U256};
use sp_runtime::traits::{BlakeTwo256, Hash};

#[test]
fn initial_state_is_correct() {
    new_test_ext().execute_with(|| {
        // Verifica se a recompensa inicial está correta
        assert_eq!(PowPallet::current_block_reward(), 60);
        
        // Verifica se o suprimento total inicial é zero
        assert_eq!(PowPallet::total_supply(), 0);
        
        // Verifica se a dificuldade inicial está correta
        assert_eq!(PowPallet::current_difficulty(), U256::from(1_000_000));
        
        // Verifica se o último bloco de halving é zero
        assert_eq!(PowPallet::last_halving_block(), 0);
    });
}

#[test]
fn halving_works() {
    new_test_ext().execute_with(|| {
        // Calcula o número de blocos para o halving
        let blocks_per_halving = BlocksPerYear::get() * HalvingYears::get();
        
        // Avança para o bloco de halving
        run_to_block(blocks_per_halving as u64);
        
        // Verifica se a recompensa foi reduzida pela metade
        assert_eq!(PowPallet::current_block_reward(), 30);
        
        // Verifica se o último bloco de halving foi atualizado
        assert_eq!(PowPallet::last_halving_block(), blocks_per_halving as u64);
        
        // Avança para o próximo halving
        run_to_block(blocks_per_halving as u64 * 2);
        
        // Verifica se a recompensa foi reduzida novamente
        assert_eq!(PowPallet::current_block_reward(), 15);
    });
}

#[test]
fn max_supply_is_respected() {
    new_test_ext().execute_with(|| {
        // Define um suprimento total próximo do máximo
        let almost_max = MaxSupply::get() - 30;
        PowPallet::TotalSupply::<Test>::put(almost_max);
        
        // Simula uma solução de PoW válida
        let pre_hash = H256::zero();
        let nonce = vec![1, 2, 3, 4];
        let solution = BlakeTwo256::hash_of(&[&pre_hash.as_bytes()[..], &nonce[..]].concat());
        
        // Submete a solução
        assert_ok!(PowPallet::submit_pow_solution(
            RuntimeOrigin::signed(1),
            nonce,
            solution,
            U256::from(1_000_000)
        ));
        
        // Verifica se a recompensa foi limitada ao restante do suprimento máximo
        assert_eq!(PowPallet::total_supply(), MaxSupply::get());
        
        // Tenta submeter outra solução
        let nonce = vec![5, 6, 7, 8];
        let solution = BlakeTwo256::hash_of(&[&pre_hash.as_bytes()[..], &nonce[..]].concat());
        
        // Submete a solução
        assert_ok!(PowPallet::submit_pow_solution(
            RuntimeOrigin::signed(1),
            nonce,
            solution,
            U256::from(1_000_000)
        ));
        
        // Verifica se o suprimento total não excedeu o máximo
        assert_eq!(PowPallet::total_supply(), MaxSupply::get());
    });
}

#[test]
fn difficulty_adjustment_works() {
    new_test_ext().execute_with(|| {
        // Avança para o bloco de ajuste de dificuldade
        run_to_block(2016);
        
        // Verifica se a dificuldade foi ajustada
        assert_eq!(
            PowPallet::current_difficulty(),
            U256::from(1_000_000).saturating_mul(U256::from(105)) / U256::from(100)
        );
    });
}

#[test]
fn reward_payment_works() {
    new_test_ext().execute_with(|| {
        // Verifica o saldo inicial do minerador
        assert_eq!(Balances::free_balance(1), 10);
        
        // Simula uma solução de PoW válida
        let pre_hash = H256::zero();
        let nonce = vec![1, 2, 3, 4];
        let solution = BlakeTwo256::hash_of(&[&pre_hash.as_bytes()[..], &nonce[..]].concat());
        
        // Submete a solução
        assert_ok!(PowPallet::submit_pow_solution(
            RuntimeOrigin::signed(1),
            nonce,
            solution,
            U256::from(1_000_000)
        ));
        
        // Verifica se a recompensa foi paga
        assert_eq!(Balances::free_balance(1), 10 + 60);
        
        // Verifica se o suprimento total foi atualizado
        assert_eq!(PowPallet::total_supply(), 60);
    });
}

#[test]
fn invalid_pow_solution_is_rejected() {
    new_test_ext().execute_with(|| {
        // Tenta submeter uma solução com dificuldade muito baixa
        assert_noop!(
            PowPallet::submit_pow_solution(
                RuntimeOrigin::signed(1),
                vec![1, 2, 3, 4],
                H256::zero(),
                U256::from(100) // Dificuldade muito baixa
            ),
            Error::<Test>::DifficultyTooLow
        );
    });
}
