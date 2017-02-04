use super::structure;
use super::ffi;
use super::mem;
use super::Vec;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum RoomGUID {
	None,
	guid (u32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Room {
	pub id: u32,
	pub guid: RoomGUID,
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
	/// Return `Room` for the room specified by the Rust to JS proxy ID.
	pub fn get_room_from_id(roomid: u32) -> Room {
		// TODO: Assign a proper room GUID.
		Room {
			id: roomid,
			guid: RoomGUID::None,
		}
	}

	/// Return enumeration of structures for this room.
	pub fn enumerate(&self) -> &'static Enumeration {
		unsafe {
			ffi::room_enumerate(self.id)
		}
	}

	/// Return enumeration of all visible rooms.
	pub fn enumerate_rooms() -> &'static Vec<Room> {
		unsafe {
			ffi::enumerate_rooms()
		}
	}
}