pub enum StructureTypeCode {
	Spawn,
	Extension,
	Road,
	Wall,
	Rampart,
	KeeperLair,
	Portal,
	Controller,
	Link,
	Storage,
	Tower,
	Observer,
	PowerBank,
	PowerSpawn,
	Extractor,
	Lab,
	Terminal,
	Container,
	Nuker,
}

pub struct Structure {
	hits: u32,
	hits_max: u32,
	id: u32,
}

impl Structure {
	pub fn get_hits(&self) -> u32 {
		self.hits
	}
	pub fn get_hits_max(&self) -> u32 {
		self.hits_max
	}
	pub fn get_id(&self) -> u32 {
		self.id
	}
}

pub struct StructureSpawn {
	pub structure: Structure,
}

pub struct StructureExtension {
	pub structure: Structure,
}

pub struct StructureRoad {
	pub structure: Structure,
}

pub struct StructureWall {
	pub structure: Structure,	
}

pub struct StructureRampart {
	pub structure: Structure,
}

pub struct StructureKeeperLair {
	pub structure: Structure,
}

pub struct StructurePortal {
	pub structure: Structure,
}

pub struct StructureController {
	pub structure: Structure,
}

pub struct StructureLink {
	pub structure: Structure,
}

pub struct StructureStorage {
	pub structure: Structure,
}

pub struct StructureTower {
	pub structure: Structure,
}

pub struct StructureObserver {
	pub structure: Structure,
}

pub struct StructurePowerBank {
	pub structure: Structure,
}

pub struct StructurePowerSpawn {
	pub structure: Structure,
}

pub struct StructureExtractor {
	pub structure: Structure,
}

pub struct StructureLab {
	pub structure: Structure,
}

pub struct StructureTerminal {
	pub structure: Structure,
}

pub struct StructureContainer {
	pub structure: Structure,
}

pub struct StructureNuker {
	pub structure: Structure,
}