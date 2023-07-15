use std::{ops::DerefMut, sync::Mutex};

use ctsh::ctsh;

ctsh! {
	run "echo" "#[derive(Debug)] struct Foo { element: u64 } #[derive(Debug)] struct Bar { element: u64 }" as stmts;
}

const SEED: [u8; 4096] = *ctsh!("head" "-c" "4096" "/dev/urandom" as bytes);
pub struct RNG {
	index: usize,
	state: [u8; 4096],
}

impl RNG {
	pub fn singleton() -> &'static Mutex<RNG> {
		static CELL: std::sync::OnceLock<Mutex<RNG>> = std::sync::OnceLock::new();
		CELL.get_or_init(|| Mutex::new(RNG { index: 0, state: SEED }))
	}
}

impl Iterator for RNG {
	type Item = u8;
	fn next(&mut self) -> Option<Self::Item> {
		let result = self.state.get(self.index).copied();
		if result.is_none() {
			// do some PRNG stuff here
		} else {
			self.index += 1;
		}
		result
	}
}

fn main() {
	ctsh! {
		run "echo" "#[derive(Debug)] struct Foo { element: u64 } use Bar;" as stmts;
		"echo" "let element = 43;" | "tr" "3" "2" as stmts;
	};

	for x in RNG::singleton().lock().unwrap().deref_mut() {
		println!("{x:?}")
	}
}
