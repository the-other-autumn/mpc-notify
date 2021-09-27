use home::home_dir;
use notify_rust::Notification;
use std::process::Command;

const FORM: &str="\n<b>Artist:</b>\t<span>%artist%</span>\n<b>Album:</b>\t<span>%album%</span>\n<b>Title:</b>\t<span>%title%</span>";

fn main() {
	loop {
		let _mpc = Command::new("mpc").args(["idle", "player"]).output().unwrap();
		let output = Command::new("mpc").args(["current", "-f", FORM]).output().unwrap();
		let parsed_output = String::from_utf8_lossy(&output.stdout).to_string();
		if !parsed_output.is_empty() {
			let path = Command::new("mpc").args(["status", "-f", "%file%"]).output().unwrap();
			let parsed_path = String::from_utf8_lossy(&path.stdout).to_string();
			let artpath = format!(
				"{}/Music/{}/cover.jpg",
				home_dir().unwrap().display().to_string(),
				parsed_path.lines().next().unwrap().rsplit_once('/').unwrap().0
			);
			Notification::new()
				.summary("MPD")
				.body(&parsed_output)
				.icon(&artpath)
				.id(3094822)
				.show()
				.unwrap();
		}
	}
}
