fn main() {
	/// S space to Linear space
	fn s_to_linear(s: f64) -> f64 {
		if s <= 0.04045 {
			s / 12.92
		} else {
			((s + 0.055) / 1.055).powf(2.4)
		}
	}

	for i in 0..256 {
		println!("{},", (s_to_linear((i as f64) / 255.0))
			as f32);
	}
}
