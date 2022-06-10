/*
	protected params: LCGParams;
	protected xn: number;
	protected x0: number; // keeps track of its initial starting value
	public constructor(params: LCGParams | number) {
		if (typeof params === 'number') {
			const lcgParams = numberToParams(params);
			this.params = lcgParams;
			this.x0 = lcgParams.x0;
			this.xn = lcgParams.x0;
		} else {
			this.params = params;
			this.xn = params.x0;
			this.x0 = params.x0;
		}
	}

	/**
	 * Resets seed to its starting state
	 *
	 * @memberof Seed
	 */
	public reset(): void {
		this.xn = this.x0;
	}
	/**
	 * Returns a fork of your current seed, guaranteed to give you the same
	 * subsequent values of its parent
	 *
	 * @returns {Seed}
	 * @memberof Seed
	 */
	public fork(): Seed {
		const myParams = this.params;
		return new Seed({
			...myParams,
			x0: this.xn,
		});
	}
	/**
	 * Returns an integer within the range provided
	 * up to (but not including) the max value
	 *
	 * @param {number} min
	 * @param {number} max
	 * @returns
	 * @memberof Seed
	 */
	public randInt(min: number, max: number): number {
		const delta = max - min;
		const p = this.randFloat(0, 1);
		return min + Math.floor(p * delta);
	}

	/**
	 * Returns a float | decimal number within the range provided
	 * up to (but not including) the max value
	 *
	 * @param {number} min
	 * @param {number} max
	 * @returns {number}
	 * @memberof Seed
	 */
	public randFloat(min: number, max: number): number {
		const n = this.nextRaw();
		const p = n / this.getUpperBound();
		return min + (max - min) * p;
	}

	/**
	 * Returns a random boolean with equal chances of being true or false no parameter passed in
	 *
	 * @param {*} [percentTrue=.5] changes the percentage of times true values get returned
	 * @returns {boolean}
	 * @memberof Seed
	 */
	public randBool(percentTrue = .5): boolean {
		const f = this.randFloat(0, 1);
		return f < percentTrue;
	}

	/**
	 * Create a random array of size N, and use the creator methods to fill it with values
	 *
	 * @template T
	 * @param {number} size
	 * @returns {IArrayCreator}
	 * @memberof Seed
	 */
	public randArray(size: number): IArrayCreator {
		const ary = new Array(size).fill(0);
		return {
			randInt: (min: number, max: number) =>
				ary.map(() => this.randInt(min, max)),
			randFloat: (min: number, max: number) =>
				ary.map(() => this.randFloat(min, max)),
			randBool: (percentTrue?: number) =>
				ary.map(() => this.randBool(percentTrue)),
		};
	}

	public randGrid(w: number, h: number): IGridCreator {
		const grid: Array<Array<number>> = new Array(h)
			.fill(0)
			.map((): Array<number> => new Array<number>(w).fill(0));

		return {
			randInt: (min: number, max: number) =>
				grid.map((line) =>
					line.map(() => this.randInt(min, max))
				),
			randFloat: (min: number, max: number) =>
				grid.map((line) =>
					line.map(() => this.randFloat(min, max))
				),
			randBool: (percentTrue?: number) =>
				grid.map((line) =>
					line.map(() => this.randBool(percentTrue))
				),
		};
	}

	public randObject(recipe: Record<string, unknown>): Record<string, unknown> {
		const obj: Record<string, unknown> = {};
		Object.keys(recipe).forEach(key => {
			const v = recipe[key];
			if (typeof v === 'function') {
				obj[key] = v(this);
			} else if (typeof v === 'object') {
				obj[key] = this.randObject(v as Record<string, unknown>);
			} else {
				obj[key] = v;
			}
		});
		return obj;
	}
	/**
	 * Gets the maximum possible value nextRaw() can return
	 *
	 * @protected
	 * @returns {number}
	 * @memberof Seed
	 */
	protected maxRaw(): number {
		return this.params.m - 1;
	}

	/**
	 * Gets the upper bound set by our LCG params
	 *
	 * @protected
	 * @returns {number}
	 * @memberof Seed
	 */
	 protected getUpperBound(): number {
		return this.params.m;
	 }

	/**
	 * Get the next raw value from the LCG sequence defined by our params
	 *
	 * @private
	 * @returns {number}
	 * @memberof Seed
	 */
	private nextRaw(): number {
		const {a, c, m} = this.params;
		const nextValue = ( a * this.xn + c ) % m;
		this.xn = nextValue;
		return nextValue;
	}
*/
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
