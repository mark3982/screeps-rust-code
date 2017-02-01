use super::Rc;
use super::Vec;

pub struct LightString {
	chars: Vec<u8>,
}

#[derive(Clone)]
pub enum MemoryKey {
	Integer (u32),
	String (Rc<LightString>),
}

#[derive(Clone)]
pub enum MemoryValue {
	Integer (u32),
	String (Rc<LightString>),
	Segment (Rc<FastMemorySegment>),
	None,
}

impl MemoryValue {
	pub fn unwrap_integer_or(&self, d: u32) -> u32 {
		match self {
			&MemoryValue::Integer (v) => v,
			_ => d,
		}
	}
}

pub struct FastMemorySegment {
	items: Vec<(MemoryKey, MemoryValue)>,
}

impl MemoryKey {
	fn type_as_integer(&self) -> i8 {
		match self {
			&MemoryKey::Integer (ref v) => 0i8,
			&MemoryKey::String (ref v) => 1i8,
		}
	}

	fn equal_to(&self, k: &MemoryKey) -> bool {
		match self {
			&MemoryKey::Integer (ref v0) => match k {
				&MemoryKey::Integer (ref v1) => *v0 == *v1,
				&MemoryKey::String (ref v1) => false,
			},
			default => false,
		}
	}
}

impl FastMemorySegment {
	pub fn set(&mut self, k: MemoryKey, v: MemoryValue) {
		let ndx = self.index_of_key(&k);

		if ndx < 0 {
			self.items.push((k, v));
		} else {
			self.items[ndx as usize] = (k, v);
		}
	}

	fn index_of_key(&self, k: &MemoryKey) -> isize {
		for q in 0..self.items.len() {
			if self.items[q].0.equal_to(k) {
				return q as isize;
			}
		}

		return -1;
	}

	pub fn get(&mut self, k: MemoryKey) -> Option<MemoryValue> {
		let ndx = self.index_of_key(&k);

		if ndx < 0 {
			Option::None
		} else {
			Option::Some(self.items[ndx as usize].1.clone())
		}
	}

	pub fn new() -> FastMemorySegment {
		FastMemorySegment {
			items: Vec::new(),
		}
	}

	/// Dump the entire structure into a buffer.
	pub fn dump(&self) -> Vec<u8> {
		Vec::new()
	}

	/// Load the entire structure from a buffer.
	pub fn load(&self, data: Vec<u8>) {

	}
}