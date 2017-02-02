#![feature(start, nostd)]
#![no_main]
#![no_std]
pub mod fmemseg;
pub mod room;
pub mod creep;
pub mod structure;
pub mod pcg;
pub mod heap;
pub mod vec;
pub mod rc;

pub use room::Room;
pub use creep::Creep;
pub use vec::Vec;
pub use rc::Rc;
pub use core::mem;

mod ffi {
	use core;
	use super::creep;

	extern {
		pub fn get_heap_region_off() -> u32;
		pub fn get_heap_region_size() -> u32;

		/// Order the creep to move in the specified direction.
		pub fn creep_move(cid: u32, dir: u8) -> i32;
		pub fn creep_harvest(cid: u32, sid: u32) -> i32;
		pub fn creep_moveto(cid: u32, tid: u32) -> i32;
		pub fn creep_upgrade_controller(cid: u32, tid: u32) -> i32;

		// Creep Memory
		pub fn _creep_mem_write(cid: u32, key: u32, data: *const u8, data_size: usize);
		pub fn _creep_mem_read(cid: u32, key: u32, data: *const u8, data_size: usize);
		pub fn creep_mem_key_exist(cid: u32, key: u32, data_size: usize) -> bool;

		/// Non-busy spawn selection to build command.
		pub fn spawn_build(bodyparts: &[creep::BodyPart]);

		/// Helper functions for BAD Emscripten.
		//pub fn write32(addr: usize, val: u32);
		//pub fn read32(addr: usize) -> u32;

		pub fn write8(addr: usize, val: u8);
		pub fn read8(addr: usize) -> u8;

		// Panic function.
		pub fn _panic();

		/// Memory accessors.
		pub fn _memory_get_integer(path: *const u8, path_len: usize);
		
		/// Fast debugging output.
		pub fn _debugmark(val: u32);

		/// Room
		pub fn room_enumerate(id: u32) -> &super::room::Enumeration;
	}

	pub fn write32(addr: usize, val: u32) {
		unsafe {
			*(addr as *mut u32) = val;
		}
	}

	pub fn read32(addr: usize) -> u32 {
		unsafe {
			*(addr as *const u32)
		}
	}

	/// Writes the binary representation of `v` into a key in the creep's
	/// memory; however, it requires that `T` of `v` be Copy (byte for byte)
	/// to make the operation safe.
	pub fn creep_mem_write<T: Copy>(cid: u32, key: u32, v: T) {
		unsafe {
			_creep_mem_write(cid, key, core::mem::transmute::<&T, *const u8>(&v), core::mem::size_of::<T>());
		}
	}

	pub fn debugmark(val: u32) {
		unsafe {
			_debugmark(val)
		}
	}

	/// Reads the binary representation of `v` from a key in the creep's
	/// memory.
	pub fn creep_mem_read<T: Copy>(cid: u32, key: u32) -> Option<T> {
		unsafe {
			let mut v: T = core::mem::uninitialized();

			if creep_mem_key_exist(cid, key, core::mem::size_of::<T>()) == false {
				Option::None
			} else {
				_creep_mem_read(cid, key, core::mem::transmute::<&mut T, *mut u8>(&mut v), core::mem::size_of::<T>());
				Option::Some(v)
			}
		}
	}

	pub fn panic() {
		unsafe { _panic() }
	}

	pub fn memory_get_integer(path: &str) {
		unsafe {
			_memory_get_integer(path.as_ptr(), path.len());
		}
	}	
}

pub enum ActionResult {
	OK,
	NotOwner,
	NoPath,
	NameExists,
	Busy,
	NotFound,
	NotEnoughEnergy,
	NotEnoughResources,
	InvalidTarget,
	Full,
	NotInRange,
	InvalidArgs,
	Tired,
	NoBodypart,
	NotEnoughExtensions,
	RCLNotEnough,
	GCLNotEnough,
	Unknown,
}

impl ActionResult {
	pub fn from_integer(i: i32) -> ActionResult {
		match i {
			0 => ActionResult::OK,
			-1 => ActionResult::NotOwner,
			-2 => ActionResult::NameExists,
			-3 => ActionResult::Busy,
			-4 => ActionResult::NotFound,
			-5 => ActionResult::NotEnoughEnergy,
			-6 => ActionResult::NotEnoughResources,
			-7 => ActionResult::InvalidTarget,
			-8 => ActionResult::Full,
			-9 => ActionResult::NotInRange,
			-10 => ActionResult::InvalidArgs,
			-11 => ActionResult::Tired,
			-12 => ActionResult::NoBodypart,
			-13 => ActionResult::NotEnoughExtensions,
			-14 => ActionResult::RCLNotEnough,
			-15 => ActionResult::GCLNotEnough,
			_ => ActionResult::Unknown,
		}
	}
}

pub struct Game {
	creeps: Vec<Creep>,
}

#[no_mangle]
pub extern fn game_tick(game: &Game) -> u32 {
	let mut a: u32 = 0;

	let mut r = pcg::PcgRng::new_unseeded();

	// Keep a specific number of creeps built.
	let bps: [creep::BodyPart; 3] = [
		creep::BodyPart::Move,
		creep::BodyPart::Carry,
		creep::BodyPart::Work,
	];

	if game.creeps.len() < 3 {
		unsafe { ffi::spawn_build(&bps) };
	}

	// Make the creeps do something useful.
	for q in 0..game.creeps.len() {
		let ref creep = game.creeps[q];

		let gathering = ffi::creep_mem_read::<u8>(creep.id, 0).unwrap_or(0);

		if gathering == 0 && creep.carry_energy == 0 {
			ffi::creep_mem_write::<u8>(creep.id, 0, 1);
		}

		if gathering == 1 && creep.carry_energy == creep.carry_capacity {
			ffi::creep_mem_write::<u8>(creep.id, 0, 0);
		}

		let renum = creep.room.enumerate();

		//rand::thread_rng().choose(&renum.sources);
		if gathering == 1 {
			// Gathering
			let rsrcndx = r.next_u32() % renum.sources.len() as u32;
			let slid = renum.sources[rsrcndx as usize].id;
			match creep.harvest(slid) {
				ActionResult::NotInRange => { creep.moveto(slid); },
				_ => (),
			}
		} else {
			// Working
			match creep.upgrade_controller(renum.controller) {
				ActionResult::NotInRange => { creep.moveto(renum.controller); },
				_ => (),
			};
		}
	}

	0
}