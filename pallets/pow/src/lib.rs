//! # CivicChain PoW Pallet
//!
//! This pallet implements the Proof of Work (PoW) consensus algorithm using Yespower-R16,
//! a lightweight mining algorithm that allows mining on common CPUs, including mobile phones.
//! It also implements GHOST (Greedy Heaviest Observed Subtree), Proof-of-History (PoH),
//! ghost confirmations, and an on-chain system for halving decisions and upgrades.
//!
//! ## Overview
//!
//! The PoW pallet provides:
//! * Implementation of the Yespower-R16 algorithm for mining
//! * Block reward control (25 CVX initially)
//! * Halving mechanism every 5 years, governed by on-chain voting
//! * Maximum supply limit of 29 million coins
//! * GHOST to improve security with low hashrate
//! * Proof-of-History for ordering blocks before consensus
//! * Ghost confirmations to reward orphan block validation
//! * On-chain governance system for halving decisions and upgrades
//! * Penalties for malicious behavior
//! * Light client support with FlyClient protocol

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
        traits::{Currency, ExistenceRequirement, Get, Imbalance, OnUnbalanced, ReservableCurrency},
        weights::Weight,
    };
    use frame_system::pallet_prelude::*;
    use sp_consensus_pow::{Seal as PowSeal, TotalPower};
    use sp_core::{H256, U256};
    use sp_runtime::{
        traits::{AccountIdConversion, BlakeTwo256, Hash, SaturatedConversion, Zero},
        Perbill,
    };
    use sp_std::prelude::*;
    use merkle_light::merkle::MerkleTree;
    use sha2::{Digest, Sha256};
    use yesha256::yespower_r16;

    // Currency type definition for the pallet
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::NegativeImbalance;

    // Structure to store block information
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct BlockInfo<BlockNumber, Hash, AccountId> {
        pub number: BlockNumber,
        pub hash: Hash,
        pub parent_hash: Hash,
        pub timestamp: u64,
        pub author: AccountId,
        pub difficulty: U256,
        pub total_difficulty: U256,
        pub poh_hash: H256,
    }

    // Structure to store governance proposal information
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Proposal<BlockNumber, AccountId, Balance> {
        pub id: u32,
        pub proposer: AccountId,
        pub description: Vec<u8>,
        pub proposed_value: u128,
        pub voting_ends_at: BlockNumber,
        pub votes_for: Balance,
        pub votes_against: Balance,
        pub status: ProposalStatus,
    }

    // Proposal status
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum ProposalStatus {
        Active,
        Approved,
        Rejected,
        Executed,
    }

    // Proposal type
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum ProposalType {
        BlockReward,
        HalvingPeriod,
        DifficultyAdjustment,
        ProtocolUpgrade,
    }

    // Structure to store vote information
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct Vote<AccountId, Balance> {
        pub voter: AccountId,
        pub proposal_id: u32,
        pub in_favor: bool,
        pub stake: Balance,
        pub delegated_to: Option<AccountId>,
        pub weight: Perbill,
    }

    // Structure to store verified expert information
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct VerifiedExpert<AccountId> {
        pub account_id: AccountId,
        pub expertise: Vec<u8>,
        pub accuracy: Perbill,
        pub total_votes: u32,
        pub correct_votes: u32,
    }

    // Structure to store orphan block information
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct OrphanBlock<BlockNumber, Hash, AccountId> {
        pub info: BlockInfo<BlockNumber, Hash, AccountId>,
        pub validators: Vec<AccountId>,
        pub is_rewarded: bool,
    }

    // Pallet definition
    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // Pallet configuration
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency in which rewards are paid.
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Handler for mining rewards.
        type RewardHandler: OnUnbalanced<NegativeImbalanceOf<Self>>;

        /// Base reward per block (25 CVX initially).
        #[pallet::constant]
        type BlockReward: Get<BalanceOf<Self>>;

        /// Number of blocks per year.
        #[pallet::constant]
        type BlocksPerYear: Get<u32>;

        /// Number of years for halving (5 years).
        #[pallet::constant]
        type HalvingYears: Get<u32>;

        /// Maximum coin supply (29 million).
        #[pallet::constant]
        type MaxSupply: Get<BalanceOf<Self>>;
    }

    // Events emitted by the pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A reward was paid to the miner.
        /// [miner, amount, current_block]
        RewardPaid(T::AccountId, BalanceOf<T>, BlockNumberFor<T>),

        /// A halving occurred in the reward.
        /// [current_block, new_reward]
        HalvingOccurred(BlockNumberFor<T>, BalanceOf<T>),

        /// The maximum supply was reached.
        /// [current_block, total_supply]
        MaxSupplyReached(BlockNumberFor<T>, BalanceOf<T>),

        /// A proposal was created.
        /// [proposal_id, proposer, proposal_type]
        ProposalCreated(u32, T::AccountId, ProposalType),

        /// A vote was registered.
        /// [proposal_id, voter, in_favor, stake]
        VoteRegistered(u32, T::AccountId, bool, BalanceOf<T>),

        /// A proposal was approved.
        /// [proposal_id, votes_for, votes_against]
        ProposalApproved(u32, BalanceOf<T>, BalanceOf<T>),

        /// A proposal was rejected.
        /// [proposal_id, votes_for, votes_against]
        ProposalRejected(u32, BalanceOf<T>, BalanceOf<T>),

        /// A proposal was executed.
        /// [proposal_id, proposal_type, new_value]
        ProposalExecuted(u32, ProposalType, u128),

        /// An expert was verified.
        /// [account, expertise]
        ExpertVerified(T::AccountId, Vec<u8>),

        /// A reward was paid for orphan block validation.
        /// [validator, amount, block_hash]
        OrphanBlockRewardPaid(T::AccountId, BalanceOf<T>, H256),

        /// A penalty was applied for malicious behavior.
        /// [account, amount, reason]
        PenaltyApplied(T::AccountId, BalanceOf<T>, Vec<u8>),

        /// A new block was added to the GHOST tree.
        /// [block_hash, total_difficulty]
        BlockAddedToGhost(H256, U256),
    }

    // Errors that can occur in the pallet
    #[pallet::error]
    pub enum Error<T> {
        /// The maximum supply was exceeded.
        MaxSupplyExceeded,
        /// PoW verification failed.
        PowVerificationFailed,
        /// Difficulty too low.
        DifficultyTooLow,
        /// Proposal not found.
        ProposalNotFound,
        /// Proposal already finalized.
        ProposalAlreadyFinalized,
        /// Voting period ended.
        VotingPeriodEnded,
        /// Insufficient stake for voting.
        InsufficientStakeForVoting,
        /// Already voted on this proposal.
        AlreadyVoted,
        /// Not a verified expert.
        NotVerifiedExpert,
        /// Orphan block not found.
        OrphanBlockNotFound,
        /// Orphan block already rewarded.
        OrphanBlockAlreadyRewarded,
        /// Proof-of-History verification failed.
        PohVerificationFailed,
        /// FlyClient verification failed.
        FlyClientVerificationFailed,
        /// GHOST tree verification failed.
        GhostVerificationFailed,
        /// Stake already locked.
        StakeAlreadyLocked,
        /// Lock period not finished.
        LockPeriodNotFinished,
    }

    // Storage for total supply issued
    #[pallet::storage]
    #[pallet::getter(fn total_supply)]
    pub type TotalSupply<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    // Storage for current block reward
    #[pallet::storage]
    #[pallet::getter(fn current_block_reward)]
    pub type CurrentBlockReward<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    // Storage for last halving block
    #[pallet::storage]
    #[pallet::getter(fn last_halving_block)]
    pub type LastHalvingBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    // Storage for current difficulty
    #[pallet::storage]
    #[pallet::getter(fn current_difficulty)]
    pub type CurrentDifficulty<T: Config> = StorageValue<_, U256, ValueQuery>;

    // Storage for last Proof-of-History hash
    #[pallet::storage]
    #[pallet::getter(fn last_poh_hash)]
    pub type LastPohHash<T: Config> = StorageValue<_, H256, ValueQuery>;

    // Storage for Proof-of-History counter
    #[pallet::storage]
    #[pallet::getter(fn poh_counter)]
    pub type PohCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

    // Storage for GHOST tree
    #[pallet::storage]
    #[pallet::getter(fn ghost_tree)]
    pub type GhostTree<T: Config> = StorageMap<_, Blake2_128Concat, H256, BlockInfo<BlockNumberFor<T>, T::Hash, T::AccountId>, OptionQuery>;

    // Storage for total difficulty of each block
    #[pallet::storage]
    #[pallet::getter(fn block_total_difficulty)]
    pub type BlockTotalDifficulty<T: Config> = StorageMap<_, Blake2_128Concat, H256, U256, ValueQuery>;

    // Storage for current best block (chain head)
    #[pallet::storage]
    #[pallet::getter(fn best_block)]
    pub type BestBlock<T: Config> = StorageValue<_, H256, ValueQuery>;

    // Storage for orphan blocks
    #[pallet::storage]
    #[pallet::getter(fn orphan_blocks)]
    pub type OrphanBlocks<T: Config> = StorageMap<_, Blake2_128Concat, H256, OrphanBlock<BlockNumberFor<T>, T::Hash, T::AccountId>, OptionQuery>;

    // Storage for governance proposals
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<_, Blake2_128Concat, u32, Proposal<BlockNumberFor<T>, T::AccountId, BalanceOf<T>>, OptionQuery>;

    // Storage for next proposal ID
    #[pallet::storage]
    #[pallet::getter(fn next_proposal_id)]
    pub type NextProposalId<T: Config> = StorageValue<_, u32, ValueQuery>;

    // Storage for votes
    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, u32, // Proposal ID
        Blake2_128Concat, T::AccountId, // Voter
        Vote<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    // Storage for verified experts
    #[pallet::storage]
    #[pallet::getter(fn verified_experts)]
    pub type VerifiedExperts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, VerifiedExpert<T::AccountId>, OptionQuery>;

    // Storage for locked stakes
    #[pallet::storage]
    #[pallet::getter(fn locked_stakes)]
    pub type LockedStakes<T: Config> = StorageMap<
        _,
        Blake2_128Concat, T::AccountId, // Account
        (BalanceOf<T>, BlockNumberFor<T>), // (Amount, Release block)
        OptionQuery,
    >;

    // Storage for Merkle tree root for FlyClient
    #[pallet::storage]
    #[pallet::getter(fn merkle_root)]
    pub type MerkleRoot<T: Config> = StorageValue<_, H256, ValueQuery>;

    // Pallet genesis
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_difficulty: U256,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                initial_difficulty: U256::from(1_000_000), // Initial difficulty
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            // Initialize block reward
            <CurrentBlockReward<T>>::put(T::BlockReward::get());
            // Initialize difficulty
            <CurrentDifficulty<T>>::put(self.initial_difficulty);
            // Initialize total supply as zero
            <TotalSupply<T>>::put(BalanceOf::<T>::zero());
            // Initialize last halving block as zero
            <LastHalvingBlock<T>>::put(BlockNumberFor::<T>::zero());
            // Initialize last PoH hash
            <LastPohHash<T>>::put(H256::zero());
            // Initialize PoH counter
            <PohCounter<T>>::put(0);
            // Initialize next proposal ID
            <NextProposalId<T>>::put(1);
            // Initialize Merkle tree root
            <MerkleRoot<T>>::put(H256::zero());
        }
    }

    // Pallet hooks
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
            // Update PoH hash
            Self::update_poh_hash();

            // Check if there are governance proposals to finalize
            Self::finalize_proposals(block_number);

            // Check if it's time for a halving (if there's no active proposal to change the period)
            if !Self::has_active_halving_proposal() {
                let blocks_per_halving = T::BlocksPerYear::get()
                    .saturating_mul(T::HalvingYears::get())
                    .saturated_into::<u32>();
                let blocks_per_halving = blocks_per_halving.saturated_into::<BlockNumberFor<T>>();

                let last_halving = Self::last_halving_block();
                if block_number.saturating_sub(last_halving) >= blocks_per_halving
                    && !Self::current_block_reward().is_zero()
                {
                    // Perform halving
                    let current_reward = Self::current_block_reward();
                    let new_reward = current_reward.saturating_div(2u32.saturated_into());
                    <CurrentBlockReward<T>>::put(new_reward);
                    <LastHalvingBlock<T>>::put(block_number);

                    // Emit halving event
                    Self::deposit_event(Event::HalvingOccurred(block_number, new_reward));
                }
            }

            // Adjust difficulty every 2016 blocks (approximately 2 weeks with 200-second blocks)
            if block_number % 2016u32.saturated_into::<BlockNumberFor<T>>() == Zero::zero() && block_number > Zero::zero() {
                Self::adjust_difficulty(block_number);
            }

            // Update Merkle tree for FlyClient every 1000 blocks
            if block_number % 1000u32.saturated_into::<BlockNumberFor<T>>() == Zero::zero() && block_number > Zero::zero() {
                Self::update_merkle_tree();
            }

            Weight::zero()
        }
    }

    // External calls for the pallet
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Submit a proof of work to receive the mining reward.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn submit_pow_solution(
            origin: OriginFor<T>,
            nonce: Vec<u8>,
            solution: H256,
            difficulty: U256,
            poh_hash: H256,
        ) -> DispatchResult {
            let miner = ensure_signed(origin)?;

            // Verify if the difficulty is valid
            ensure!(
                difficulty >= Self::current_difficulty(),
                Error::<T>::DifficultyTooLow
            );

            // Verify the Proof-of-History hash
            ensure!(
                Self::verify_poh(poh_hash),
                Error::<T>::PohVerificationFailed
            );

            // Verify the PoW solution
            let pre_hash = <frame_system::Pallet<T>>::parent_hash();
            let pre_hash_bytes = pre_hash.as_bytes();
            
            let seal = PowSeal {
                difficulty,
                work: solution,
                nonce,
            };

            ensure!(
                Self::verify_pow(pre_hash_bytes, &seal, poh_hash),
                Error::<T>::PowVerificationFailed
            );

            // Calculate the reward
            let reward = Self::calculate_reward()?;

            // Pay the reward to the miner
            if !reward.is_zero() {
                let imbalance = T::Currency::issue(reward);
                T::RewardHandler::on_unbalanced(imbalance);
                T::Currency::deposit_creating(&miner, reward);

                // Update total supply
                let new_supply = Self::total_supply().saturating_add(reward);
                <TotalSupply<T>>::put(new_supply);

                // Emit reward event
                let current_block = <frame_system::Pallet<T>>::block_number();
                Self::deposit_event(Event::RewardPaid(miner, reward, current_block));
            }

            // Update GHOST tree
            let current_block_hash = <frame_system::Pallet<T>>::block_hash(
                <frame_system::Pallet<T>>::block_number()
            );
            Self::update_ghost_tree(current_block_hash, pre_hash, difficulty, miner.clone(), poh_hash);

            Ok(())
        }

        /// Create a governance proposal.
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn create_proposal(
            origin: OriginFor<T>,
            proposal_type: ProposalType,
            description: Vec<u8>,
            proposed_value: u128,
            voting_period: BlockNumberFor<T>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;

            // Verify if the proposer has enough stake (1000 coins)
            let min_stake = 1000 * 10_u128.pow(18);
            let min_stake_balance: BalanceOf<T> = min_stake.saturated_into();
            ensure!(
                T::Currency::free_balance(&proposer) >= min_stake_balance,
                Error::<T>::InsufficientStakeForVoting
            );

            // Lock the proposer's stake
            T::Currency::reserve(&proposer, min_stake_balance)?;

            // Calculate the voting end block
            let current_block = <frame_system::Pallet<T>>::block_number();
            let voting_ends_at = current_block.saturating_add(voting_period);

            // Create the proposal
            let proposal_id = Self::next_proposal_id();
            let proposal = Proposal {
                id: proposal_id,
                proposer: proposer.clone(),
                description,
                proposed_value,
                voting_ends_at,
                votes_for: BalanceOf::<T>::zero(),
                votes_against: BalanceOf::<T>::zero(),
                status: ProposalStatus::Active,
            };

            // Store the proposal
            <Proposals<T>>::insert(proposal_id, proposal);
            <NextProposalId<T>>::put(proposal_id.saturating_add(1));

            // Register the proposer's vote (in favor)
            let vote = Vote {
                voter: proposer.clone(),
                proposal_id,
                in_favor: true,
                stake: min_stake_balance,
                delegated_to: None,
                weight: Perbill::from_percent(100),
            };
            <Votes<T>>::insert(proposal_id, proposer.clone(), vote);

            // Update the proposal's votes
            let mut proposal = Self::proposals(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
            proposal.votes_for = proposal.votes_for.saturating_add(min_stake_balance);
            <Proposals<T>>::insert(proposal_id, proposal);

            // Emit proposal creation event
            Self::deposit_event(Event::ProposalCreated(proposal_id, proposer, proposal_type));

            Ok(())
        }

        /// Vote on a governance proposal.
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn vote(
            origin: OriginFor<T>,
            proposal_id: u32,
            in_favor: bool,
            stake: BalanceOf<T>,
            delegated_to: Option<T::AccountId>,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            // Verify if the proposal exists and is active
            let proposal = Self::proposals(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
            ensure!(
                proposal.status == ProposalStatus::Active,
                Error::<T>::ProposalAlreadyFinalized
            );

            // Verify if the voting period is still active
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(
                current_block <= proposal.voting_ends_at,
                Error::<T>::VotingPeriodEnded
            );

            // Verify if the voter has enough stake
            ensure!(
                T::Currency::free_balance(&voter) >= stake,
                Error::<T>::InsufficientStakeForVoting
            );

            // Verify if the voter has already voted on this proposal
            ensure!(
                !<Votes<T>>::contains_key(proposal_id, voter.clone()),
                Error::<T>::AlreadyVoted
            );

            // Verify if the delegate is a verified expert
            let weight = if let Some(ref expert) = delegated_to {
                ensure!(
                    <VerifiedExperts<T>>::contains_key(expert),
                    Error::<T>::NotVerifiedExpert
                );
                
                // Get the expert's accuracy to calculate vote weight
                let expert_info = Self::verified_experts(expert).unwrap();
                expert_info.accuracy
            } else {
                Perbill::from_percent(100)
            };

            // Lock the voter's stake
            T::Currency::reserve(&voter, stake)?;

            // Register the vote
            let vote = Vote {
                voter: voter.clone(),
                proposal_id,
                in_favor,
                stake,
                delegated_to: delegated_to.clone(),
                weight,
            };
            <Votes<T>>::insert(proposal_id, voter.clone(), vote);

            // Update the proposal's votes
            let mut proposal = Self::proposals(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
            let weighted_stake = weight.mul_floor(stake);
            
            if in_favor {
                proposal.votes_for = proposal.votes_for.saturating_add(weighted_stake);
            } else {
                proposal.votes_against = proposal.votes_against.saturating_add(weighted_stake);
            }
            
            <Proposals<T>>::insert(proposal_id, proposal);

            // Emit vote registered event
            Self::deposit_event(Event::VoteRegistered(proposal_id, voter, in_favor, stake));

            Ok(())
        }

        /// Verify an expert.
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn verify_expert(
            origin: OriginFor<T>,
            expert: T::AccountId,
            expertise: Vec<u8>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Create verified expert record
            let expert_info = VerifiedExpert {
                account_id: expert.clone(),
                expertise: expertise.clone(),
                accuracy: Perbill::from_percent(100), // Initially 100% accuracy
                total_votes: 0,
                correct_votes: 0,
            };

            // Store the verified expert
            <VerifiedExperts<T>>::insert(expert.clone(), expert_info);

            // Emit expert verified event
            Self::deposit_event(Event::ExpertVerified(expert, expertise));

            Ok(())
        }

        /// Validate an orphan block.
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn validate_orphan_block(
            origin: OriginFor<T>,
            block_hash: H256,
        ) -> DispatchResult {
            let validator = ensure_signed(origin)?;

            // Verify if the orphan block exists
            let mut orphan_block = Self::orphan_blocks(block_hash).ok_or(Error::<T>::OrphanBlockNotFound)?;
            
            // Verify if the orphan block has already been rewarded
            ensure!(
                !orphan_block.is_rewarded,
                Error::<T>::OrphanBlockAlreadyRewarded
            );

            // Add the validator to the orphan block's validators list
            if !orphan_block.validators.contains(&validator) {
                orphan_block.validators.push(validator.clone());
                <OrphanBlocks<T>>::insert(block_hash, orphan_block.clone());
            }

            // If there are at least 3 validators, pay the reward
            if orphan_block.validators.len() >= 3 {
                // Calculate the reward (20% of the block reward)
                let block_reward = Self::current_block_reward();
                let ghost_reward = block_reward.saturating_mul(20u32.saturated_into()) / 100u32.saturated_into();
                
                // Mark the orphan block as rewarded
                orphan_block.is_rewarded = true;
                <OrphanBlocks<T>>::insert(block_hash, orphan_block.clone());
                
                // Pay the reward to each validator
                for validator in orphan_block.validators.iter() {
                    let validator_reward = ghost_reward.saturating_div(orphan_block.validators.len().saturated_into());
                    
                    if !validator_reward.is_zero() {
                        let imbalance = T::Currency::issue(validator_reward);
                        T::RewardHandler::on_unbalanced(imbalance);
                        T::Currency::deposit_creating(validator, validator_reward);
                        
                        // Update total supply
                        let new_supply = Self::total_supply().saturating_add(validator_reward);
                        <TotalSupply<T>>::put(new_supply);
                        
                        // Emit orphan block reward event
                        Self::deposit_event(Event::OrphanBlockRewardPaid(validator.clone(), validator_reward, block_hash));
                    }
                }
            }
            
            Ok(())
        }
        
        /// Apply a penalty for malicious behavior.
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn apply_penalty(
            origin: OriginFor<T>,
            account: T::AccountId,
            amount: BalanceOf<T>,
            reason: Vec<u8>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            // Apply the penalty by slashing the account
            T::Currency::slash(&account, amount);
            
            // Emit penalty applied event
            Self::deposit_event(Event::PenaltyApplied(account, amount, reason));
            
            Ok(())
        }
    }
    
    // Implementation of pallet functions
    impl<T: Config> Pallet<T> {
        // Calculate the current block reward
        fn calculate_reward() -> Result<BalanceOf<T>, DispatchError> {
            let current_reward = Self::current_block_reward();
            let total_supply = Self::total_supply();
            let max_supply = T::MaxSupply::get();
            
            // Check if maximum supply has been reached
            if total_supply >= max_supply {
                // Emit max supply reached event
                let current_block = <frame_system::Pallet<T>>::block_number();
                Self::deposit_event(Event::MaxSupplyReached(current_block, total_supply));
                return
