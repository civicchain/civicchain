use crate as civicchain_pow;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64, OnFinalize, OnInitialize},
};
use sp_core::{H256, U256};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        PowPallet: civicchain_pow,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = ();
    type FreezeIdentifier = ();
    type MaxHolds = ();
    type MaxFreezes = ();
}

parameter_types! {
    pub const BlockReward: u64 = 60;
    pub const BlocksPerYear: u32 = 2_628_000; // ~5 segundos por bloco, 365 dias
    pub const HalvingYears: u32 = 5;
    pub const MaxSupply: u64 = 29_000_000;
}

impl civicchain_pow::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type RewardHandler = ();
    type BlockReward = BlockReward;
    type BlocksPerYear = BlocksPerYear;
    type HalvingYears = HalvingYears;
    type MaxSupply = MaxSupply;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 10), (2, 20)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    civicchain_pow::GenesisConfig::<Test> {
        initial_difficulty: U256::from(1_000_000),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        if System::block_number() > 0 {
            PowPallet::on_finalize(System::block_number());
            System::on_finalize(System::block_number());
        }
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        PowPallet::on_initialize(System::block_number());
    }
}
