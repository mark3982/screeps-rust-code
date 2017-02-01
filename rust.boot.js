var Module = {};

var asm = require('rust.asm');
var asm_mem = require('rust.mem');

function avgk(k, v) {
	if (Memory[k] === undefined) {
		Memory[k] = v;
	} else {
		Memory[k] = (Memory[k] * 300.0 + v) / (301);
	}

	return Memory[k];
}

function invoke_i(index) {
  try {
    return Module["dynCall_i"](index);
  } catch(e) {
    if (typeof e !== 'number' && e !== 'longjmp') throw e;
    asm["setThrew"](1, 0);
  }
}

function invoke_ii(index,a1) {
  try {
    return Module["dynCall_ii"](index,a1);
  } catch(e) {
    if (typeof e !== 'number' && e !== 'longjmp') throw e;
    asm["setThrew"](1, 0);
  }
}

function invoke_vii(index,a1,a2) {
  try {
    Module["dynCall_vii"](index,a1,a2);
  } catch(e) {
    if (typeof e !== 'number' && e !== 'longjmp') throw e;
    asm["setThrew"](1, 0);
  }
}

function invoke_iii(index,a1,a2) {
  try {
    return Module["dynCall_iii"](index,a1,a2);
  } catch(e) {
    if (typeof e !== 'number' && e !== 'longjmp') throw e;
    asm["setThrew"](1, 0);
  }
}

function invoke_vi(index,a1,a2,a3) {
  try {
    Module["dynCall_vi"](index,a1,a2,a3);
  } catch(e) {
    if (typeof e !== 'number' && e !== 'longjmp') throw e;
    asm["setThrew"](1, 0);
  }
}

function invoke_vii(index,a1,a2,a3) {
  try {
    Module["dynCall_vii"](index,a1,a2,a3);
  } catch(e) {
    if (typeof e !== 'number' && e !== 'longjmp') throw e;
    asm["setThrew"](1, 0);
  }
}

function invoke_viii(index,a1) {
  try {
    Module["dynCall_viii"](index,a1);
  } catch(e) {
    if (typeof e !== 'number' && e !== 'longjmp') throw e;
    asm["setThrew"](1, 0);
  }
}

Module["dynCall_i"] = function() {
	return Module["asm"]["dynCall_i"].apply(null, arguments)
};

Module["dynCall_ii"] = function() {
	console.log(Module.asm);
	return Module["asm"]["dynCall_ii"].apply(null, arguments)
};

Module["dynCall_vii"] = function() {
	return Module["asm"]["dynCall_vii"].apply(null, arguments)
};

Module["dynCall_iii"] = function() {
	return Module["asm"]["dynCall_iii"].apply(null, arguments)
};

Module["dynCall_vi"] = function() {
	return Module["asm"]["dynCall_vi"].apply(null, arguments)
};

Module["dynCall_vii"] = function() {
	return Module["asm"]["dynCall_vii"].apply(null, arguments)
};

Module["dynCall_viii"] = function() {
	return Module["asm"]["dynCall_viii"].apply(null, arguments)
};

// Reuse the memory.
var memsize = 1024 * 1024 * 10;
var buf = new ArrayBuffer(memsize);
var u8 = new Uint8Array(buf);

console.log('filling memory with static data');
for (var q = 0; q < asm_mem.length; ++q) {
	u8[q] = asm_mem.charCodeAt(q);
}

module.exports = {};

var run_func = null;
var g_asm = null;
var g_u32 = null;

var id_to_object = {};
var unlocid = 0;

function get_id_for_object(obj) {
	var id = unlocid++;

	id_to_object[id] = obj;

	return id;
}

if (Game.cpu.bucket < 50) {
	console.log('cpu bucket too little');
	module.exports.run = function () { };
	return;
}

