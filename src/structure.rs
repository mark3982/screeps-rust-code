use super::Role;
use super::SubRole;
use super::ActionResult;
use super::ffi;
use super::room::RoomGUID;

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

#[derive(Clone)]
#[repr(C)]
pub struct Structure {
	pub id: u32,
	pub hits: u32,
	pub hits_max: u32,
}

#[derive(Clone)]
#[repr(C)]
pub struct Spawn {
	pub structure: Structure,
	pub energy: u32,
	pub energy_capacity: u32,
	pub spawning: bool,
}

#[repr(C)]
pub struct SpawnCreepSpec {
	pub work: u8,
	pub carry: u8,
	pub attack: u8,
	pub rattack: u8,
	pub heal: u8,
	pub claim: u8,
	pub tough: u8,
	pub moves: u8,
	pub role: Role,
	pub subrole: SubRole,
	pub room_guid: RoomGUID,
}

impl Spawn {
	pub fn create_creep(&self, spec: &SpawnCreepSpec) -> ActionResult {
		ActionResult::from_integer(
			unsafe {
				ffi::create_creep(self.structure.id, spec)
			}
		)
	}
}

#[derive(Clone)]
#[repr(C)]
pub struct Extension {
	pub structure: Structure,
	pub energy: u32,
	pub energy_capacity: u32,
}

#[derive(Clone)]
#[repr(C)]
pub struct Road {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Wall {
	pub structure: Structure,	
}

#[derive(Clone)]
#[repr(C)]
pub struct Rampart {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct KeeperLair {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Portal {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Controller {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Link {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Storage {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Tower {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Observer {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct PowerBank {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct PowerSpawn {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Extractor {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Lab {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Terminal {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Container {
	pub structure: Structure,
}

#[derive(Clone)]
#[repr(C)]
pub struct Nuker {
	pub structure: Structure,
}