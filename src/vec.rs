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

impl<T: PartialOrd> Vec<T> {
	pub fn sort(&mut self) {
		for q in 1..self.len() {
			let mut a = unsafe { self.yank(q) }; // 4

			for w in 0..q {
				let mut b = unsafe { self.yank(w) }; // 5

				if a < b {
					core::mem::swap(&mut a, &mut b);
				}
				
				unsafe { self.stuff(w, b); }
			}

			unsafe { self.stuff(q, a); }
		}
	}
}

impl<T: Clone> Vec<T> {
	pub fn clone(&self) -> Vec<T> {
		let mut out: Vec<T> = Vec::with_capacity(self.capacity);

		for q in 0..self.len() {
			out.push(self[q].clone());
		}

		out
	}
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

	pub fn pop(&mut self) -> T {
		if self.count == 0 {
			ffi::panic();
		}

		self.count -= 1;

		let count = self.count;

		unsafe {
			self.yank(count)
		}
	}

	/// Places the value at the index specified _but_ does not
	/// deallocate any existing value. Existing values are ignored
	/// therefore this function is unsafe.
	pub unsafe fn stuff(&mut self, ndx: usize, v: T) {
		let is = core::mem::size_of::<T>();
		let addr = self.ptr + is * ndx;

		unsafe {
			*(addr as *mut T) = v;
		}		
	}

	/// Removes a value from the index specified placing all zeros
	/// at that location and does not deinitialize the value removed.
	/// The value removed is returned; however, the all zero byte 
	/// value is dangerously uninitialized thus this function is unsafe.
	pub unsafe fn yank(&mut self, ndx: usize) -> T {
		let is = core::mem::size_of::<T>();
		let addr = self.ptr + is * ndx;

		unsafe {
			let mut out: T = core::mem::zeroed();
			core::mem::swap(core::mem::transmute::<usize, &mut T>(addr), &mut out);
			out
		}
	}
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
	fn index_mut<'a>(&'a mut self, ndx: usize) -> &'a mut T {
		if ndx >= self.count {
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
		if ndx >= self.count {
			ffi::panic();
		}

		let is = core::mem::size_of::<T>();
		let addr = self.ptr + is * ndx;

		unsafe {
			core::mem::transmute::<usize, &T>(addr)
		}
	} 
}