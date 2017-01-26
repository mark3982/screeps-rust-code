var rust = require('./rust.boot.js');


rust.setup(function (opts) {
	var asm = opts.asm;
	var u32 = opts.u32;
	var u8 = opts.u8;
	var heapstart = opts.heapstart;
	var memsize = opts.memsize;
	var ptr_cur = heapstart;

	u32[0] = (ptr_cur >> 2) << 2;
	u32[1] = ((memsize - ptr_cur) >> 2) << 2;

	// Initialize the heap region to have one free chunk.
	u32[u32[0] >> 2] = 0x2;
	u32[(u32[0] >> 2) + 1] = u32[1];


	// Do a light test of the heap.
	heap_test(asm, 1024 * 10);

}	

/**
	Do a stress test of the heap in an attempt to break it. This
	should help to reveal the existance of a bug in the heap code;
	however, it is not deterministic.
*/
function heap_test(asm, itercnt) {
	var used = [];

	function check_alloc_no_overlap(head, tail) {
		for (var w = 0; w < used.length; ++w) {
			if (head >= used[w][0] && head < used[w][0] + used[w][1]) {
				console.log('overlap', used[w][0], used[w][1]);
				return false;
			}

			if (tail >= used[w][0] && tail < used[w][0] + used[w][1]) {
				console.log('overlap', used[w][0], used[w][1]);
				return false;
			}
		}

		return true;
	}

	for (var q = 0; q < itercnt; ++q) {
		var allocsz = Math.floor(Math.random() * 256);

		console.log('@@@');
		var addr = asm.___allocate(allocsz, 1);

		if (addr !== 0) {
			console.log('__allocate check', addr, allocsz);

			// Ensure no overlap.
			if (check_alloc_no_overlap(addr, addr + allocsz) === false) {
				throw Error('alloc failed with overlap');
			}
		} else {
			console.log('OOM; nice');
		}

		// Delete a random chunk.
		if (Math.random() > 0.8) {
			var w = Math.floor(Math.random() * (used.length - 1));

			console.log('__deallocate', used[w][0], used[w][1]);
			asm.___deallocate(used[w][0], used[w][1], 1)

			var tmp = [];

			for (var z = 0; z < used.length; ++z) {
				if (z != w) {
					tmp.push(used[z]);
				}
			}

			used = tmp;
		}

		used.push([addr, allocsz])
	}
});