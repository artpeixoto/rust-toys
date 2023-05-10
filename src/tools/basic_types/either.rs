pub enum Either<T, U> {
	Left(T),
	Right(U),
}

impl<T,U> Either<T,U> {
	pub fn is_left(&self) -> bool{
		match self{
			Either::Left(_)  => true,
			Either::Right(_) => false
		}
	}

	pub fn l_val_ref<'a> (&'a self) -> Option<&'a T> {
		match self{
			Either::Left(x)  => Some(x),
			Either::Right(_) => None,
		}
	}

	pub fn r_val_ref<'a>(&'a self) -> Option<&'a U> {
		match self{
			Either::Right(x)  => Some(x),
			Either::Left(_)   => None,
		}
	}

	pub fn l_val(self) -> Option<T>{
		match self{
			Either::Left(x)  => Some(x),
			Either::Right(_)   => None,
		}
	}

	pub fn r_val(self) -> Option<U>{
		match self{
			Either::Right(x)  => Some(x),
			Either::Left(_)   => None,
		}
	}
	pub fn as_ref<'a>(&'a self) -> Either<&T, &U>{
		match self{
			Either::Left(x) => 	 Either::Left(x),
			Either::Right(x) =>  Either::Right(x),
		}
	}
}