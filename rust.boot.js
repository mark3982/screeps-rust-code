var Module = {};

var asm = require('rust.asm');

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
var buf = new ArrayBuffer(1024 * 1024 * 10);

module.exports = {};

module.exports.run = function () {
	var glb = {
		Int32Array: Int32Array,
		Int8Array: Int8Array,
		Math: Math,
	};

	var heapstart = 1024 * 1024 * 5;

	var env = {
		STACKTOP: 0,
		STACK_MAX: heapstart,
		invoke_i: invoke_i,
		invoke_ii: invoke_ii,
		invoke_iii: invoke_iii,
		invoke_vi: invoke_vi,
		invoke_vii: invoke_vii,		
	};


	/*
		This set of functions exposes Screeps to the Rust
		runtime. For Rust, it expects these functions to
		handle raw references and values. The Rust code
		shall incorporate protections to ensure that the 
		proper type of value is passed to these functions.
	*/
	env._creep_move = function (cndx, dir) {
		console.log('creep move', cndx, dir, creep_index_to_creep[cndx]);
		creep_index_to_creep[cndx].move(dir);
	};


	Module.asm = asm(glb, env, buf);

	// Need to encode the game state as the actual objects
	// in a binary form which can be interpreted by the Rust
	// code.

	var u32 = new Uint32Array(buf);
	var f32 = new Float32Array(buf);

	var creeps = Game.creeps;

	var ptr_cur = heapstart;

	var creep_index_to_creep = {};

	function load_creeps_into_memory(creeps) {
		var ndx = ptr_cur >> 2;

		u32[ndx++] = ptr_cur + 12;

		var szp = ndx++;

		ndx++;

		var cnt = 0;

		for (var k in creeps) {
			var c = creeps[k];

			creep_index_to_creep[cnt] = c;

			u32[ndx++] = cnt;
			u32[ndx++] = c.hits;
			u32[ndx++] = c.hitsMax; 

			++cnt;
		}

		// There is actually a RawVec and a Vec.
		// See the Rust stdlib source code.
		u32[szp++] = cnt | 0;
		u32[szp++] = cnt | 0;
	}

	load_creeps_into_memory(creeps);


	console.log('tick exit value', Module.asm._game_tick(heapstart));
};