use crate as civicchain_contracts;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64, OnFinalize, OnInitialize},
};
use pallet_contracts::{
    weights::WeightInfo,
    Config as ContractsConfig,
};
use sp_core::H256;
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
        Contracts: pallet_contracts,
        ContractsPallet: civicchain_contracts,
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
    pub const DepositPerItem: u64 = 1;
    pub const DepositPerByte: u64 = 1;
    pub const DefaultDepositLimit: u64 = 1000;
    pub const MaxDelegateDependencies: u32 = 32;
    pub const CodeHashLockupDepositPercent: u64 = 0;
    pub const MaxStorageKeyLen: u32 = 128;
    pub const MaxCodeLen: u32 = 2 * 1024 * 1024; // 2 MB
    pub const MaxDebugBufferLen: u32 = 2 * 1024 * 1024; // 2 MB
    pub const MaxValueSize: u32 = 16 * 1024; // 16 KB
    pub const DeletionQueueDepth: u32 = 1024;
    pub const DeletionWeightLimit: Weight = Weight::from_parts(500_000_000, 0);
    pub const Schedule: pallet_contracts::Schedule<Test> = Default::default();
}

impl pallet_contracts::Config for Test {
    type Time = ();
    type Randomness = ();
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallFilter = frame_support::traits::Everything;
    type CallStack = [pallet_contracts::Frame<Self>; 5];
    type WeightPrice = ();
    type WeightInfo = ();
    type ChainExtension = ();
    type Schedule = Schedule;
    type DepositPerByte = DepositPerByte;
    type DepositPerItem = DepositPerItem;
    type DefaultDepositLimit = DefaultDepositLimit;
    type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
    type MaxCodeLen = MaxCodeLen;
    type MaxStorageKeyLen = MaxStorageKeyLen;
    type UnsafeUnstableInterface = frame_support::traits::ConstBool<false>;
    type MaxDebugBufferLen = MaxDebugBufferLen;
    type RuntimeHoldReason = RuntimeHoldReason;
    type Migrations = ();
    type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
    type MaxDelegateDependencies = MaxDelegateDependencies;
    type Debug = ();
    type Environment = ();
    type MaxValueSize = MaxValueSize;
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
}

parameter_types! {
    pub const MinContractFee: u64 = 1;
    pub const MaxTransactionsPerBlock: u32 = 1000;
}

impl civicchain_contracts::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinContractFee = MinContractFee;
    type MaxTransactionsPerBlock = MaxTransactionsPerBlock;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 1000), (2, 2000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        if System::block_number() > 0 {
            ContractsPallet::on_finalize(System::block_number());
            System::on_finalize(System::block_number());
        }
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        ContractsPallet::on_initialize(System::block_number());
    }
}
