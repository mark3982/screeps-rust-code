var rust = require('rust.boot');

function avgk(k, v) {
	if (Memory[k] === undefined) {
		Memory[k] = v;
	} else {
		Memory[k] = (Memory[k] * 300.0 + v) / (301);
	}

	return Memory[k];
}

module.exports.loop = () => {
	rust.run();
	//equiv();
};

function equiv() {
	console.log('equiv-before-loop', avgk('equiv-before-loop', Game.cpu.getUsed()));
	let spawns = Game.rooms.E88S18.find(FIND_MY_STRUCTURES);

	let spawn = _.filter(spawns, s => s.structureType === STRUCTURE_SPAWN)[0];

	let cc = 0;

	for (let k in Game.creeps) {
		let c = Game.creeps[k];
		let cm = c.memory;

		++cc;

		let gathering = cm.gathering || 0;

		if (c.carry.energy === 0 && gathering === 0) {
			cm.gathering = 1;
			gathering = 1;
		}

		if (c.carry.energy === c.carryCapacity && gathering === 1) {
			cm.gathering = 0;
			gathering = 0;
		}

		if (gathering === 1) {
			var source = c.room.find(FIND_SOURCES)[1];
			if (c.harvest(source) === ERR_NOT_IN_RANGE) {
				c.moveTo(source);
			}
		} else {
			if (c.upgradeController(c.room.controller) === ERR_NOT_IN_RANGE) {
				c.moveTo(c.room.controller);
			}
		}
	}

	if (cc < 3) {
		spawn.createCreep([WORK, CARRY, MOVE]);
	}

	console.log('equiv-after-js', avgk('equiv-after-js', Game.cpu.getUsed()));
}