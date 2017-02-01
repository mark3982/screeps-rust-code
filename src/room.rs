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
	pub fn enumerate(&self) -> Enumeration {
		let mut e;

		unsafe {
			// To simplify the FFI work it requires that the second
			// argument point to the local space allocated for the
			// enumeration and if needed the FFI will allocate additional
			// heap space for the elements of the enumeration. Then
			// the numeration when dropped will simply free that memory.
			e = mem::uninitialized::<Enumeration>();
			ffi::room_enumerate(self.id, &mut e);
		}

		e
	}
}