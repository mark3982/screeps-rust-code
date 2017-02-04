use super::room;
use super::ActionResult;
use super::ffi;
use super::Memkey;

#[derive(Copy, Clone, PartialEq)]
pub enum BodyPart {
	Work,
	Carry,
	Move,
	Attack,
	RangedAttack,
	Heal,
	Claim,
}

#[repr(C)]
pub struct Creep {
	pub id: u32,
	pub hits: u32,
	pub hits_max: u32,
	pub room: room::Room,
	pub carry_energy: u32,
	pub carry_capacity: u32,
	pub ticks_to_live: u32,
	pub spawning: bool,
}

impl Creep {
	pub fn mem_read<T: Copy>(&self, key: Memkey) -> Option<T> {
		ffi::creep_mem_read(self.id, key)
	}

	pub fn mem_write<T: Copy>(&self, key: Memkey, v: T) {
		ffi::creep_mem_write(self.id, key, v)
	}

	pub fn upgrade_controller(&self, tid: u32) -> ActionResult {
		ActionResult::from_integer(
			unsafe {
				ffi::creep_upgrade_controller(self.id, tid)
			}
		)		
	}

	pub fn transfer(&self, tid: u32) -> ActionResult {
		ActionResult::from_integer(
			unsafe {
				ffi::creep_transfer(self.id, tid)
			}
		)		
	}
	
	pub fn harvest(&self, tid: u32) -> ActionResult {
		ActionResult::from_integer(
			unsafe {
				ffi::creep_harvest(self.id, tid)
			}
		)
	}
	pub fn moveto(&self, tid: u32) -> ActionResult {
		ActionResult::from_integer(
			unsafe {
				ffi::creep_moveto(self.id, tid)
			}
		)	
	}
}