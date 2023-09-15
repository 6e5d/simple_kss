use std::sync::mpsc::Receiver;
use std::sync::Arc;

use crate::pssyn::Pssyn;
use polysplit::{Synth, SynthGenerator};

pub struct Psgen {
	sr: usize,
	recv: Receiver<Arc<Vec<f32>>>,
	param: Arc<Vec<f32>>,
}

impl Psgen {
	pub fn new(recv: Receiver<Arc<Vec<f32>>>) -> Self {
		Self {
			sr: 0,
			recv,
			param: Arc::new(vec![0f32; 5]),
		}
	}
}

impl SynthGenerator for Psgen {
	fn set_sr(&mut self, sr: usize) {
		self.sr = sr;
	}
	fn generate(&mut self, note: u8, velocity: f32) -> Box<dyn Synth> {
		let freq = 440.0 * f32::powf(2.0, (note as f32 - 69.0) / 12.0);

		while let Ok(param) = self.recv.try_recv() {
			eprintln!("update param");
			self.param = param;
		}
		let ss = Pssyn::new(&self.param, self.sr, freq, velocity);
		Box::new(ss)
	}
}
