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
}

#[no_mangle]
pub extern fn game_tick(game: &Game) -> u32 {
	let mut a: u32 = 0;

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
			creep_move(cndx as u16, 1);
		}		
	}

	a
}