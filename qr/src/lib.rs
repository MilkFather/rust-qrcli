mod ec;
mod encode;
mod matrix;
mod version;

pub use ec::ErrorCorrectionLevel;
pub use encode::{EncodeMode, test_encode_mode};
pub use version::smallest_version_by_encoding_and_eclevel;

use ec::error_correction;
use encode::{compile_pool, encode_text};
use matrix::make_matrix;

pub struct QrMatrix {
	pub version: u8,
	pub encode_mode: EncodeMode,
	pub error_correction_level: ErrorCorrectionLevel,
	pub matrix: ndarray::Array2<bool>
}

pub fn make_qr(text: &String, preferred_encode_mode: Option<EncodeMode>, error_correction_level: ErrorCorrectionLevel, preferred_version: Option<u8>) -> Result<QrMatrix, String> {
	let encode_mode = if preferred_encode_mode.is_none() { test_encode_mode(text) } else { Some(preferred_encode_mode.unwrap()) };
	if encode_mode.is_none() { return Err(String::from("Cannot encode text")) }
	let encode_mode = encode_mode.unwrap();

	let version = if preferred_version.is_none() {
		smallest_version_by_encoding_and_eclevel(text.len(), encode_mode, error_correction_level) 
	} else { Some(preferred_version.unwrap()) };
	if version.is_none() { return Err(String::from("Cannot find suitable version")) }
	let version = version.unwrap();
	if version > 40 || version < 1 { return Err(String::from("Impossible version")) }

	let codepool = encode_text(text, encode_mode, error_correction_level, version);
	if codepool.is_err() { return Err(codepool.unwrap_err()) }
	let codepool = codepool.unwrap();
	let ecpool = error_correction(&codepool, error_correction_level, version);
	
	/* Interleaving */
	let data_box = compile_pool(&codepool, &ecpool, error_correction_level, version);
	let matrix: ndarray::Array2<bool> = make_matrix(&data_box, error_correction_level, version);

	Ok(QrMatrix{version, encode_mode, error_correction_level, matrix})
}