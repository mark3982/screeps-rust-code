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
var g_room_enumerate_pointers = null;

var id_to_object = [];
var unlocid = 0;

function get_id_for_object(obj) {
	var id = id_to_object.length;
	id_to_object.push(obj);
	return id;
}

if (Game.cpu.bucket < 50) {
	console.log('cpu bucket too little');
	module.exports.run = function () { };
	return;
}

function room_name_to_guid32(rname) {
	var ew = rname[0];
	var ns = rname[3];
	var ew0 = rname.charCodeAt(1);
	var ew1 = rname.charCodeAt(2);
	var ns0 = rname.charCodeAt(4);
	var ns1 = rname.charCodeAt(5);

	var a = (ew0 - 48);
	var b = (ew1 - 48);
	var c = (ns0 - 48);
	var d = (ns1 - 48);

	a = a << (7 * 3);
	b = b << (7 * 2);
	c = c << (7 * 1);
	d = d;

	let e = ew == 'E' ? 0 : 1;
	let f = ns == 'N' ? 0 : 1;

	e = e << (7 * 4);
	f = f << (7 * 4 + 1);

	return a | b | c | d | e | f;
}

module.exports.run = function () {
	console.log('used-cpu-before-loop', avgk('before-loop', Game.cpu.getUsed()));

	id_to_object = [];
	unlocid = 0;
	g_room_enumerate_pointers = {};

	let heapstart = 1024 * 1024 * 2;

	g_u32[1] = heapstart;
	g_u32[2] = memsize - heapstart;

	g_u32[g_u32[1] >> 2] = 0x2;
	g_u32[(g_u32[1] >> 2) + 1] = g_u32[1];

	// The following encodes the data so that Rust can read
	// and write it in native form instead of doing active
	// marshalling of data. This performs one large marshall
	// of the data.

	var creeps = Game.creeps;

	var cnt = 0;

	for (var k in creeps) {
		++cnt;
	}

	let gameobj = g_asm.___allocate(12) | 0;
	let data = g_asm.___allocate(cnt * 4 * 8, 1) | 0;

	console.log('used-cpu-after-alloc', avgk('rust-after-alloc', Game.cpu.getUsed()));

	let tmp = (gameobj >> 2) | 0;

	g_u32[tmp++] = data | 0;
	g_u32[tmp++] = cnt | 0;
	g_u32[tmp++] = cnt | 0;

	data = (data | 0) >> 2;

	let id;

	// Option<creep::Creep>
	for (let q in creeps) {
		let creep = creeps[q];
		g_u32[data++] = 1;
		id = id_to_object.length | 0;
		id_to_object.push(creep);
		g_u32[data++] = id | 0;
		g_u32[data++] = creep.hits | 0;
		g_u32[data++] = creep.hitsMax | 0;
		// structure::Room
		id = id_to_object.length | 0;
		id_to_object.push(creep.room);		
		g_u32[data++] = id | 0;
		g_u32[data++] = 0; //room_name_to_guid32(creep.room.name) | 0;
		g_u32[data++] = creep.carry.energy | 0;
		g_u32[data++] = creep.carryCapacity | 0;
	}

	console.log('used-cpu-before-rust', avgk('before-rust', Game.cpu.getUsed()));
	run_func(gameobj);
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
		var q = (path_addr - 8) | 0;
		var p = [];

		for (let x = 0; x < path_size; ++x) {
			p.push(String.fromCharCode(u8[x+q]));
		}

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

		//console.log('__creep_mem_write', cid, key, data_addr, data_size, s);

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

		//console.log('__creep_mem_read', cid, key, data_addr, data_size);

		return 1;
	}

	env._creep_mem_key_exist = function (cid, key, data_size) {
		var c = id_to_object[cid];

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

		// This is just a hack function. It is incomplete
		// but intended to do something minimal.
		var structs = Game.rooms.E88S18.find(FIND_MY_STRUCTURES);

		for (var q = 0; q < structs.length; ++q) {
			var s = structs[q];

			if (s.structureType === STRUCTURE_SPAWN) {
				if (s.spawning === null) {
					return s.createCreep(tparts);
				}
			}
		}

		// Incorrect, but works for now.
		return -10;
	}

	env.__ZN4core9panicking5panic17hcb48c02563cd769eE = function () {
		throw Error('rust panic');
	}

	env._creep_upgrade_controller = function (cid, tid) {
		let st = Game.cpu.getUsed();
		let res = id_to_object[cid].upgradeController(id_to_object[tid]);
		return res;
	}

	env._creep_transfer = function (cndx, tid) {
		let res = id_to_object[cndx].transfer(id_to_object[tid], RESOURCE_ENERGY);
		return res;
	}

	env._creep_harvest = function (cndx, tid) {
		let res = id_to_object[cndx].harvest(id_to_object[tid]);
		return res;
	}

	env._creep_moveto = function (cid, tid) {
		let res = id_to_object[cid].moveTo(id_to_object[tid]);
		return res;
	}

	env._creep_move = function (cndx, dir) {
		id_to_object[cndx].move(dir);
	};		

	env._room_enumerate = function (rid, addr) {
		var room = id_to_object[rid];

		//console.log('room enumerate', rid, room);

		var data;

		if (g_room_enumerate_pointers[room.name] === undefined) {
			var sources = room.find(FIND_SOURCES);
			var structs = room.find(FIND_MY_STRUCTURES);

			let addr = env.___rust_allocate(4 * 7, 1); 
			let data = env.___rust_allocate(sources.length * 4 * 4, 1) >> 2;

			g_room_enumerate_pointers[room.name] = addr;

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

			let spawns_col = [];

			for (let q = 0; q < structs.length; ++q) {
				let s = structs[q];

				if (s.structureType === STRUCTURE_SPAWN) {
					spawns_col.push(s);
				}
			}

			data = env.___rust_allocate(3 * spawns_col.length, 1) >> 2;

			u32[addr++] = data << 2;
			u32[addr++] = spawns_col.length;
			u32[addr++] = spawns_col.length;

			for (let q = 0; q < spawns_col.length; ++q) {
				let s = spawns_col[q];
				u32[data++] = get_id_for_object(s);
				u32[data++] = s.hits;
				u32[data++] = s.hitsMax;
			}

		}
		
		return g_room_enumerate_pointers[room.name];
	};

	asm = opts.build_with_env();

	real__rust_allocate = asm.___allocate;
	real__rust_deallocate = asm.___deallocate;

	g_asm = asm;
	run_func = asm._game_tick;
});
