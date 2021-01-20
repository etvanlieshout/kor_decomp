// decompose a korean char into components

#![feature(assoc_char_funcs)]
use std::fmt::Display;
use std::char::from_u32;

fn decomp(c: &char) -> (char, char, char) {
	let num_val = *c as u32;
	let tail_num = 1+(num_val - 0xac00)%0x1c;
	let lead_num = 1+(num_val-0xac00)/0x24c;
	let vowel_num = 1+ ((num_val-0xac00-tail_num)%0x24c)/0x1c;

//	let lead_char = match char::from_u32(lead_num) {
//		Some(character) => character,
//		None => 'x',
//	};
//	let vowel_char = match char::from_u32(vowel_num) {
//		Some(character) => character,
//		None => 'x',
//	};
//	let tail_char = match char::from_u32(tail_num) {
//		Some(character) => character,
//		None => 'x',
//	};

//	let lead_char = '\u{lead_num}';
//	let vowel_char = '\u{vowel_num}';
//	let tail_char = '\u{tail_num}';

	(lead_char, vowel_char, tail_char)
}

#[derive(Debug)]
struct Jamo(char, char, char);

fn main() {

	// Start with a static char; later use user input and perhaps multiple char
	// strings

	let c = '설';
	let (a, b, c) = decomp(&c);
	let jamo = Jamo(a, b, c);
	println!("{} decomposes into {:?}", c, jamo);

}
