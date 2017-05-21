use std::cell::RefCell;

pub trait Callback<T>: Send {
    fn with_ref(&self, function: &mut FnMut(&T));
    fn with_mut(&self, function: &mut FnMut(&mut T));
}

impl<T: Send> Callback<T> for RefCell<T> {
    fn with_ref(&self, function: &mut FnMut(&T)) {
        function(&*self.borrow())
    }

    fn with_mut(&self, function: &mut FnMut(&mut T)) {
        function(&mut *self.borrow_mut())
    }
}