module.exports.run = function () {
	console.log('used-cpu-before-loop', avgk('before-loop', Game.cpu.getUsed()));

	id_to_object = {};
	unlocid = 0;

	let heapstart = 1024 * 1024 * 2;
	g_u32[0] = heapstart;
	g_u32[1] = ((memsize - heapstart) >> 2) << 2;	

	g_u32[g_u32[0] >> 2] = 0x2;
	g_u32[(g_u32[0] >> 2) + 1] = g_u32[1];

	// The following encodes the data so that Rust can read
	// and write it in native form instead of doing active
	// marshalling of data. This performs one large marshall
	// of the data.

	var creeps = Game.creeps;

	var cnt = 0;

	for (var k in creeps) {
		++cnt;
	}

	var gameobj = g_asm.___allocate(12);

	var data = g_asm.___allocate(cnt * 4 * 6, 1);

	//console.log('gameobj', gameobj);
	//console.log('data', data);
	//console.log('creeps.length', cnt);

	var tmp = gameobj >> 2;

	g_u32[tmp++] = data | 0;
	g_u32[tmp++] = cnt | 0;
	g_u32[tmp++] = cnt | 0;

	data = data >> 2;

	for (var q in creeps) {
		var creep = creeps[q];
		g_u32[data++] = get_id_for_object(creep) | 0;
		g_u32[data++] = creep.hits | 0;
		g_u32[data++] = creep.hitsMax | 0;
		g_u32[data++] = get_id_for_object(creep.room) | 0;
		g_u32[data++] = creep.carry.energy | 0;
		g_u32[data++] = creep.carryCapacity | 0;
	}

	console.log('used-cpu-before-rust', avgk('before-rust', Game.cpu.getUsed()));
	console.log('tick exit value', run_func(gameobj));
	console.log('used-cpu-after-rust', avgk('after-rust', Game.cpu.getUsed()));	
}

module.exports.setup = function (cb) {
	var glb = {
		Int32Array: Int32Array,
		Uint32Array: Uint32Array,
		Int8Array: Int8Array,
		Uint8Array: Uint8Array,
		Int16Array: Int16Array,
		Uint16Array: Uint16Array,
		Float32Array: Float32Array,
		Float64Array: Float64Array,
		Math: Math,
	};

	/*
		The memory layout is important.

		256b	- static global system parameters region
		2mb~	- stack region
		8mb~	- heap region

		Any special heap region is allocated and used
		from within the normal heap region. A special
		heap region can be populated by data loaded from
		the `Memory` object in ASCII/binary form and subsequently
		stored back by doing a byte for byte copy.
	*/

	var u32 = new Uint32Array(buf);
	var f32 = new Float32Array(buf);
	var u8 = new Uint8Array(buf);

	function getTotalMemory() {
		return memsize;
	}

	let heapstart = 1024 * 1024 * 2;

	function get_heap_region_off() {
		return heapstart;
	}

	function get_heap_region_size() {
		return ((memsize - heapstart) >> 2) << 2; 
	}

	let env = {
		STACKTOP: ((asm_mem.length >> 2) << 2) + 4,
		STACK_MAX: heapstart,
		invoke_i: invoke_i,
		invoke_ii: invoke_ii,
		invoke_iii: invoke_iii,
		invoke_iiii: invoke_viii,
		invoke_v: invoke_viii,
		invoke_vi: invoke_vi,
		invoke_vii: invoke_vii,
		invoke_viii: invoke_viii,
		invoke_viiii: invoke_viii,
		invoke_viiiii: invoke_viii,
		invoke_viiiiii: invoke_viii,
		getTotalMemory: getTotalMemory,
		_get_heap_region_off: get_heap_region_off,
		_get_heap_region_size: get_heap_region_size,
	};

	env.__debugmark = function (val) {
		console.log('debug-mark', val);
	}

	// Another quick workaround. It solves a problem that I do
	// not wish to currently spend time trying to rectify. This
	// is just a make it work hack.

	function build_with_env() {
		Module.asm = asm(glb, env, buf);

		return Module.asm;	
	}

	cb({
		heapstart: heapstart,
		u32: u32,
		u8: u8,
		asm: Module.asm,
		memsize: memsize,
		env: env,
		build_with_env: build_with_env,
	})
};

