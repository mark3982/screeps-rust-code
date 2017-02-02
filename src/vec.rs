/// Static sized vector. Will not automatically resize. Will panic
/// if more items than capacity are pushed and shall panic if bad
/// index is accessed.
///
/// Use DyVec for a dynamic sized vector.
use core;
use super::ffi;
use super::heap;

pub struct Vec<T> {
	ptr: usize,
	capacity: usize,
	count: usize,
	phantom: core::marker::PhantomData<T>,
}

impl<T> Vec<T> {
	pub fn with_capacity(count: usize) -> Vec<T> {
		unsafe {
			let ptr = heap::__allocate(
				core::mem::size_of::<T>() * count,
				1
			);

			Vec {
				ptr: ptr as usize,
				capacity: count,
				count: 0,
				phantom: core::marker::PhantomData,
			}
		}
	}

	pub fn len(&self) -> usize {
		self.count
	}

	pub fn push(&mut self, v: T) {
		if self.count == self.capacity {
			ffi::panic();
		}

		let is = core::mem::size_of::<T>();
		let addr = self.ptr + is * self.count;

		unsafe {
			*(addr as *mut T) = v;
		}

		self.count = self.count + 1;
	}	
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
	fn index_mut<'a>(&'a mut self, ndx: usize) -> &'a mut T {
		if (ndx >= self.count) {
			ffi::panic();
		}

		let is = core::mem::size_of::<T>();
		let addr = self.ptr + is * ndx;
		
		unsafe {
			core::mem::transmute::<usize, &mut T>(addr)
		}		
	}
}

impl<T> core::ops::Index<usize> for Vec<T> {
	type Output = T;

	fn index(&self, ndx: usize) -> &T {
		if (ndx >= self.count) {
			ffi::panic();
		}

		let is = core::mem::size_of::<T>();
		let addr = self.ptr + is * ndx;

		unsafe {
			core::mem::transmute::<usize, &T>(addr)
		}
	} 
}