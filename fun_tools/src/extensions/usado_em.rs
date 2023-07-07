
pub trait UsadoEm: Sized {	
	fn used_in<TRes: Sized, TFunc:      Fn(Self) 		-> TRes>(self, func: TFunc) 			-> TRes 	{func(self)} 
	fn used_in_once<TRes: Sized, TFunc: FnOnce(Self) -> TRes>	(self, func: TFunc) 	 			-> TRes 	{func(self)} 
	fn used_in_mut<TRes: Sized, TFunc:  FnMut(Self) -> TRes>(self,  func: &mut TFunc) 	-> TRes 	{func(self)} 
}

impl<T:Sized> UsadoEm for T{}