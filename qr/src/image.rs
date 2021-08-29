#![cfg(feature = "image")]

use image_::{
	RgbImage,
	Rgb,
	imageops::{
		FilterType,
		resize,
	},
};

use crate::qr::QrMatrix;

#[derive(Clone, Copy, Debug)]
pub enum QrSizeConfig {
	TotalSize(u32),
	BitSize(u32),
}

impl QrMatrix {

	pub fn to_image(&self, size: QrSizeConfig, foreground: Rgb<u8>, background: Rgb<u8>, silent_zone_size: u32) -> RgbImage {
		let bitsize = match size {
			QrSizeConfig::TotalSize(t) => f64::ceil(t as f64 / (self.matrix.shape()[1] as f64 + 2.0 * silent_zone_size as f64)) as u32,
			QrSizeConfig::BitSize(b) => b,
		};
		let mut image = RgbImage::from_fn((self.matrix.shape()[1] as u32 + 2 * silent_zone_size) * bitsize, (self.matrix.shape()[0] as u32 + 2 * silent_zone_size) * bitsize, |_, _| background);
		for ((y, x), q) in self.matrix.indexed_iter() {
			for i in 0..bitsize {
				for j in 0..bitsize {
					image.put_pixel((x as u32 + silent_zone_size) * bitsize + i, (y as u32 + silent_zone_size) * bitsize + j, if *q { foreground } else { background });
				}
			}
		}

		if let QrSizeConfig::TotalSize(total) = size {
			resize(&image, total, total, FilterType::Triangle)
		} else {
			image
		}
	}

}