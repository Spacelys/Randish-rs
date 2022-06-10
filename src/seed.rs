use crate::lcg::LCGParams;
use core::ops::{Add, Sub};

pub struct Seed {
	params: LCGParams,
	x0: u32,
	xn: u32,
}

impl Seed {
	pub fn new(p: LCGParams, n: u32) -> Seed {
		Seed {
			params: p,
			x0: p.x0,
			xn: n
		}
	}

	fn upper_bound(&self) -> u32 {
		self.params.m
	}

	pub fn next_raw(&mut self) -> u32 {
		let next_value = (
			// self.params.a * self.xn + self.params.c
			(
				self.params.a.wrapping_mul(self.xn)
			).wrapping_add(self.params.c)
		).rem_euclid(self.params.m);
		self.xn = next_value;
		next_value
	}

	pub fn rand_bytes(&mut self) -> [u8; 4] {
		let val = self.next_raw();
		val.to_be_bytes()
	}

	pub fn rand_bool(&mut self, percent_true: f32) -> bool {
		self.rand_float(0.0, 1.0) < percent_true
	}

	pub fn rand_int(&mut self, min: i32, max: i32) -> i32 {
		let delta = (max - min) as f32;
		let p = self.rand_float(0.0, 1.0);
		min + ((delta) * p) as i32
	}

	pub fn rand_uint(&mut self, min: u32, max: u32) -> u32 {
		let delta = (max - min) as f32;
		let p = self.rand_float(0.0, 1.0);
		min + ((delta) * p) as u32
	}

	pub fn rand_float(&mut self, min: f32, max: f32) -> f32 {
		let n = self.next_raw();
		let p = n as f32 / self.upper_bound() as f32;
		min + (max - min) * p
	}
}
