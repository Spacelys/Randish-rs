pub mod lcg;
pub mod seed;

#[cfg(test)]
mod tests {

	#[test]
	fn create_predictable_LCGParams() {
		let lcg = crate::lcg::number_to_params(32);
		assert_eq!(lcg, crate::lcg::LCGParams {
			c: 3,
			a: 12289,
			m: 2147483648,
			x0: 1347763329
		});

		let lcg2 = crate::lcg::number_to_params(40296);
		assert_eq!(lcg2, crate::lcg::LCGParams {
			c: 145,
			a: 14609,
			m: 2147483648,
			x0: 1932850945
		});
	}

	#[test]
	fn generates_predictable_floats() {
		let lcg = crate::lcg::number_to_params(32);
		let mut rng = crate::seed::Seed::new(lcg, 101);

		assert_eq!(rng.rand_float(0_f32, 1_f32), 0.00057797506);
		assert_eq!(rng.rand_float(0_f32, 1_f32), 0.10273557);
		assert_eq!(rng.rand_float(0_f32, 1_f32), 0.51741034);
	}

	#[test]
	fn generates_predictable_uints() {
		let lcg = crate::lcg::number_to_params(4122);
		let mut rng = crate::seed::Seed::new(lcg, 333);

		assert_eq!(rng.rand_uint(10, 1000), 12);
		assert_eq!(rng.rand_uint(10, 1000), 586);
		assert_eq!(rng.rand_uint(10, 1000), 534);
	}

	#[test]
	fn generates_predictable_ints() {
		let lcg = crate::lcg::number_to_params(4122);
		let mut rng = crate::seed::Seed::new(lcg, 333);

		assert_eq!(rng.rand_int(-1000, 1000), -996);
		assert_eq!(rng.rand_int(-1000, 1000), 164);
		assert_eq!(rng.rand_int(-1000, 1000), 60);
	}


	#[test]
	fn generates_predictable_booleans() {
		let lcg = crate::lcg::number_to_params(12762);
		let mut rng = crate::seed::Seed::new(lcg, 8200);

		assert_eq!(rng.rand_bool(0.3), true);
		assert_eq!(rng.rand_bool(0.3), false);
		assert_eq!(rng.rand_bool(0.3), true);
	}

	#[test]
	fn generates_predictable_bytes() {
		let lcg = crate::lcg::number_to_params(54321);
		let mut rng = crate::seed::Seed::new(lcg, 12345);

		assert_eq!(rng.rand_bytes(), [15, 80, 138, 232]);
		assert_eq!(rng.rand_bytes(), [23, 45, 48, 139]);
	}

}
