#![feature(core_intrinsics)]
#![feature(start, nostd, alloc, heap_api)]
#![no_std]

pub mod room;
pub mod creep;
pub mod structure;
pub mod pcg;
pub mod heap;
pub mod vec;
pub mod rc;
pub mod spawner;

pub use room::Room;
pub use creep::Creep;
pub use vec::Vec;
pub use rc::Rc;
use core::mem;

mod ffi {
	use core;
	use super::creep;
	use super::Memkey;
	use super::structure;
	use super::vec::Vec;
	use super::room::Room;

	extern {
		pub fn get_heap_region_off() -> u32;
		pub fn get_heap_region_size() -> u32;

		/// Order the creep to move in the specified direction.
		pub fn creep_move(cid: u32, dir: u8) -> i32;
		pub fn creep_harvest(cid: u32, sid: u32) -> i32;
		pub fn creep_moveto(cid: u32, tid: u32) -> i32;
		pub fn creep_upgrade_controller(cid: u32, tid: u32) -> i32;
		pub fn creep_transfer(cid: u32, tid: u32) -> i32;

		// Creep Memory
		pub fn _creep_mem_write(cid: u32, key: u32, data: *const u8, data_size: usize);
		pub fn _creep_mem_read(cid: u32, key: u32, data: *const u8, data_size: usize);
		pub fn creep_mem_key_exist(cid: u32, key: u32, data_size: usize) -> bool;

		/// Non-busy spawn selection to build command.

		#[deprecated]
		pub fn spawn_build(bodyparts: &[creep::BodyPart]);

		pub fn create_creep(spawnid: u32, spec: &structure::SpawnCreepSpec) -> i32;

		/// Helper functions for BAD Emscripten.
		//pub fn write32(addr: usize, val: u32);
		//pub fn read32(addr: usize) -> u32;

		pub fn write8(addr: usize, val: u8);
		pub fn read8(addr: usize) -> u8;

		// Panic function.
		pub fn _panic();

		pub fn _print_string(path: *const u8, path_len: usize);
		pub fn _print_i32(v: i32);
		pub fn _print_eol();
		
		/// Fast debugging output.
		pub fn _debugmark(val: u32);

		/// Room(s)
		pub fn enumerate_rooms() -> &'static Vec<Room>;
		pub fn room_enumerate(id: u32) -> &'static super::room::Enumeration;
	}

	pub fn print_string(s: &str) {
		unsafe { _print_string(s.as_ptr(), s.len()) }
	}

	pub fn print_i32(v: i32) {
		unsafe { _print_i32(v) }
	}

	pub fn print_eol() {
		unsafe { _print_eol() }
	}

	pub unsafe fn write32(addr: usize, val: u32) {
		*(addr as *mut u32) = val;
	}

	pub unsafe fn read32(addr: usize) -> u32 {
		*(addr as *const u32)
	}

	/// Writes the binary representation of `v` into a key in the creep's
	/// memory; however, it requires that `T` of `v` be Copy (byte for byte)
	/// to make the operation safe.
	pub fn creep_mem_write<T: Copy>(cid: u32, key: Memkey, v: T) {
		unsafe {
			_creep_mem_write(cid, key.to_u32(), core::mem::transmute::<&T, *const u8>(&v), core::mem::size_of::<T>());
		}
	}

	pub fn debugmark(val: u32) {
		unsafe {
			_debugmark(val)
		}
	}

	/// Reads the binary representation of `v` from a key in the creep's
	/// memory.
	pub fn creep_mem_read<T: Copy>(cid: u32, key: Memkey) -> Option<T> {
		unsafe {
			let mut v: T = core::mem::uninitialized();

			if creep_mem_key_exist(cid, key.to_u32(), core::mem::size_of::<T>()) == false {
				Option::None
			} else {
				_creep_mem_read(cid, key.to_u32(), core::mem::transmute::<&mut T, *mut u8>(&mut v), core::mem::size_of::<T>());
				Option::Some(v)
			}
		}
	}

	#[cfg(target_family = "emscripten")]
	pub fn panic() {
		unsafe { _panic() }
	}

	#[cfg(not(target_family = "emscripten"))]
	pub fn panic() {
		panic!("unspecified");
	}
}

use ffi::print_string;
use ffi::print_i32;
use ffi::print_eol;

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
	InvalidRustProxyID,
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
			-100 => ActionResult::InvalidRustProxyID,
			_ => ActionResult::Unknown,
		}
	}
}

pub struct Game {
	pub creeps: Vec<Option<Creep>>,
	pub rooms: &'static Vec<room::Room>,
}

pub struct GatherAndWorkCreep {
	pub creep: Creep,
}

pub enum Memkey {
	Gathering,
	Role,
	SubRole,
	Room,
}

impl Memkey {
	pub fn to_u32(&self) -> u32 {
		match self {
			&Memkey::Gathering => 0,
			&Memkey::Role => 1,
			&Memkey::SubRole => 2,
			&Memkey::Room => 3,
		}
	}
}

impl GatherAndWorkCreep {
	pub fn new(creep: Creep) -> GatherAndWorkCreep {
		GatherAndWorkCreep {
			creep: creep,
		}
	}

