/// Provides automation of spawner control and a paradigm of accessing creeps.
///
use super::Game;
use super::creep::Creep;
use super::Role;
use super::SubRole;
use super::vec::Vec;
use super::Memkey;
use super::room;
use super::room::Room;
use super::structure;
use super::ffi;
use core::intrinsics::ceilf32;
use core::cmp::{PartialOrd, PartialEq, Ordering};

#[derive(Copy, Clone)]
pub struct BodyPartSpec {
	pub work: u8,
	pub carry: u8,
	pub claim: u8,
	pub attack: u8,
	pub rattack: u8,
	// The desired number of heal bodyparts.
	pub heal: u8,
	// The desired number of tough bodyparts.
	pub tough: u8,
	/// If the creep can be off-road.
	pub offroad: bool,
	/// The maximum number to build for this order.
	pub num: u8,
}

impl BodyPartSpec {
	pub fn move_parts(&self) -> u32 {
		let tparts = self.total_parts_without_move();
		if self.offroad {
			tparts
		} else {
			unsafe { ceilf32(tparts as f32 * 0.5) as u32 }
		}		
	}

	pub fn total_parts_without_move(&self) -> u32 {
		self.work as u32 + 
		self.carry as u32 + 
		self.claim as u32 + 
		self.attack as u32 + 
		self.rattack as u32 + 
		self.heal as u32 + 
		self.tough as u32
	}

	pub fn total_parts(&self) -> u32 {
		let tparts = self.total_parts_without_move();
		if self.offroad == true {
			tparts * 2
		} else {
			unsafe { ceilf32(tparts as f32 * 0.5) as u32 + tparts }
		}			
	}
	
	pub fn build_time(&self) -> u32 {
		self.total_parts() * 3
	}

	pub fn cost(&self) -> u32 {
		let mparts = self.move_parts();
		let bcost = self.work as u32 * 100 + 
					self.carry as u32 * 50 +
					self.attack as u32 * 80 +
					self.rattack as u32 * 150 +
					self.heal as u32 * 250 +
					self.claim as u32 * 600 + 
					self.tough as u32 * 10;
		bcost + mparts * 50		
	}

	pub fn multiply(&mut self, x: f32) {
		let mut work = self.work as f32 * x;
		let mut carry = self.carry as f32 * x;
		let mut attack = self.attack as f32 * x;
		let mut rattack = self.rattack as f32 * x;
		let mut heal = self.heal as f32 * x;
		let mut claim = self.claim as f32 * x;
		let mut tough = self.tough as f32 * x;

		if work < 0f32 {
			work = 0.0;
		}

		if carry < 0f32 {
			carry = 0.0;
		}

		if attack < 0f32 {
			attack = 0.0;
		}

		if rattack < 0f32 {
			rattack = 0.0;
		}

		if heal < 0f32 {
			heal = 0.0;
		}

		if claim < 0f32 {
			claim = 0.0;
		}

		if tough < 0f32 {
			tough = 0.0;
		}

		if self.work > 0 && work < 1f32 {
			work = 1.0;
		}

		if self.carry > 0 && carry < 1f32 {
			carry = 1.0;
		}

		if self.attack > 0 && attack < 1f32 {
			attack = 1.0;
		}

		if self.rattack > 0 && rattack < 1f32 {
			rattack = 1.0;
		}

		if self.heal > 0 && heal < 1f32 {
			heal = 1.0;
		}

		if self.claim > 0 && claim < 1f32 {
			claim = 1.0;
		}

		if self.tough > 0 && tough < 1f32 {
			tough = 1.0;
		}

		self.work = work as u8;
		self.carry = carry as u8;
		self.attack = attack as u8;
		self.rattack = rattack as u8;
		self.heal = heal as u8;
		self.claim = claim as u8;
		self.tough = tough as u8;		
	}		

	pub fn least_but_not_zero(&self) -> u8 {
		let mut a = 255;

		if self.work > 0 && self.work < a {
			a = self.work;
		}

		if self.carry > 0 && self.carry < a {
			a = self.carry;
		}

		if self.attack > 0 && self.attack < a {
			a = self.attack;
		}

		if self.rattack > 0 && self.rattack < a {
			a = self.rattack;
		}

		if self.heal > 0 && self.heal < a {
			a = self.heal
		}

		if self.claim > 0 && self.claim < a {
			a = self.claim
		}

		if self.tough > 0 && self.tough < a {
			a = self.tough
		}		

		a
	}

