// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for delegation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=delegation
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/delegation.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `delegation`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> delegation::WeightInfo for WeightInfo<T> {
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	// Storage: Ctype Ctypes (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationNodes (r:0 w:1)
	fn create_hierarchy() -> Weight {
		(37_795_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_delegation() -> Weight {
		(45_303_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight {
		(18_819_000 as Weight)
			// Standard Error: 50_000
			.saturating_add((13_562_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 50_000
			.saturating_add((180_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Delegation DelegationNodes (r:6 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_leaf(r: u32, c: u32, ) -> Weight {
		(32_036_000 as Weight)
			// Standard Error: 35_000
			.saturating_add((133_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 35_000
			.saturating_add((5_171_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	fn remove_delegation(r: u32, ) -> Weight {
		(52_158_000 as Weight)
			// Standard Error: 67_000
			.saturating_add((23_383_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:0 w:1)
	fn reclaim_deposit(r: u32, ) -> Weight {
		(44_630_000 as Weight)
			// Standard Error: 82_000
			.saturating_add((23_457_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Delegation DelegationNodes (r:1 w:0)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn can_attest() -> Weight {
		(12_490_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	fn can_revoke(c: u32, ) -> Weight {
		(8_147_000 as Weight)
			// Standard Error: 37_000
			.saturating_add((5_115_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	fn can_remove(c: u32, ) -> Weight {
		(8_136_000 as Weight)
			// Standard Error: 25_000
			.saturating_add((5_056_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
	}
}
