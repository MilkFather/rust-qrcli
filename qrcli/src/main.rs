use qr::prelude::*;
use qr::writer::prelude::*;

#[macro_use]
extern crate clap;

mod args;
use args::build_argparse;

fn main() {
	let arg = build_argparse().get_matches();
	/* Validate args */
	let input = arg.value_of("INPUT").unwrap(); // This argument is required so we always have a value
	let pre_enc_mode = match arg.value_of("enc").unwrap().to_lowercase().as_str() {
		"numeric" => Some(EncodeMode::Numeric),
		"alphanumeric" => Some(EncodeMode::Alphanumeric),
		"byte" => Some(EncodeMode::Byte),
		"kanji" => Some(EncodeMode::Kanji),
		"auto" | _ => None,
	};
	let ec_level = match arg.value_of("ec").unwrap().to_uppercase().as_str() {
		"L" => ErrorCorrectionLevel::L,
		"M" => ErrorCorrectionLevel::M,
		"Q" => ErrorCorrectionLevel::Q,
		"H" => ErrorCorrectionLevel::H,
		_ => { panic!("Invalid error correction level bypassed argument check"); }
	};
	let perf_ver = if arg.value_of("ver").unwrap().to_lowercase() == "auto" {
		None 
	} else {
		let raw = value_t!(arg, "ver", u8).unwrap_or_else(|e| {
			println!("Invalid version");
			e.exit();
		});
		Some(raw)
	};
	let qrsize = if arg.value_of("imsize").is_some() {
		let raw = value_t!(arg, "imsize", u32).unwrap_or_else(|e| {
			println!("Invalid imsize");
			e.exit();
		});
		QrSizeConfig::TotalSize(raw)
	} else {
		let raw = value_t!(arg, "bitsize", u32).unwrap_or_else(|e| {
			println!("Invalid bitsize");
			e.exit();
		});
		QrSizeConfig::BitSize(raw)
	};
	let silentzonesize = value_t!(arg, "silent", u32).unwrap_or_else(|e| {
		println!("Invalid silentzone");
		e.exit();
	});
	let output = arg.value_of("output").unwrap(); // This argument has a default so we always have a value

	/* Make QR code */
	let qr = make_qr(&String::from(input), pre_enc_mode, ec_level, perf_ver);
	match qr {
		Err(s) => { println!("Error, abort: {}", s) },
		Ok(q) => {
			q.to_image_luma(qrsize, &[0], &[255], silentzonesize).save(output).unwrap();
		}
	}
}
