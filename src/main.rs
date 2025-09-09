use jsonformat::format;
use jsonformat::Indentation::Tab;
use std::env;
use std::fs::read_to_string;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let final_file_path = get_file_path(cfg!(windows))?;
	let file_contents = format(&read_to_string(&final_file_path)?, Tab);
	println!("{}", file_contents);
	Ok(())
}

fn get_file_path(is_windows: bool) -> Result<String, Box<dyn std::error::Error>> {
	let home_dir = env::var("HOME")
		.or_else(|_| env::var("USERPROFILE"))
		.map_err(|_| io::Error::new(ErrorKind::NotFound, "Could not find home directory"))?;

	let mut base_path = PathBuf::from(home_dir);
	if is_windows {
		base_path.push("AppData\\Roaming\\Mozilla\\Firefox");
	} else {
		base_path.push(".mozilla/firefox");
	}

	let profile_file_path = base_path.join("profiles.ini");
	let profiles_ini_file = get_all_lines(&profile_file_path)?;
	let profile_sub_path = get_profile_sub_path(&profiles_ini_file)?;

	let final_file = base_path.join(profile_sub_path).join("logins.json");
	Ok(final_file.to_string_lossy().into_owned())
}

fn get_all_lines(path: &PathBuf) -> Result<Vec<String>, io::Error> {
	Ok(read_to_string(path)?
		.lines()
		.map(|line| line.to_string())
		.collect())
}

fn get_profile_sub_path(profile_file: &[String]) -> Result<String, io::Error> {
	profile_file
		.iter()
		.find(|line| line.starts_with("Path="))
		.map(|line| line[5..].to_string())
		.ok_or_else(|| io::Error::new(ErrorKind::NotFound, "No profile path could be found"))
}
