// "afi" - Aldaron's Format Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.  (See accompanying
// file LICENSE or copy at https://www.boost.org/LICENSE_1_0.txt)

/// A linear HSVA value, can be created from sRGB value.
struct LHsva(pub f32, pub f32, pub f32, pub f32);

fn f32_to_u8(v: f32) -> u8 {
	// (range 0 to 255) then round.
	(v * 255.0).round() as u8
}

/// Linear space to S space
fn linear_to_s(linear: f32) -> f32 {
	if linear <= 0.0031308 {
		linear * 12.92
	} else {
		1.055 * linear.powf(1.0/2.4) - 0.055
	}
}

impl LHsva {
	/// Create new Linear HSV from sRGBA.
	fn new(rgb: &[u8]) -> LHsva {
		// u8 S to f32 Linear Look Up Table
		const STOLINLUT: [f32; 256] = [
			0.0, 0.000303527, 0.000607054, 0.000910581,
			0.001214108, 0.001517635, 0.001821162, 0.0021246888,
			0.002428216, 0.0027317428, 0.00303527, 0.0033465358,
			0.0036765074, 0.004024717, 0.004391442, 0.0047769533,
			0.0051815165, 0.0056053917, 0.006048833, 0.0065120906,
			0.00699541, 0.007499032, 0.008023193, 0.008568126,
			0.009134059, 0.009721218, 0.010329823, 0.010960094,
			0.011612245, 0.012286488, 0.0129830325, 0.013702083,
			0.014443844, 0.015208514, 0.015996294, 0.016807375,
			0.017641954, 0.01850022, 0.019382361, 0.020288562,
			0.02121901, 0.022173885, 0.023153367, 0.024157632,
			0.02518686, 0.026241222, 0.027320892, 0.02842604,
			0.029556835, 0.030713445, 0.031896032, 0.033104766,
			0.034339808, 0.035601314, 0.03688945, 0.038204372,
			0.039546236, 0.0409152, 0.04231141, 0.04373503,
			0.045186203, 0.046665087, 0.048171826, 0.049706567,
			0.051269457, 0.052860647, 0.054480277, 0.05612849,
			0.05780543, 0.059511237, 0.061246052, 0.063010015,
			0.064803265, 0.06662594, 0.06847817, 0.070360094,
			0.07227185, 0.07421357, 0.07618538, 0.07818742,
			0.08021982, 0.08228271, 0.08437621, 0.08650046,
			0.08865558, 0.09084171, 0.093058966, 0.09530747,
			0.09758735, 0.099898726, 0.10224173, 0.104616486,
			0.107023105, 0.10946171, 0.11193243, 0.114435375,
			0.116970666, 0.11953843, 0.122138776, 0.12477182,
			0.12743768, 0.13013647, 0.13286832, 0.13563333,
			0.13843161, 0.14126329, 0.14412847, 0.14702727,
			0.14995979, 0.15292615, 0.15592647, 0.15896083,
			0.16202937, 0.1651322, 0.1682694, 0.17144111,
			0.1746474, 0.17788842, 0.18116425, 0.18447499,
			0.18782078, 0.19120169, 0.19461784, 0.19806932,
			0.20155625, 0.20507874, 0.20863687, 0.21223076,
			0.2158605, 0.2195262, 0.22322796, 0.22696587,
			0.23074006, 0.23455058, 0.23839757, 0.24228112,
			0.24620132, 0.25015828, 0.2541521, 0.25818285,
			0.26225066, 0.2663556, 0.2704978, 0.2746773,
			0.27889428, 0.28314874, 0.28744084, 0.29177064,
			0.29613826, 0.30054379, 0.3049873, 0.30946892,
			0.31398872, 0.31854677, 0.3231432, 0.3277781,
			0.33245152, 0.33716363, 0.34191442, 0.34670407,
			0.3515326, 0.35640013, 0.3613068, 0.3662526,
			0.3712377, 0.37626213, 0.38132602, 0.38642943,
			0.39157248, 0.39675522, 0.40197778, 0.4072402,
			0.4125426, 0.41788507, 0.42326766, 0.4286905,
			0.43415365, 0.43965718, 0.4452012, 0.4507858,
			0.45641103, 0.462077, 0.4677838, 0.47353148,
			0.47932017, 0.48514995, 0.49102086, 0.49693298,
			0.5028865, 0.50888133, 0.5149177, 0.52099556,
			0.5271151, 0.5332764, 0.5394795, 0.54572445,
			0.55201143, 0.5583404, 0.5647115, 0.57112485,
			0.57758045, 0.58407843, 0.59061885, 0.59720176,
			0.60382736, 0.61049557, 0.6172066, 0.6239604,
			0.63075715, 0.63759685, 0.6444797, 0.65140563,
			0.65837485, 0.6653873, 0.67244315, 0.6795425,
			0.6866853, 0.69387174, 0.7011019, 0.70837575,
			0.7156935, 0.7230551, 0.73046076, 0.7379104,
			0.7454042, 0.7529422, 0.7605245, 0.76815116,
			0.7758222, 0.7835378, 0.7912979, 0.7991027,
			0.80695224, 0.8148466, 0.82278574, 0.8307699,
			0.838799, 0.8468732, 0.8549926, 0.8631572,
			0.8713671, 0.8796224, 0.8879231, 0.8962694,
			0.9046612, 0.91309863, 0.92158186, 0.9301109,
			0.9386857, 0.9473065, 0.9559733, 0.9646863,
			0.9734453, 0.9822506, 0.9911021, 1.0,
		];

		// Linear RGBA
		let (r, g, b) = (
			STOLINLUT[rgb[0] as usize],
			STOLINLUT[rgb[1] as usize],
			STOLINLUT[rgb[2] as usize],
		);

		let a = rgb[3] as f32 * (1.0 / 255.0); // Alpha is always linear

		// Calculate hue.
		if r == g && g == b {
			LHsva(0.0, 0.0, r, a)
		} else if r > g {
			if r > b { // R is Max
				if b < g { // R is Max, B is Min
					let delta = r - b;
					let sat = if rgb[0] == 0 { 0.0 }
						else { delta / r };
					let hue = (g - b) / delta;
					LHsva(hue, sat, r, a)
				} else { // R is Max, G is Min
					let delta = r - g;
					let sat = if rgb[0] == 0 { 0.0 }
						else { delta / r };
					let hue = (g - b) / delta;
					LHsva(hue, sat, r, a)
				}
			} else { // B is Max, G is Min
				let delta = b - g;
				let sat = if rgb[1] == 0 { 0.0 }
					else { delta / b };
				let hue = 4.0 + (r - g) / delta;
				LHsva(hue, sat, b, a)
			}
		} else if g > b { // G is Max
			if r < b { // G is Max, R is Min
				let delta = g - r;
				let sat = if rgb[2] == 0 { 0.0 }
					else { delta / g };
				let hue = 2.0 + (b - r) / delta;
				LHsva(hue, sat, g, a)
			} else { // G is Max, B is Min
				let delta = g - b;
				let sat = if rgb[2] == 0 { 0.0 }
					else { delta / g };
				let hue = 2.0 + (b - r) / delta;
				LHsva(hue, sat, g, a)
			}
		} else { // B is Max, R is Min
			let delta = b - r;
			let sat = if rgb[1] == 0 { 0.0 }
				else { delta / b };
			let hue = 4.0 + (r - g) / delta;
			LHsva(hue, sat, b, a)
		}
	}

