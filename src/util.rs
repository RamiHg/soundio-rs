use std::ffi::CStr;
use std::ptr;
use std::os::raw::c_char;

/// Convert a Latin1 C String to a String. Specifically it assumes each byte
/// is a Unicode code point in the range 0-255. Values 0-127 are always ASCII,
/// but there are a few different uses of the values 128-255. Specifically:
///
/// * [ISO/IEC 8859-1](https://en.wikipedia.org/wiki/ISO/IEC_8859-1) AKA "Latin-1".
/// * [Unicode Latin-1](https://en.wikipedia.org/wiki/Latin-1_Supplement_(Unicode_block))
/// * [Windows 1252](https://en.wikipedia.org/wiki/Windows-1252)
///
/// Annoyingly, ISO/IEC 8859-1 leaves 32 code points undefined in the 128-255 range.
/// Unicode Latin-1 defines them to be control codes. Windows 1252 uses them for a few
/// rare characters, but most notably ™ and €.
///
/// This function assumes Unicode Latin-1.
/// 
/// If `s` is null, an empty string is returned.
///
/// # Examples
///
/// ```
/// use std::os::raw::c_char;
/// let latin1: Vec<c_char> = "£µ±«\0".chars().map(|c| c as c_char).collect();
/// assert_eq!(soundio::latin1_to_string(latin1.as_ptr()), "£µ±«")
/// ```
pub fn latin1_to_string(s: *const c_char) -> String {
	if s == ptr::null() {
		return String::new();
	}
	let c_str: &CStr = unsafe { CStr::from_ptr(s) };
	// This converts Latin1 to a String, instead of assuming UTF-8 (which I probably could to be fair).
	c_str.to_bytes().iter().map(|&c| c as char).collect()
}

/// Convert a UTF-8 C String to a String.
/// If `s` is null or contains invalid UTF-8, an empty string is returned.
pub fn utf8_to_string(s: *const c_char) -> String {
	if s == ptr::null() {
		return String::new();
	}
	let c_str: &CStr = unsafe { CStr::from_ptr(s) };

	c_str.to_str().unwrap_or("").to_string()
}