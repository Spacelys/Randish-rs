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

// 	const seed: Seed = new Seed({
// 		...defaultParams,
// 		x0: n,
// 	});
// 	const c = 2 * seed.randInt(1, 3000) + 1;
// 	const a = 4 * seed.randInt(3000, 6000) + 1;
// 	const m = Math.pow(2, 32);
// 	const x0 = seed.randInt(1, m);
// 	return {
// 		c, a, m, x0,
// 	};
pub fn number_to_params(n: u32) -> LCGParams {
	// let seed: Seed = crate::seed::Seed::new(DEFAULT_LCG_PARAMS);
	let mut s = Seed::new(DEFAULT_LCG_PARAMS, n);
	let m = u32::pow(2, 31);
	LCGParams {
		c: 2 * s.rand_uint(1, 3000) + 1,
		a: 4 * s.rand_uint(3000, 6000) + 1,
		m: m,
		x0: s.rand_uint(1, m)
	}
}
// export interface LCGParams {
// 	c: number;
// 	a: number;
// 	m: number;
// 	x0: number;
// }

// const defaultParams: LCGParams = {
// 	c: 2 * 117 + 1,
// 	a: 4 * 317 + 1,
// 	m: Math.pow(2, 16),
// 	x0: 0,
// };

// export const numberToParams = (n: number): LCGParams => {
// 	const seed: Seed = new Seed({
// 		...defaultParams,
// 		x0: n,
// 	});
// 	const c = 2 * seed.randInt(1, 3000) + 1;
// 	const a = 4 * seed.randInt(3000, 6000) + 1;
// 	const m = Math.pow(2, 32);
// 	const x0 = seed.randInt(1, m);
// 	return {
// 		c, a, m, x0,
// 	};
// };