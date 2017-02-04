use super::structure;
use super::ffi;
use super::mem;
use super::Vec;
use core;


#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum RoomGUID {
	None,
	guid (u32),
}

#[repr(C)]
#[derive(Clone)]
pub struct Room {
	pub id: u32,
	pub guid: RoomGUID,
	pub renum: usize,
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
	pub extensions: Vec<structure::Extension>,
}

impl Room {
	pub fn get_enum(&self) -> &'static Enumeration {
		unsafe {
			core::mem::transmute::<usize, &'static Enumeration>(self.renum)
		}
	}

	// Return enumeration of structures for this room.
	//pub fn enumerate(&self) -> &'static Enumeration {
	//	unsafe {
	//		ffi::room_enumerate(self.id)
	//	}
	//}

	// Return enumeration of all visible rooms.
	//pub fn enumerate_rooms() -> &'static Vec<Room> {
	//	unsafe {
	//		ffi::enumerate_rooms()
	//	}
	//}
}