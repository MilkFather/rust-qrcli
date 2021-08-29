use qr::prelude::*;

use image::Rgb;
use serde_json;

fn main() {
	let qr = make_qr(&String::from("hello world"), None, ErrorCorrectionLevel::M, None);
	match qr {
		Err(s) => { println!("{}", s) },
		Ok(q) => {
			let jstr = serde_json::to_string(&q).unwrap();
			println!("{}", jstr);
			q.to_image(QrSizeConfig::BitSize(16), Rgb::<u8>([0, 0, 0]), Rgb::<u8>([255, 255, 255]), 4).save("hello.png").unwrap();
		}
	}
}
