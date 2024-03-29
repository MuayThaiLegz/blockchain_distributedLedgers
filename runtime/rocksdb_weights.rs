// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-08 (Y/M/D)
//! HOSTNAME: `LAPTOP-D3QLJQ6T`, CPU: `Intel(R) Core(TM) i7-10750H CPU @ 2.60GHz`
//!
//! DATABASE: `RocksDb`, RUNTIME: `Local Testnet`
//! BLOCK-NUM: `BlockId::Number(0)`
//! SKIP-WRITE: `false`, SKIP-READ: `false`, WARMUPS: `1`
//! STATE-VERSION: `V1`, STATE-CACHE-SIZE: `0`
//! WEIGHT-PATH: ``
//! METRIC: `Average`, WEIGHT-MUL: `1`, WEIGHT-ADD: `0`

// Executed Command:
//   ./target/release/node-template
//   benchmark
//   storage
//   --state-version
//   1

/// Storage DB weights for the `Local Testnet` runtime and `RocksDb`.
pub mod constants {
	use frame_support::{
		parameter_types,
		weights::{constants, RuntimeDbWeight},
	};

	parameter_types! {
		/// By default, Substrate uses `RocksDB`, so this will be the weight used throughout
		/// the runtime.
		pub const RocksDbWeight: RuntimeDbWeight = RuntimeDbWeight {
			/// Time to read one storage item.
			/// Calculated by multiplying the *Average* of all values with `1` and adding `0`.
			///
			/// Stats nanoseconds:
			///   Min, Max: 8_000, 43_600
			///   Average:  12_519
			///   Median:   11_100
			///   Std-Dev:  6288.87
			///
			/// Percentiles nanoseconds:
			///   99th: 43_600
			///   95th: 24_100
			///   75th: 12_400
			read: 12_519 * constants::WEIGHT_PER_NANOS,

			/// Time to write one storage item.
			/// Calculated by multiplying the *Average* of all values with `1` and adding `0`.
			///
			/// Stats nanoseconds:
			///   Min, Max: 18_300, 1_405_700
			///   Average:  70_430
			///   Median:   30_400
			///   Std-Dev:  225987.94
			///
			/// Percentiles nanoseconds:
			///   99th: 1_405_700
			///   95th: 68_100
			///   75th: 38_200
			write: 70_430 * constants::WEIGHT_PER_NANOS,
		};
	}

	#[cfg(test)]
	mod test_db_weights {
		use super::constants::RocksDbWeight as W;
		use frame_support::weights::constants;

		/// Checks that all weights exist and have sane values.
		// NOTE: If this test fails but you are sure that the generated values are fine,
		// you can delete it.
		#[test]
		fn bound() {
			// At least 1 µs.
			assert!(
				W::get().reads(1) >= constants::WEIGHT_PER_MICROS,
				"Read weight should be at least 1 µs."
			);
			assert!(
				W::get().writes(1) >= constants::WEIGHT_PER_MICROS,
				"Write weight should be at least 1 µs."
			);
			// At most 1 ms.
			assert!(
				W::get().reads(1) <= constants::WEIGHT_PER_MILLIS,
				"Read weight should be at most 1 ms."
			);
			assert!(
				W::get().writes(1) <= constants::WEIGHT_PER_MILLIS,
				"Write weight should be at most 1 ms."
			);
		}
	}
}