	/// Use energy from source specified to perform the upgrade 
	/// controller operation on the target specified.
	///
	/// A fast code-path for performing the action.
	pub fn action_upgrade(&self, source: u32, target: u32) -> ActionResult {
		let creep = &self.creep;

		let gathering = creep.mem_read::<u8>(Memkey::Gathering).unwrap_or(0);

		if gathering == 0 && creep.carry_energy == 0 {
			creep.mem_write::<u8>(Memkey::Gathering, 1);
		}

		if gathering == 1 && creep.carry_energy == creep.carry_capacity {
			creep.mem_write::<u8>(Memkey::Gathering, 0);
		}

		if gathering == 1 {
			match creep.harvest(source) {
				ActionResult::NotInRange => { creep.moveto(source); },
				_ => (),
			}
		} else {
			match creep.upgrade_controller(target) {
				ActionResult::NotInRange => { creep.moveto(target); },
				_ => (),
			};
		}

		ActionResult::OK
	}

	pub fn action_transfer(&self, source: u32, target: u32) -> ActionResult {
		let creep = &self.creep;

		let gathering = creep.mem_read::<u8>(Memkey::Gathering).unwrap_or(0);

		print_string("creep.carry_capacity=");
		print_i32(creep.carry_capacity as i32);
		print_eol();

		if gathering == 0 && creep.carry_energy == 0 {
			creep.mem_write::<u8>(Memkey::Gathering, 1);
		}

		if gathering == 1 && creep.carry_energy == creep.carry_capacity {
			creep.mem_write::<u8>(Memkey::Gathering, 0);
		}

		if gathering == 1 {
			print_string("gathering");
			print_eol();
			match creep.harvest(source) {
				ActionResult::NotInRange => { creep.moveto(source); },
				_ => (),
			}
		} else {
			print_string("transfering");
			print_eol();			
			match creep.transfer(target) {
				ActionResult::NotInRange => { creep.moveto(target); },
				_ => (),
			};
		}

		ActionResult::OK
	}
}

#[derive(Copy, Clone, PartialEq)]
pub enum Role {
	SourceMiner,
	SourceHauler,
	BaseFueling,
	SourceMinerAndHauler,
	None,
}

impl Role {
	pub fn to_u32(&self) -> u32 {
		match self {
			&Role::SourceMiner => 0,
			&Role::SourceHauler => 1,
			&Role::BaseFueling => 2,
			&Role::SourceMinerAndHauler => 3,
			&Role::None => 4,
		}
	}
}

#[derive(Copy, Clone, PartialEq)]
pub enum SubRole {
	None,
}

impl SubRole {
	pub fn to_u32(&self) -> u32 {
		match self {
			&SubRole::None => 0,
		}
	}
}

/// Handling room specific functions.
pub fn game_tick_room(game: &mut Game, room: &Room) {
	let mut spawner = spawner::Spawner::new(room.clone());

	/// Enumerate sources in the room.
	let renum = room.get_enum();

	for q in 0..renum.sources.len() {
		let source = &renum.sources[q];

		/// Use the spawner to get one or more creeps capable
		/// of fully harvesting the source.
		let mut creeps = spawner.get_creep(game, spawner::BodyPartSpec {
			work: 6,
			carry: 6,
			claim: 0,
			attack: 0,
			rattack: 0,
			heal: 0,
			tough: 0,
			offroad: true,
			num: 2,
		}, Role::SourceMiner, SubRole::None, -5.0);

		for w in 0..creeps.len() {
			let creep = creeps.pop();
			print_string("spawner returned creep ");
			print_i32(creep.id as i32);
			print_eol();

			print_string("renum sources[0]=");
			print_i32(renum.sources[0].id as i32);
			print_eol();

			let creep = GatherAndWorkCreep::new(creep);
			creep.action_transfer(renum.sources[0].id, renum.spawns[0].structure.id);
		}
	}

	spawner.action(game);
}

/// Entry function from Screeps.
#[no_mangle]
pub extern fn game_tick(game: &mut Game) -> u32 {
	print_string("hello world");
	print_eol();

	//print_string("size_of debug ");
	//print_i32(core::mem::size_of::<Option<Creep>>() as i32);
	//print_eol();

	let rooms = game.rooms;

	for q in 0..rooms.len() {
		let room = &rooms[q];
		print_string("got room ");
		print_i32(room.id as i32);
		print_eol();
		game_tick_room(game, room);
	}
	0
}

/// Obsolete function for development.
pub fn game_tick_mine_and_upgrade(game: &mut Game) -> u32 {
	if game.creeps.len() < 12 {
		// Keep a specific number of creeps built.
		let bps: [creep::BodyPart; 3] = [
			creep::BodyPart::Move,
			creep::BodyPart::Carry,
			creep::BodyPart::Work,
		];

		unsafe { ffi::spawn_build(&bps) };
	}

	// Make the creeps do something useful.
	let mut q = 0;

	if game.creeps.len() > 0 {
		let renum = game.creeps[0].as_ref().unwrap().room.get_enum();

		while q < game.creeps.len() && q < 1 {
			let creep = GatherAndWorkCreep::new(game.creeps[q].take().unwrap());
			creep.action_transfer(renum.sources[0].id, renum.spawns[0].structure.id);
			q += 1;
		}

		while q < game.creeps.len() && q < 4 {
			let creep = GatherAndWorkCreep::new(game.creeps[q].take().unwrap());
			creep.action_upgrade(renum.sources[1].id, renum.controller);
			q += 1;
		}

		while q < game.creeps.len() && q < 20 {
			let creep = GatherAndWorkCreep::new(game.creeps[q].take().unwrap());
			creep.action_upgrade(renum.sources[0].id, renum.controller);
			q += 1;
		}
	}

	0
}

#[test]
fn it_works() {

}