use crate as audit_log_pallet;
use frame::{prelude::*, runtime::prelude::*, testing_prelude::*};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
#[frame_construct_runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system;

    #[runtime::pallet_index(1)]
    pub type AuditLogPallet = audit_log_pallet;
}

// System pallet configuration
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
}

// AuditLog pallet configuration
impl audit_log_pallet::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_name::weights::SubstrateWeight<Test>;
}

// Test externalities initialization
pub fn new_test_ext() -> TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}