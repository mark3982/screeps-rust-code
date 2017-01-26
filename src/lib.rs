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
	pub fn debugmark(val: usize);
}

// The region the allocator works*mut  in is specified by these;
// global variables. The ability to select a specific region
// allows regions to easily be offloaded into the `Memory` for 
// Screeps as a binary string.

#[no_mangle]
pub extern fn __allocate(size: usize, _align: usize) -> *mut u8 {
	unsafe {
		// TODO: implement alignment by overallocation if needed
		let regionoff: u32 = read32(0);
		let regionsize: u32 = read32(4);
		let mut loff = 0;

		while (true) {
			let chunkflags: u32 = read32((regionoff + 0 + loff) as usize);
			let chunksize: u32 = read32((regionoff + 4 + loff) as usize);

			if chunkflags & (1 as u32) == 0 && chunksize as u32 >= size as u32 {
				// The chunk is free.
				if chunksize as u32 > size as u32 + 32u32 {
					// Use a portion of the chunk.
					write32((regionoff + 4u32 + loff) as usize, size as u32);
					write32((regionoff + loff) as usize, 1u32);
					// Write the next header as unused and copy the last chunk bit flag.
					write32((regionoff + 8u32 + size as u32 + 0u32) as usize, chunkflags & 2 as u32);
					write32((regionoff + 8u32 + size as u32 + 4u32) as usize, chunksize - size as u32);
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
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {

}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize, _align: usize) -> *mut u8 {
	0 as *mut u8
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize) -> usize {
	0
} 

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
	0
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