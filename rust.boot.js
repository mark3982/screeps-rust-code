var Module = {};

var asm = require('/home/kmcguire/screeps/rust/output/rust.asm.js');

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

function invoke_vi(index,a1) {
  try {
    Module["dynCall_vi"](index,a1);
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

// Reuse the memory.
var memsize = 1024 * 1024 * 10;
var buf = new ArrayBuffer(memsize);

module.exports = {};

module.exports.run = function () {
	module.exports.setup(function (opts) {
		var heapstart = opts.heapstart;
		var u32 = opts.u32;
		var u8 = opts.u8;
		var asm = opts.asm;

		var creep_index_to_creep = {};

		env._creep_move = function (cndx, dir) {
			console.log('creep move', cndx, dir, creep_index_to_creep[cndx]);
			creep_index_to_creep[cndx].move(dir);
		};		

		// The following encodes the data so that Rust can read
		// and write it in native form instead of doing active
		// marshalling of data. This performs one large marshall
		// of the data.

		//var creeps = Game.creeps;
		var creeps = {
			John: { hits: 100, hitsMax: 200 },
			Mark: { hits: 200, hitsMax: 300 },
		};

		var ptr_cur = heapstart;

		var creep_index_to_creep = {};

		function make_struct_vector(ptr, ary, cb) {
			var ndx = 0;

			u32[ptr++] = (ptr << 2) + 12;

			var szp = ptr;

			for (var k in ary) {
				ptr = cb(ptr, ary[k], ndx);
				++ndx;
			}

			// The RawVec also has a capacity field.
			u32[szp++] = ndx;
			u32[szp++] = ndx;

			return ptr;
		}

		function make_struct_creep(ptr, creep, ndx) {
			u32[ptr++] = ndx;
			u32[ptr++] = creep.hits;
			u32[ptr++] = creep.hitsMax;
			return ptr;
		}

		// Write the game structure into the heap memory.
		ptr_cur = make_struct_vector(
			ptr_cur >> 2, 
			creeps, 
			function (ptr, creep, ndx) {
				creep_index_to_creep[ndx] = creep;
				return make_struct_creep(ptr, creep, ndx);
			}
		) << 2;

		u32[0] = (ptr_cur >> 2) << 2;
		u32[1] = ((memsize - ptr_cur) >> 2) << 2;

		// Initialize the heap region to have one free chunk.
		u32[u32[0] >> 2] = 0x2;
		u32[(u32[0] >> 2) + 1] = u32[1];	

		console.log('tick exit value', asm._game_tick(heapstart));
	});
}

module.exports.setup = function (cb) {
	var glb = {
		Int32Array: Int32Array,
		Int8Array: Int8Array,
		Math: Math,
	};

	var heapstart = 1024 * 1024 * 2;

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

	var env = {
		STACKTOP: 256,
		STACK_MAX: heapstart,
		invoke_i: invoke_i,
		invoke_ii: invoke_ii,
		invoke_iii: invoke_iii,
		invoke_vi: invoke_vi,
		invoke_vii: invoke_vii,
		___rust_allocate: null,
	};

	// This was a workaround for what I believe Emscripten was emitting
	// an llvm_trap call. It did not like writes to the heap.. It did allow
	// writes with a constant address but anything else emitted the llvm
	// trap call. I hope until that is fixed that V8 can optimize this by
	// inlining the call where it is used or it does not slow things down
	// much. The read32 function simply makes the code look cleaner.
	env._write32 = function (addr, val) {
		// Force alignment.
		//console.log('write32', addr, val);
		u32[addr >> 2] = val;
	}

	env._read32 = function (addr) {
		// Force alignment.
		//console.log('read32', addr, u32[addr >> 2]);
		return u32[addr >> 2];
	}

	env._write8 = function (addr, val) {
		// Force alignment.
		u8[addr] = val;
	}

	env._read8 = function (addr) {
		// Force alignment.
		return u8[addr];
	}

	env._debugmark = function (val) {
		console.log('debug-mark', val);
	}

	// Another quick workaround. It solves a problem that I do
	// not wish to currently spend time trying to rectify. This
	// is just a make it work hack.
	Module.asm = asm(glb, env, buf);
	env.___rust_allocate = Module.asm.___allocate;
	env.___rust_deallocate = Module.asm.___deallocate;
	Module.asm = asm(glb, env, buf);

	cb({
		heapstart: heapstart,
		u32: u32,
		u8: u8,
		asm: Module.asm,
		memsize: memsize,
		env: env,
	})
};