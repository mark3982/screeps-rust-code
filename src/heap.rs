use super::ffi;

/// Sets the region for the heap functions.
#[no_mangle]
pub extern fn __heap_region(addr: usize, size: usize) {
	unsafe {
		ffi::write32(0, addr as u32);
		ffi::write32(4, size as u32);
	}
}

#[cfg(not(target_os = "emscripten"))]
pub unsafe fn __allocate(size: usize, _align: usize) -> *mut u8 {
	extern crate alloc;
	alloc::heap::allocate(size, _align)
}

/// Returns the address of a chunk of memory that is of at least the size
/// specified.
///
/// This function is designed to have its region specified. This allocator
/// is a balance between performance and code size. 
///
/// The intention is that there is an abundance of RAM and that each invocation
/// of the code is going to only use the heap for a few rounds before it is
/// destroyed, therefore, no work is done to reduce heap segmentation.
#[no_mangle]
#[cfg(target_os = "emscripten")]
pub extern fn __allocate(mut size: usize, _align: usize) -> *mut u8 {
	unsafe {
		// TODO: implement alignment by overallocation if needed
		let regionoff: u32 = ffi::read32(4);
		let regionsize: u32 = ffi::read32(8);

		let mut loff = 0;

		// _Must_ have 4 byte alignment for chunks. By aligning
		// the chunk size it also aligns the offset of every allocation
		// since the headers are in four byte multiples.
		if size & 3 > 0 {
			size = ((size >> 2) << 2) + 4;
		}

		while true {
			let chunkflags: u32 = ffi::read32((regionoff + 0 + loff) as usize);
			let chunksize: u32 = ffi::read32((regionoff + 4 + loff) as usize);

			//debugmark(99);
			//debugmark(chunkflags as usize);
			//debugmark(chunksize as usize);

			if chunkflags & (1 as u32) == 0 && chunksize as u32 >= size as u32 {
				// The chunk is free.
				if chunksize as u32 > size as u32 + 32u32 {
					// Use a portion of the chunk.
					ffi::write32((regionoff + loff) as usize, 1u32);
					ffi::write32((regionoff + 4u32 + loff) as usize, size as u32);
					// Write the next header as unused and copy the last chunk bit flag.
					ffi::write32((regionoff + loff + 8u32 + size as u32 + 0u32) as usize, chunkflags & 2 as u32);
					ffi::write32((regionoff + loff + 8u32 + size as u32 + 4u32) as usize, (chunksize - size as u32 - 8 as u32) as u32);
				} else {
					// Use the entire chunk, therefore, leave everything as is. Also,
					// copy the last chunk bit flag.
					ffi::write32((regionoff + loff) as usize, (chunkflags & 2 as u32) | 1 as u32);
				}

				// Skip both flags and size fields.
				return (regionoff + 8 + loff) as *mut u8;
			}

			if chunkflags & (2 as u32) == 2 {
				//// The last chunk has been scanned.
				break;
			}

			loff = loff + 8 + chunksize;
		}
	}
	
	0 as *mut u8
}

#[no_mangle]
#[cfg(not(target_os = "emscripten"))]
pub unsafe fn __deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
	extern crate alloc;
	alloc::heap::deallocate(ptr, _old_size, _align)
}

#[no_mangle]
#[cfg(target_os = "emscripten")]
pub extern fn __deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
	let hdr = ptr as usize - 8;
	unsafe {
		let flags = ffi::read32(hdr);

		// Copy the last-chunk flag but clear the used flag.
		ffi::write32(hdr, flags & 2);
	}
}

/// Copies the specified number of bytes from source to destination.
///
/// A version of memcpy to reduce the amount of emitted code. A balance
/// between performance and code size.
#[cfg(target_os = "emscripten")]
unsafe fn memcpy(dst: *mut u8, src: *mut u8, size: usize) {
	let chunks = size / 4;
	let slack = size - (chunks * 4);
	let q = 0usize;

	for q in 0..chunks {
		unsafe {
			ffi::write32(dst as usize + q * 4, ffi::read32(src as usize + q * 4))
		}
	}

	let off = chunks * 4;

	for q in 0..slack {
		unsafe {
			ffi::write8(dst as usize + off + q, ffi::read8(src as usize + off + q));
		}
	}
}

#[no_mangle]
#[cfg(target_os = "emscripten")]
pub extern fn __reallocate(ptr: *mut u8, _old_size: usize, size: usize, _align: usize) -> *mut u8 {
	__deallocate(ptr, _old_size, _align);

	let nptr = __allocate(size, _align);

	if nptr as usize == 0 {
		0 as *mut u8
	} else {
		if size > _old_size {
			unsafe { memcpy(nptr, ptr, _old_size); }
		} else {
			unsafe { memcpy(nptr, ptr, size); }
		}

		nptr
	}
}