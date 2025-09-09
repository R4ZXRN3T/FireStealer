use std::env;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use jsonformat::format;
use jsonformat::Indentation::Tab;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// get path of logins.json (Firefox password file)
	let final_file_path = get_file_path(cfg!(windows))?;

	// print contents of logins.json
	let file_contents = format(&read_to_string(final_file_path)?, Tab);
	println!("{}", file_contents);
	Ok(())
}

fn get_file_path(is_windows: bool) -> Result<String, &'static str> {
	// get path of the user's home directory
	let home_dir = env::var("HOME")
		.or_else(|_| env::var("USERPROFILE"))
		.map_err(|_| "Could not find home directory")?;

	// set path of the Firefox folder
	let base_path = if is_windows {
		home_dir.clone() + "\\AppData\\Roaming\\Mozilla\\Firefox"
	} else {
		home_dir.clone() + "/.mozilla/firefox"
	};
	let profile_sub_path =
		get_profile_sub_path(&get_all_lines(&(base_path.clone() + "/profiles.ini")).unwrap())
			.unwrap();
	let final_file_name = base_path + "/" + &profile_sub_path + "/logins.json";

	Ok(final_file_name)
}

// This function returns all lines of a file as a Vec<String>, wrapped in a Result
fn get_all_lines(path: &str) -> Result<Vec<String>, Error> {
	Ok(read_to_string(path)?
		.lines()
		.map(|line| line.to_string())
		.collect())
}

// Reads and returns the subpath of the first profile found in the provided profiles.ini
fn get_profile_sub_path(profile_file: &Vec<String>) -> Result<String, Error> {
	// get location of the first mention of path
	let string_location = get_string_location("Path=", profile_file);

	// error handling
	if string_location.is_err() {
		return Err(Error::new(
			ErrorKind::NotFound,
			"No profile path could be found",
		));
	}

	// Remove the "Path=" from the string and return the final profile sub path
	let profile_sub_path = profile_file[string_location?][5..].to_string();
	Ok(profile_sub_path)
}

// This function returns the first array index that contains the provided string
fn get_string_location(string_to_find: &str, array: &Vec<String>) -> Result<usize, Error> {
	for i in 0..array.len() {
		if array[i].contains(&string_to_find.to_string()) {
			return Ok(i);
		}
	}
	Err(Error::new(ErrorKind::NotFound, "String could not be found"))
}
