use std::{ops::{Deref, DerefMut}, cell::{RefCell, Ref}, borrow::{Borrow, BorrowMut}};
use super::either::Either;

pub enum Thunk<'a, TVal>{
	Promise(Option<Box<dyn FnOnce() -> TVal + 'a>>),
	Value(TVal),
}

use Thunk::*;

impl<'a, TVal> Thunk<'a, TVal>{
	pub fn new_getter<TGetter: 'a + FnOnce() -> TVal>(getter: TGetter) -> Thunk<'a, TVal> {
		let boxed_getter: Box<dyn FnOnce() -> TVal + 'a> = Box::new(getter);
		Thunk::Promise(Some(boxed_getter))
	}

	pub fn load(&mut self) {
		if let Thunk::Promise(some_getter) = self{
			if let Some(getter) = some_getter.take(){
				let val = ( getter )();
				*self = Thunk::Value(val);
			}
			else {
				panic!("Erro!");
			}
		}
	}

	pub fn unwrap(mut self) -> TVal{
		self.load();
		if let Value(val) = self {
			val
		}
		else {
			panic!()
		}
	}

	pub fn val_ref(& mut self) -> & TVal { 
		self.load();
		if let Value(val) = self {
			val
		}
		else {
			panic!()
		}
	}
}