	/// Convert back into sRGBA.
	fn to_srgba(self) -> [u8; 4] {
		let a = f32_to_u8(self.3);
		let fh = self.0;
		let h = fh as i8; // int 0-6
		let s = self.1;
		let v = self.2;

		if !self.1.is_normal() { // if saturation is 0, then it's gray
			let e = f32_to_u8(linear_to_s(self.2));
			return [e, e, e, a];
		}

		let f = fh - (h as f32); // difference from rounding to 0-6
		let p = v * (1.0 - s);
		let q = v * (1.0 - s * f);
		let t = v * (1.0 - s * (1.0 - f));

		let (r, g, b) = match h {
			1 => (q, v, p),
			2 => (p, v, t),
			3 => (p, q, v),
			4 => (t, p, v),
			5 => (v, p, q),
			_ => (v, t, p), // 0 or 6
		};

		[
			f32_to_u8(linear_to_s(r)),
			f32_to_u8(linear_to_s(g)),
			f32_to_u8(linear_to_s(b)),
			a
		]
	}
}

fn over_blend(src: [u8; 4], dst: &mut [u8]) {
	let mut src = LHsva::new(&src);
	let mut dst2 = LHsva::new(dst);

	let src_hue_angle = src.0 * (::std::f32::consts::PI / 3.0);
	let (mut src_x, mut src_y) = src_hue_angle.sin_cos();

	let dst_hue_angle = dst2.0 * (::std::f32::consts::PI / 3.0);
	let (mut dst_x, mut dst_y) = dst_hue_angle.sin_cos();

	// Premultiply alpha
	src_x *= src.3;
	src_y *= src.3;
	src.1 *= src.3;
	src.2 *= src.3;
	dst_x *= dst2.3;
	dst_y *= dst2.3;
	dst2.1 *= dst2.3;
	dst2.2 *= dst2.3;
	// Calculate the 3 color channels (sat, val, alpha), and HUE XY.
	dst_x = src_x + dst_x * (1.0 - src.3);
	dst_y = src_y + dst_y * (1.0 - src.3);
	dst2.1 = src.1 + dst2.1 * (1.0 - src.3);
	dst2.2 = src.2 + dst2.2 * (1.0 - src.3);
	dst2.3 = src.3 + dst2.3 * (1.0 - src.3);
	// Bring the color value & saturation up (Postdivide alpha)
	dst2.1 /= dst2.3;
	dst2.2 /= dst2.3;
	// Turn XY back to HUE
	dst2.0 = dst_y.atan2(dst_x) * (3.0 / ::std::f32::consts::PI);

	dst.copy_from_slice(&dst2.to_srgba());
}

