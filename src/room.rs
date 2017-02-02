use super::structure;
use super::ffi;
use super::mem;
use super::Vec;

#[repr(C)]
pub struct Room {
	pub id: u32,
	pub guid: u32,
}

#[repr(C)]
pub struct Source {
	pub id: u32,
	pub energy: u32,
	pub energy_capacity: u32,
	pub ticks_to_regenerate: u32,
}

#[repr(C)]
pub struct Enumeration {
	pub sources: Vec<Source>,
	pub controller: u32,
	pub spawns: Vec<structure::Spawn>,
}

impl Room {
	pub fn enumerate(&self) -> &'static Enumeration {
		unsafe {
			ffi::room_enumerate(self.id)
		}
	}
}