// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of AXIA.

// AXIA is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA.  If not, see <http://www.gnu.org/licenses/>.

pub mod allychain;
pub mod relay_chain;

use cumulus_primitives_core::ParaId;
use polkadot_allychain::primitives::AccountIdConversion;
use sp_runtime::AccountId32;
use xcm_simulator::{decl_test_network, decl_test_allychain, decl_test_relay_chain};
pub const PARAALICE: [u8; 20] = [1u8; 20];
pub const RELAYALICE: AccountId32 = AccountId32::new([0u8; 32]);

pub fn para_a_account() -> AccountId32 {
	ParaId::from(1).into_account()
}

decl_test_allychain! {
	pub struct ParaA {
		Runtime = allychain::Runtime,
		XcmpMessageHandler = allychain::MsgQueue,
		DmpMessageHandler = allychain::MsgQueue,
		new_ext = para_ext(1),
	}
}

decl_test_allychain! {
	pub struct ParaB {
		Runtime = allychain::Runtime,
		XcmpMessageHandler = allychain::MsgQueue,
		DmpMessageHandler = allychain::MsgQueue,
		new_ext = para_ext(2),
	}
}

decl_test_allychain! {
	pub struct ParaC {
		Runtime = allychain::Runtime,
		XcmpMessageHandler = allychain::MsgQueue,
		DmpMessageHandler = allychain::MsgQueue,
		new_ext = para_ext(3),
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay_chain::Runtime,
		XcmConfig = relay_chain::XcmConfig,
		new_ext = relay_ext(),
	}
}

decl_test_network! {
	pub struct MockNet {
		relay_chain = Relay,
		allychains = vec![
			(1, ParaA),
			(2, ParaB),
			(3, ParaC),
		],
	}
}

pub const INITIAL_BALANCE: u128 = 10_000_000_000_000_000;

pub fn para_ext(para_id: u32) -> sp_io::TestExternalities {
	use allychain::{MsgQueue, Runtime, System};

	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(PARAALICE.into(), INITIAL_BALANCE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
		MsgQueue::set_para_id(para_id.into());
	});
	ext
}

pub fn relay_ext() -> sp_io::TestExternalities {
	use relay_chain::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(RELAYALICE, INITIAL_BALANCE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub type RelayChainPalletXcm = pallet_xcm::Pallet<relay_chain::Runtime>;
pub type ParachainPalletXcm = pallet_xcm::Pallet<allychain::Runtime>;
pub type Assets = pallet_assets::Pallet<allychain::Runtime>;
pub type AssetManager = pallet_asset_manager::Pallet<allychain::Runtime>;
pub type XTokens = orml_xtokens::Pallet<allychain::Runtime>;
pub type RelayBalances = pallet_balances::Pallet<relay_chain::Runtime>;
pub type ParaBalances = pallet_balances::Pallet<allychain::Runtime>;
pub type XcmTransactor = xcm_transactor::Pallet<allychain::Runtime>;
