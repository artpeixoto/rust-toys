use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy, Debug)]
struct Complex<TNum>{
	pub re : TNum,
	pub im : TNum,
}


impl<TNum> Add for Complex<TNum> 
	where TNum: Add<Output=TNum> + Copy {
    type Output = Complex<TNum>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex{
			re: self.re + rhs.re,
			im: self.im + rhs.im,
		}
    }
}

impl<TNum> Sub for Complex<TNum> 
	where TNum: Sub<Output=TNum> + Copy {
    type Output = Complex<TNum>;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex{
			re: self.re - rhs.re,
			im: self.im - rhs.im,
		}
    }
}

impl<TNum> Mul for Complex<TNum> 
	where TNum: Add<Output=TNum> + Mul<Output=TNum> + Sub<Output=TNum> + Copy {
    type Output = Complex<TNum>;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex{
			re: (self.re * rhs.re) - (self.im * rhs.im),
			im: (self.re * rhs.im) + (self.im * rhs.re),
		}
    }
}
impl<TNum>  Complex<TNum> 
	where TNum: Add<Output=TNum> + Mul<Output=TNum> + Sub<Output=TNum> + Copy {
    

}
