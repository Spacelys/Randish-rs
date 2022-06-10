use crate::seed::Seed;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LCGParams {
	pub c: u32,
	pub a: u32,
	pub m: u32,
	pub x0: u32
}

const DEFAULT_LCG_PARAMS: LCGParams = LCGParams {
	c: 2 * 117 + 1,
	a: 4 * 317 + 1,
	m: u32::pow(2, 31),
	x0: 0
};

pub fn number_to_params(n: u32) -> LCGParams {
	let mut s = Seed::new(DEFAULT_LCG_PARAMS, n);
	let m = u32::pow(2, 31);
	LCGParams {
		c: 2 * s.rand_uint(1, 3000) + 1,
		a: 4 * s.rand_uint(3000, 6000) + 1,
		m: m,
		x0: s.rand_uint(1, m)
	}
}
