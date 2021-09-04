use clap::{App, Arg};

pub fn build_argparse() -> App<'static, 'static> {
	App::new("QR cli")
		.about("Create QR code from command line")
		.version(crate_version!())
		.args(&[
			Arg::with_name("INPUT")
				.required(true)
				.index(1)
				.help("Sets the text to encode"),
			Arg::with_name("enc")
				.long("enc")
				.required(true)
				.value_name("ENC")
				.takes_value(true)
				.case_insensitive(true)
				.possible_values(&["auto", "numeric", "alphanumeric", "byte", "kanji"])
				.default_value("auto")
				.help("Sets the preferred text encode mode"),
			Arg::with_name("ec")
				.long("ec")
				.required(true)
				.value_name("LVL")
				.takes_value(true)
				.case_insensitive(true)
				.possible_values(&["L", "M", "Q", "H"])
				.default_value("M")
				.help("Sets the error correction level for encoding"),
			Arg::with_name("ver")
				.long("ver")
				.required(true)
				.value_name("VERSION")
				.takes_value(true)
				.default_value("auto")
				.help("Sets the preferred QR code version, should be 1~40"),
			Arg::with_name("imsize")
				.long("imsize")
				.value_name("SIZE")
				.takes_value(true)
				.required(false)
				.help("Sets the total size (in pixels) in the output image"),
			Arg::with_name("bitsize")
				.long("bitsize")
				.required(false)
				.value_name("SIZE")
				.takes_value(true)
				.default_value("10")
				.overrides_with("imsize")
				.help("Sets the size (in pixels) of individual bit in the QR code"),
			Arg::with_name("silent")
				.long("silent")
				.required(true)
				.value_name("SIZE")
				.takes_value(true)
				.default_value("4")
				.help("Sets the size (in bits) of the silent zone around the QR code matrix"),
			Arg::with_name("output")
				.long("output")
				.short("o")
				.required(true)
				.value_name("FILE")
				.takes_value(true)
				.default_value("output.png")
				.help("Sets the output file path and name"),
		])
}