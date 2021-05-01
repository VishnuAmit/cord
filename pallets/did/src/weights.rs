
//! Autogenerated weights for pallet_did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-04-30, STEPS: [20, ], REPEAT: 1, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Interpreted, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/cord
// benchmark
// --chain=dev
// --execution=wasm
// --pallet=pallet_did
// --extrinsic=*
// --steps=20
// --output=./pallets/did/src/weights.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_did.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_did::WeightInfo for WeightInfo<T> {
	fn anchor() -> Weight {
		(90_660_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove() -> Weight {
		(70_391_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
