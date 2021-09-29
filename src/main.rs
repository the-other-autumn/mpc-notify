use home::home_dir;
use mpd::{Client, Idle};
use mpd::{State::Play, Subsystem::Player};
use notify_rust::{Notification, Urgency::Normal};
use std::collections::BTreeMap;

fn main() {
	let mut conn = Client::connect("0.0.0.0:6600").unwrap();
	loop {
		conn.wait(&[Player]).unwrap();
		let status = conn.status().unwrap();
		if status.state == Play {
			let song = conn.currentsong().unwrap().unwrap();
			println!("{}", &song.file);
			let artpath = format!(
				"{}/Music/{}/cover.jpg",
				home_dir().unwrap().display().to_string(),
				song.file.rsplit_once('/').unwrap().0
			);

			let output = parse_tags(song.tags, song.title);

			Notification::new()
				.summary("MPD")
				.body(&output)
				.icon(&artpath)
				.urgency(Normal)
				.id(3094822)
				.show()
				.unwrap();
		}
	}
}

fn parse_tags(tags: BTreeMap<String, String>, title: Option<String>) -> String {
	let mut parsed_tags = String::new();

	if title.is_some() {
		parsed_tags.push_str(&*format!("\n<b>Title:</b>\t<span>{}</span>", title.unwrap()));
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