module.exports.setup(function (opts) {
	var heapstart = opts.heapstart;
	var u32 = opts.u32;
	var u8 = opts.u8;
	var asm = opts.asm;
	var env = opts.env;

	g_u32 = u32;

	var room_id_to_room = {};

	/*
		A unique local ID has the following properties:

		  * different every tick and invocation of this function
		  * this id can not be compared with others to 
		    determine if something is the same object
		  * ids only serve as a handle to the actual object
		  * to get a global unique object call the appropriate
		    function to convert the local unique id into one
		  * this id only serves to fit into a CPU register
	*/

	let real__rust_allocate = null;
	let real__rust_deallocate = null;

	env.___rust_allocate = function (a, b) {
		return real__rust_allocate(a, b);
	}

	env.___rust_deallocate = function (a, b) {
		return real__rust_deallocate(a, b);
	}

	env._read32 = function (addr) {
		return u32[addr >> 2];
	}

	env._write32 = function (addr, v) {
		u32[addr >> 2] = v;
	}

	env.__memory_get_integer = function (path_addr, path_size) {
		console.log('path_addr', path_addr);

		var q = (path_addr - 8) | 0;
		var p = [];

		for (let x = 0; x < path_size; ++x) {
			p.push(String.fromCharCode(u8[x+q]));
		}

		console.log('path_addr', p.join(''));

		//var v = eval('Memory.' + path);
		//console.log('v', v);
	}

	//pub fn _creep_mem_write(cid: u32, key: u32, data: *const u8, data_size: usize);
	//pub fn _creep_mem_read(cid: u32, key: u32, data: *const u8, data_size: usize);
	//pub fn creep_mem_key_exist(cid: u32, key: u32) -> bool;

	env.__creep_mem_write = function (cid, key, data_addr, data_size) {
		var c = id_to_object[cid];

		var s = [];

		for (let q = 0; q < data_size; ++q) {
			s.push(String.fromCharCode(u8[data_addr + q]));
		}

		console.log('__creep_mem_write', cid, key, data_addr, data_size, s);

		c.memory[key] = s.join('');

		return 1;
	}

	env.__creep_mem_read = function (cid, key, data_addr, data_size) {
		var c = id_to_object[cid];

		if (c.memory[key] === undefined) {
			return;
		}

		if (c.memory[key].length !== data_size) {
			return;
		}

		let m = c.memory[key];

		for (let q = 0; q < data_size; ++q) {
			u8[data_addr + q] = m.charCodeAt(q);
		}

		console.log('__creep_mem_read', cid, key, data_addr, data_size);

		return 1;
	}

	env._creep_mem_key_exist = function (cid, key, data_size) {
		var c = id_to_object[cid];

		console.log('c=', c);

		if (c.memory[key] === undefined) {
			return 0;
		}

		if (c.memory[key].length !== data_size) {
			return 0;
		}

		return 1;
	}

	env._spawn_build = function (bparts, bparts_count) {
		// Problem is will break on Rust ABI change.
		var tparts = [];
		for (var q = 0; q < bparts_count | 0; ++q) {
			var part = u8[bparts + q] | 0;
			switch (part) {
				case 0: tparts.push(WORK); break;
				case 1: tparts.push(CARRY); break;
				case 2: tparts.push(MOVE); break;
				case 3: tparts.push(ATTACK); break;
				case 4: tparts.push(RANGED_ATTACK); break;
				case 4: tparts.push(HEAL); break;
				case 4: tparts.push(CLAIM); break;
				default: throw Error('unknown part');
			}
		}

		console.log('tparts', tparts);

		// This is just a hack function. It is incomplete
		// but intended to do something minimal.
		var structs = Game.rooms.E88S18.find(FIND_MY_STRUCTURES);

		for (var q = 0; q < structs.length; ++q) {
			var s = structs[q];

			console.log('s', s);

			if (s.structureType === STRUCTURE_SPAWN) {
				console.log('YES??');
				if (s.spawning === null) {
					console.log('got spawn');
					return s.createCreep(tparts);
				}
			}
		}

		// Incorrect, but works for now.
		return -10;
	}

	env._creep_upgrade_controller = function (cid, tid) {
		console.log('_creep_upgrade_controller', cid, tid);
		return id_to_object[cid].upgradeController(id_to_object[tid]);
	}

	env._creep_harvest = function (cndx, tid) {
		let res = id_to_object[cndx].harvest(id_to_object[tid]);
		console.log('_creep_harvest', id_to_object[cndx], tid, res);
		return res;
	}

	env._creep_moveto = function (cid, tid) {
		console.log('_creep_moveto', tid, id_to_object[tid]);
		return id_to_object[cid].moveTo(id_to_object[tid]);
	}

	env._creep_move = function (cndx, dir) {
		console.log('creep move', cndx, dir, id_to_object[cndx]);
		id_to_object[cndx].move(dir);
	};		

	env._room_enumerate = function (rid, addr) {
		var room = id_to_object[rid];

		console.log('room enumerate', rid, room);

		var sources = room.find(FIND_SOURCES);

		var data;

		//throw Error('here:' + sources.length);

		data = env.___rust_allocate(sources.length * 4) >> 2;

		addr = addr >> 2;

		u32[addr++] = data << 2;
		u32[addr++] = sources.length;
		u32[addr++] = sources.length;
		u32[addr++] = get_id_for_object(room.controller);

		for (var q = 0; q < sources.length; ++q) {
			u32[data++] = get_id_for_object(sources[q]);
			u32[data++] = sources[q].energy;
			u32[data++] = sources[q].energyCapacity;
			u32[data++] = sources[q].ticksToRegenerate;
		}
	};

	asm = opts.build_with_env();

	real__rust_allocate = asm.___allocate;
	real__rust_deallocate = asm.___deallocate;

	g_asm = asm;
	run_func = asm._game_tick;
});
