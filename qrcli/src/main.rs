use image::{GrayImage, Luma};
use qr::make_qr;
use qr::ErrorCorrectionLevel;

fn main() {
	let qr = make_qr(&String::from("hello, world"), None, ErrorCorrectionLevel::M, None);
	match qr {
		Err(s) => { println!("{}", s) },
		Ok(q) => {
			let pixel_per_bit: u32 = 16;
			let slient_zone_size: u32 = 4;
			let foreground_color = Luma([0]);
			let background_color = Luma([255]);

			let mat = q.matrix;
			let mut img = GrayImage::from_fn(((mat.shape()[1] as u32 + 2 * slient_zone_size) * pixel_per_bit) as u32, ((mat.shape()[0] as u32 + 2 * slient_zone_size) * pixel_per_bit) as u32, |_, _| background_color);

			for ((y, x), q) in mat.indexed_iter() {
				for i in 0..pixel_per_bit {
					for j in 0..pixel_per_bit {
						img.put_pixel((x as u32 + slient_zone_size) * pixel_per_bit + i, (y as u32 + slient_zone_size) * pixel_per_bit + j, if *q { foreground_color } else { background_color });
					}
				}
			}

			img.save("test.png").unwrap();

		}
	}
}