	/// Using a build specification determine the realistic number of 
	/// bodyparts kept in proportion by the avaliable energy and also
	/// determine the number of creeps needed to equal the request if
	/// a single creep can not meet the specification.
	///
	/// The energy parameter dictates what the build is reduced by.
	///
	pub fn limit_by_energy(&self, energy: u32) -> BodyPartSpec {
		let mut n = *self;

		let min = n.least_but_not_zero() - 1;

		if n.work > 0 { n.work -= min; }
		if n.carry > 0 { n.carry -= min; }
		if n.attack > 0 { n.attack -= min; }
		if n.rattack > 0 { n.rattack -= min; }
		if n.heal > 0 { n.heal -= min; }
		if n.claim > 0 { n.claim -= min; }
		if n.tough > 0 { n.tough -= min; }

		let cost = n.cost() as f32;
		let ratio = energy as f32 / cost;

		n.multiply(ratio);

		let mut cnt = 1.0f32;

		// The limiter may have reduced the creep, therefore,
		// compute how many creeps we need to equal close as
		// possible to the original specified creep.
		if self.work as f32 / n.work as f32 > cnt {
			cnt = self.work as f32 / n.work as f32;
		}

		if self.carry as f32 / n.carry as f32 > cnt {
			cnt = self.carry as f32 / n.carry as f32;
		}

		if self.attack as f32 / n.attack as f32 > cnt {
			cnt = self.attack as f32 / n.attack as f32;
		}

		if self.rattack as f32 / n.rattack as f32 > cnt {
			cnt = self.rattack as f32 / n.rattack as f32;
		}

		if self.heal as f32 / n.heal as f32 > cnt {
			cnt = self.heal as f32 / n.heal as f32;
		}

		if self.claim as f32 / n.claim as f32 > cnt {
			cnt = self.claim as f32 / n.claim as f32;
		}

		if self.tough as f32 / n.tough as f32 > cnt {
			cnt = self.tough as f32 / n.tough as f32;
		}

		if cnt > 255.0 {
			cnt = 255.0;
		}

		n.num = cnt as u8;

		// Limit the number of creeps as specified.
		if n.num > self.num {
			n.num = self.num;
		}

		n
	}		
}

#[derive(Copy, Clone)]
pub struct BuildOrder {
	pub spec: BodyPartSpec,
	pub role: Role,
	pub subrole: SubRole,
	pub priority: f32,
	pub ttl: u32,
}

impl PartialEq for BuildOrder {
	fn eq(&self, other: &BuildOrder) -> bool {
		self.priority == other.priority
	}
}

impl PartialOrd for BuildOrder {
	fn partial_cmp(&self, other: &BuildOrder) -> Option<Ordering> {
		if self.priority == other.priority {
			Option::Some(Ordering::Equal)
		} else if self.priority > other.priority {
			Option::Some(Ordering::Greater)
		} else {
			Option::Some(Ordering::Less)
		}
	}

	fn lt(&self, other: &BuildOrder) -> bool {
		if self.priority < other.priority {
			true
		} else {
			false
		}
	}

	fn le(&self, other: &BuildOrder) -> bool {
		if self.priority <= other.priority {
			true
		} else {
			false
		}	
	}

	fn gt(&self, other: &BuildOrder) -> bool {
		if self.priority > other.priority {
			true
		} else {
			false
		}
	}

	fn ge(&self, other: &BuildOrder) -> bool {
		if self.priority >= other.priority {
			true
		} else {
			false
		}
	}	
}

pub struct Spawner {
	room: Room,
	bque: Vec<BuildOrder>,
	best: bool,
}

/// The `Spawner` is room based. It recieves requests for creeps and
/// either returns the creeps or queues for the building of them. It
/// may return zero or more creeps per requests. 
/// 
/// #Useful Mechanic:
/// The returned creeps
/// are no longer accessible via the `Game` object unless they were 
/// copied undestructively beforehand.
///
/// 
impl Spawner {
	/// Return a new spawner object with the room specified as an
	/// argument.
	pub fn new(room: Room) -> Spawner {
		Spawner {
			bque: Vec::with_capacity(255),
			best: true,
			room: room,
		}
	}
	/// Iterate the build orders in priority order using the TTL and
	/// build time to determine if a new creep shall be built while
	/// also ensuring that no higher level priority build order will
	/// be blocked. Uses a simple heuristic to estimate energy refill
	/// time for the spawn and extensions.
	pub fn action(&mut self, game: &mut Game) {
		use structure::SpawnCreepSpec;

		// Order the queue by priority.
		self.bque.sort();

		// Transverse the queue and find the build order that
		// needs to be built.
		let mut blocked = false;
		for q in 0..self.bque.len() {
			let bo = self.bque[q];
			let bt = bo.spec.build_time() as f32;

			if (bo.ttl as f32) < bt * 1.3 {
				// See if this will block anything else.
				for w in 0..self.bque.len() {
					let bo2 = self.bque[q];

					// Same priority or less priority can not block
					// another.
					if bo2.priority >= bo.priority {
						continue;
					}

					if (bt + (bo2.spec.build_time()) as f32) * 1.6 > bo2.ttl as f32 {
						blocked = true;
						break;
					}
				}

				if blocked {
					break;
				}

				// IF no blocks then try to build it, then exit the 
				// loop since this blocks all below.
				let res = self.get_spawn_energy_and_free_spawn();
				match res.2 {	
					Option::None => (),
					Option::Some(spawn) => {
						spawn.create_creep(&SpawnCreepSpec {
							work: bo.spec.work,
							carry: bo.spec.carry,
							attack: bo.spec.attack,
							rattack: bo.spec.rattack,
							heal: bo.spec.heal,
							claim: bo.spec.claim,
							tough: bo.spec.tough,
							moves: bo.spec.move_parts() as u8,
							role: bo.role,
							subrole: bo.subrole,
							room_guid: self.room.guid,
						});
					},
				};
				break;
			}
		}
		//
	}

