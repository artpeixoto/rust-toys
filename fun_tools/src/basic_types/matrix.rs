use std::{sync::RwLock, fmt::Write, num::NonZeroU16};


type DimsT = [usize];



pub struct RGB{
	Red 	: u16,
	Green	: u16,
	Blue 	: u16,
}	

const FullHdDims: [usize;2] = [1920, 1080];

pub struct ScreenHandle();



static screen_handle: Option<ScreenHandle> = None;
