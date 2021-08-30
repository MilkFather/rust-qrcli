use qr::prelude::*;
use qr::writer::prelude::*;

fn main() {
	let qr = make_qr(&String::from("hello world"), None, ErrorCorrectionLevel::M, None);
	match qr {
		Err(s) => { println!("{}", s) },
		Ok(q) => {
			q.to_image_luma(QrSizeConfig::BitSize(16), &[0], &[255], 4).save("hello.png").unwrap();
		}
	}
}