/// Put sRGBA src color over sRGBA dst color in the linear HSVA colorspace.
#[inline(always)]
pub fn over(src: [u8; 4], dst: &mut [u8]) {
	// OPTIMIZATIONS

	// If dst alpha is zero, then src.
	if dst[3] == 0 {
		unsafe { ::std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), 4) }
	} else if src[3] != 0 { // If src alpha isn't zero, then change dst.
		over_blend(src, dst);
	}
}

/// Blend multiple sRGBA colors in the linear HSVA colorspace.
pub fn blend(colors: &[([u8; 4], f32)]) -> [u8; 4] {
	let mut out = LHsva::new(&colors[0].0);
	let hue_angle = out.0 * (::std::f32::consts::PI / 3.0);
	let (mut x, mut y) = hue_angle.sin_cos();
	let mut div = colors[0].1;
	let alpha = out.3;

	// Alpha Blend * Weighted Blend
	x *= alpha * colors[0].1;
	y *= alpha * colors[0].1;
	out.1 *= alpha * colors[0].1;
	out.2 *= alpha * colors[0].1;
	out.3 *= alpha * colors[0].1;

	for i in colors.iter() {
		let mut new = LHsva::new(&i.0);
		let hue_angle = new.0 * (::std::f32::consts::PI / 3.0);
		let (mut new_x, mut new_y) = hue_angle.sin_cos();
		div += i.1;
		let alpha = new.3;

		// Alpha Blend * Weighted Blend
		new_x *= alpha * i.1;
		new_y *= alpha * i.1;
		new.1 *= alpha * i.1;
		new.2 *= alpha * i.1;
		new.3 *= alpha * i.1;

		x += new_x;
		y += new_y;
		out.1 += new.1;
		out.2 += new.2;
		out.3 += new.3;
	}

	div = 1.0 / div;
	out.0 = y.atan2(x) * (3.0 / ::std::f32::consts::PI);
	out.1 *= div;
	out.2 *= div;
	out.3 *= div;

	out.to_srgba()
}
