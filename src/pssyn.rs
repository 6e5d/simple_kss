use std::collections::VecDeque;

use polysplit::polyrel::{PolyrelWrapper, Polyrel};

pub struct Pssyn {
	pub buffer: VecDeque<f32>,
	f: f32,
	point: f32,
	v: f32,
	old: f32,
	decay: f32,
}

impl Pssyn {
	pub fn new(param: &[f32], sr: usize, freq: f32, velocity: f32) -> PolyrelWrapper {
		let mut ps = Self {
			buffer: Default::default(),
			point: param[0],
			f: freq,
			v: velocity,
			decay: 1.0 - param[3],
			old: param[2],
		};
		ps.initialize_buffer(param[4] as u32, sr as f32);
		PolyrelWrapper::new(Box::new(ps), 2.0, param[1] * sr as f32)
	}

	pub fn initialize_buffer(&mut self, ty: u32, sr: f32) {
		let size = (1.0 / self.f * sr) as usize;
		if self.point > 1.0 { self.point = 1.0; }
		if self.point < 0.0 { self.point = 0.0; }
		let point = (self.point * size as f32) as usize;
		eprintln!("{} {} {}", self.f, size, point);
		let mut v = vec![0f32; size];
		if ty == 0 {
			for i in 0..point {
				v[i] = i as f32 / point as f32;
			}
			for i in point..size {
				v[i] = (size - i) as f32 / (size - point) as f32;
			}
		} else if ty == 1 {
			for i in 0..point as usize  {
				v[i] = 1f32;
			}
		} else {
			use rand::{Rng, SeedableRng};
			let mut rng = rand::rngs::SmallRng::seed_from_u64(0);
			for i in 0..point as usize  {
				v[i] = rng.gen::<f32>() * 2.0 - 1.0;
			}
		}
		self.buffer = VecDeque::from(v);
	}
}

impl Polyrel for Pssyn {
	fn go(&mut self) -> Option<[f32; 2]> {
		let x = self.buffer.pop_back().unwrap();
		let mut o = (x + self.buffer[0] * self.old) / (1.0 + self.old)
			* self.decay;
		self.buffer.push_front(o);
		o *= 0.3 * self.v;
		Some([o, o])
	}
}
