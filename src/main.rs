use home::home_dir;
use mpd::{Client, Idle, State::Play, Subsystem::Player};
use notify_rust::{Notification, Urgency::Normal};
use std::{collections::BTreeMap, thread::sleep, time::Duration};

fn main() {
	let mut conn = {
		loop {
			let conn = Client::connect("127.0.0.1:6600");
			if let Ok(value) = conn {
				break value;
			}
			sleep(Duration::from_secs(1));
		}
	};
	loop {
		conn.wait(&[Player]).unwrap();
		let status = conn.status().unwrap();
		if status.state == Play {
			let song = conn.currentsong().unwrap().unwrap();
			let file = get_file(song.file);
			let cover_path = format!(
				"{}/Music/{}/cover.jpg",
				home_dir().unwrap().display().to_string(),
				file
			);

			let parsed_tags = parse_tags(song.tags, song.title);

			Notification::new()
				.summary("MPD")
				.body(&parsed_tags)
				.icon(&cover_path)
				.urgency(Normal)
				//arbitrary id 
				.id(3094822)
				.show()
				.unwrap();
		}
	}
}

fn parse_tags(tags: BTreeMap<String, String>, title: Option<String>) -> String {
	let mut parsed_tags = String::new();

	if let Some(value) = title {
		parsed_tags.push_str(&*format!("\n<b>Title:</b>\t<span>{}</span>", &value));
	}

	//tags will only contain duration if song lacks tags
	if tags.len() == 1 {
		return parsed_tags;
	}

	for (key, value) in tags.into_iter() {
		match &*key {
			"Artist" => parsed_tags.push_str(&*format!("\n<b>Artist:</b>\t<span>{}</span>", value)),
			"Album" => parsed_tags.push_str(&*format!("\n<b>Album:</b>\t<span>{}</span>", value)),
			_ => (),
		}
	}
	parsed_tags
}

fn get_file(path: String) -> String {
	let path_parsed = path.rsplit_once('/');

	if let Some(value) = path_parsed {
		value.0.to_string()
	} else {
		path.split_once('.').unwrap().0.to_string()
	}
}
