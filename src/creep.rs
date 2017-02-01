use super::room;
use super::ActionResult;
use super::ffi;

pub enum BodyPart {
	Work,
	Carry,
	Move,
	Attack,
	RangedAttack,
	Heal,
	Claim,
}

pub struct Creep {
	pub id: u32,
	pub hits: u32,
	pub hits_max: u32,
	pub room: room::Room,
	pub carry_energy: u32,
	pub carry_capacity: u32,
}

impl Creep {
	pub fn upgrade_controller(&self, tid: u32) -> ActionResult {
		ActionResult::from_integer(
			unsafe {
				ffi::creep_upgrade_controller(self.id, tid)
			}
		)		
	}
	pub fn harvest(&self, luid: u32) -> ActionResult {
		ActionResult::from_integer(
			unsafe {
				ffi::creep_harvest(self.id, luid)
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