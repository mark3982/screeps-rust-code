use super::structure;
use super::ffi;
use super::mem;
use super::Vec;

pub struct Room {
	pub id: u32,
}

pub struct Source {
	pub id: u32,
	pub energy: u32,
	pub energy_capacity: u32,
	pub ticks_to_regenerate: u32,
}

pub struct Enumeration {
	pub sources: Vec<Source>,
	pub controller: u32,
}

impl Room {
	pub fn enumerate(&self) -> &Enumeration {
		unsafe {
			ffi::room_enumerate(self.id)
		}
	}
}