	pub fn require_worst(&mut self) {
		self.best = false;
	}

	pub fn require_best(&mut self) {
		self.best = true;
	}

	pub fn get_spawn_energy_and_free_spawn(&self) -> (u32, u32, Option<structure::Spawn>) {
		use core;

		let renum = self.room.get_enum();

		let mut spawn: Option<structure::Spawn> = Option::None;

		let mut energy_capacity: u32 = 0;
		let mut energy_ava: u32 = 0;

		for q in 0..renum.spawns.len() {
			let spawn_ref = &renum.spawns[q];

			energy_capacity += spawn_ref.energy_capacity;
			energy_ava += spawn_ref.energy;

			if spawn_ref.spawning == false {
				spawn = Option::Some((*spawn_ref).clone());
			}
		}

		for q in 0..renum.extensions.len() {
			let ext_ref = &renum.extensions[q];

			energy_capacity += ext_ref.energy_capacity;
			energy_ava += ext_ref.energy;
		}

		(energy_capacity, energy_ava, spawn)
	}

	pub fn add_build_queue_order(&mut self, bo: BuildOrder, existing: &Vec<usize>, creeps: &Vec<Option<Creep>>) {
		let renum = self.room.get_enum();

		let res = self.get_spawn_energy_and_free_spawn();

		let spawn = res.2;
		let energy_capacity = res.0;
		let energy_ava = res.1;

		let nspec;

		if self.best {
			nspec = bo.spec.limit_by_energy(energy_capacity);
		} else {
			nspec = bo.spec.limit_by_energy(energy_ava);
		}

		// The existing must be ordered from highest TTL to the
		// lowest TTL since that will be associated with the build
		// orders that are about to be placed.
		let mut ndx: Vec<u32> = Vec::with_capacity(existing.len());

		for q in 0..existing.len() {
			let c = creeps[existing[q]].as_ref().unwrap();
			if c.spawning {
				// Put spawning status creeps at the top of the list after sort.
				ndx.push(c.ticks_to_live | 0x10000);
			} else {
				// Put non-spawning creeps at the bottom after sort.
				ndx.push(c.ticks_to_live);
			}
		}

		ndx.sort();

		let ndxlast = ndx.len() - 1;

		for q in 0..nspec.num as usize {
			let ttl = if q >= ndx.len() {
				0
			} else {
				ndx[ndxlast - q] & 0xFFFF
			};

			self.bque.push(BuildOrder {
				spec: nspec,
				role: bo.role,
				subrole: bo.subrole,
				priority: bo.priority,
				ttl: ttl,
			});
		}
	}

	/// Return the creep matching the home, role, and subrole while attemping to meet
	/// the creep specification. If one or more creeps exist they are returned and if 
	/// one or more needs to be built they are queued for building for _only_ this tick.
	///
	/// May return zero or more creeps.
	pub fn get_creep(&mut self, game: &mut Game, spec: BodyPartSpec, role: Role, subrole: SubRole, build_priority: f32) -> Vec<Creep> {
		let mut ndx: Vec<usize> = Vec::with_capacity(spec.num as usize);

		for q in 0..game.creeps.len() {
			if game.creeps[q].is_none() {
				continue;
			}

			let creep = game.creeps[q].as_ref().unwrap();

			let tmp = creep.mem_read::<room::RoomGUID>(Memkey::Room).unwrap_or(room::RoomGUID::None);

			if creep.mem_read::<room::RoomGUID>(Memkey::Room).unwrap_or(room::RoomGUID::None) == self.room.guid {
				if creep.mem_read::<Role>(Memkey::Role).unwrap_or(Role::None) == role {
					if creep.mem_read::<SubRole>(Memkey::SubRole).unwrap_or(SubRole::None) == subrole {
						ndx.push(q);
						break;
					}
				}
			}
		}

		self.add_build_queue_order(BuildOrder {
			spec: spec, 
			role: role, 
			subrole: subrole, 
			priority: build_priority, 
			ttl: 0
		}, &ndx, &game.creeps);

		if ndx.len() == 0 {
			Vec::with_capacity(0)
		} else {
			let mut out: Vec<Creep> = Vec::with_capacity(ndx.len());

			while ndx.len() > 0 {
				out.push(game.creeps[ndx.pop()].take().unwrap());
			}

			out
		}
	}
}