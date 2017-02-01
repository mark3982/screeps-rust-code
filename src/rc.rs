use core;

pub struct Rc<T> {
	phantom: core::marker::PhantomData<T>,
}

impl<T> Clone for Rc<T> {
	fn clone(&self) -> Self {
		Rc {
			phantom: core::marker::PhantomData,
		}
	}
}

impl<T> Rc<T> {
}