#![feature(start)]
#![no_main]

pub struct Creep {
	qid: u32,
	hits: u32,
	hits_max: u32,
}

pub enum StructureTypeCode {
	SPAWN,
	EXTENSION,
	ROAD,
	WALL,
	RAMPART,
	KEEPER_LAIR,
	PORTAL,
	CONTROLLER,
	LINK,
	STORAGE,
	TOWER,
	OBSERVER,
	POWER_BANK,
	POWER_SPAWN,
	EXTRACTOR,
	LAB,
	TERMINAL,
	CONTAINER,
	NUKER,
}

impl StructureTypeCode {
	fn get_string(&self) -> &str {
		match self {
			&StructureTypeCode::SPAWN => "spawn",
			&StructureTypeCode::EXTENSION => "extension",
			&StructureTypeCode::ROAD => "road",
			&StructureTypeCode::WALL => "constructedWall",
			&StructureTypeCode::RAMPART => "rampart",
			&StructureTypeCode::KEEPER_LAIR => "keeperLair",
			&StructureTypeCode::PORTAL => "portal",
			&StructureTypeCode::CONTROLLER => "controller",
			&StructureTypeCode::LINK => "link",
			&StructureTypeCode::STORAGE => "storage",
			&StructureTypeCode::TOWER => "tower",
			&StructureTypeCode::OBSERVER => "observer",
			&StructureTypeCode::POWER_BANK => "powerBank",
			&StructureTypeCode::POWER_SPAWN => "powerSpawn",
			&StructureTypeCode::EXTRACTOR => "extractor",
			&StructureTypeCode::LAB => "lab",
			&StructureTypeCode::TERMINAL => "terminal",
			&StructureTypeCode::CONTAINER => "container",
			&StructureTypeCode::NUKER => "nuker",
		}
	}
}

pub trait Structure {
	fn get_hits(&self) -> u32;
	fn get_hits_max(&self) -> u32;
	fn get_structure_type(&self) -> StructureTypeCode;
}

pub struct StructureSpawn {
	hits: u32,
	hits_max: u32,
}

impl Structure for StructureSpawn {
	fn get_hits(&self) -> u32 { self.hits }
	fn get_hits_max(&self) -> u32 { self.hits_max }
	fn get_structure_type(&self) -> StructureTypeCode { StructureTypeCode::SPAWN }
}

pub struct StructureExtension {
	hits: u32,
	hits_max: u32,
}

pub struct StructureRoad {
	hits: u32,
	hits_max: u32,
}

pub struct Room {
	name: [u8; 6],
	spawns: Vec<StructureSpawn>,
	extensions: Vec<StructureExtension>,
	roads: Vec<StructureRoad>,
}

pub struct Game {
	creeps: Vec<Creep>,
	rooms: Vec<Room>,
}

extern {
	pub fn creep_move(cndx: u16, dir: u8);
	pub fn write32(addr: usize, val: u32);
	pub fn read32(addr: usize) -> u32;
	pub fn write8(addr: usize, val: u8);
	pub fn read8(addr: usize) -> u8;
	pub fn debugmark(val: usize);
}

/// Sets the region for the heap functions.
pub extern fn __heap_region(addr: usize, size: usize) {
	unsafe {
		write32(0, addr as u32);
		write32(4, size as u32);
	}
}

/// Returns the address of a chunk of memory that is of at least the size
/// specified.
///
/// This function is designed to have its region specified. This allocator
/// is a balance between performance and code size. 
///
/// The intention is that there is an abundance of RAM and that each invocation
/// of the code is going to only use the heap for a few rounds before it is
/// destroyed, therefore, no work is done to reduce heap segmentation.
#[no_mangle]
pub extern fn __allocate(mut size: usize, _align: usize) -> *mut u8 {
	unsafe {
		// TODO: implement alignment by overallocation if needed
		let regionoff: u32 = read32(0);
		let regionsize: u32 = read32(4);
		let mut loff = 0;

		// _Must_ have 4 byte alignment for chunks.
		if size & 3 > 0 {
			//debugmark(9999);
			size = ((size >> 2) << 2) + 4;
		}

		while (true) {
			let chunkflags: u32 = read32((regionoff + 0 + loff) as usize);
			let chunksize: u32 = read32((regionoff + 4 + loff) as usize);

			//debugmark(99);
			//debugmark(chunkflags as usize);
			//debugmark(chunksize as usize);

			if chunkflags & (1 as u32) == 0 && chunksize as u32 >= size as u32 {
				// The chunk is free.
				if chunksize as u32 > size as u32 + 32u32 {
					// Use a portion of the chunk.
					write32((regionoff + loff) as usize, 1u32);
					write32((regionoff + 4u32 + loff) as usize, size as u32);
					// Write the next header as unused and copy the last chunk bit flag.
					write32((regionoff + loff + 8u32 + size as u32 + 0u32) as usize, chunkflags & 2 as u32);
					write32((regionoff + loff + 8u32 + size as u32 + 4u32) as usize, (chunksize - size as u32 - 8 as u32) as u32);
				} else {
					// Use the entire chunk, therefore, leave everything as is. Also,
					// copy the last chunk bit flag.
					write32((regionoff + loff) as usize, (chunkflags & 2 as u32) | 1 as u32);
				}

				// Skip both flags and size fields.
				return (regionoff + 8 + loff) as *mut u8;
			}

			if chunkflags & (2 as u32) == 2 {
				//// The last chunk has been scanned.
				break;
			}

			loff = loff + 8 + chunksize;
		}
	}
	
	0 as *mut u8
}

#[no_mangle]
pub extern fn __deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
	let hdr = ptr as usize - 8;
	unsafe {
		let flags = read32(hdr);

		// Copy the last-chunk flag but clear the used flag.
		write32(hdr, flags & 2);
	}
}

/// Copies the specified number of bytes from source to destination.
///
/// A version of memcpy to reduce the amount of emitted code. A balance
/// between performance and code size.
unsafe fn memcpy(dst: *mut u8, src: *mut u8, size: usize) {
	let chunks = size / 4;
	let slack = size - (chunks * 4);
	let q = 0usize;

	for q in 0..chunks {
		unsafe {
			write32(dst as usize + q * 4, read32(src as usize + q * 4))
		}
	}

	let off = chunks * 4;

	for q in 0..slack {
		unsafe {
			write8(dst as usize + off + q, read8(src as usize + off + q));
		}
	}
}

#[no_mangle]
pub extern fn __reallocate(ptr: *mut u8, _old_size: usize, size: usize, _align: usize) -> *mut u8 {
	__deallocate(ptr, _old_size, _align);

	let nptr = __allocate(size, _align);

	if nptr as usize == 0 {
		0 as *mut u8
	} else {
		if size > _old_size {
			unsafe { memcpy(nptr, ptr, _old_size); }
		} else {
			unsafe { memcpy(nptr, ptr, size); }
		}

		nptr
	}
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize) -> usize {
	0
} 

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
	size
}

#[no_mangle]
pub extern fn game_tick(game: &Game) -> u32 {
	let mut a: u32 = 0;

	for ndx in 0..1024 {
		for cndx in 0..game.creeps.len() {
			a += game.creeps[cndx].hits;
			unsafe {
				// Tell the creep to move up. It is faster
				// to use a static index to refer to the creep
				// since it will fit into a single processor
				// register. This is contrasted to a string or
				// integer value representing the MongoDB _id field
				// of the creep which is cumbersome.
				//
				//creep_move(cndx as u16, 1);
			}		
		}
	}

	// 2097215
	__allocate(100, 0);
	__allocate(100, 0) as u32
}