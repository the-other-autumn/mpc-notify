use home::home_dir;
use mpd::{error::Error, Client, Idle, Song, State::Play, Subsystem::Player};
use notify_rust::{Notification, Urgency::Normal};
use std::{io::Error as IoError, os::unix::net::UnixStream, path::PathBuf, thread::sleep, time::Duration};
fn main() {
	loop {
		let mut conn = get_conn();
		//conn.music_directory only works when connected via unix socket
		let music_dir = conn.music_directory().unwrap();
		notif_loop(conn, music_dir);
	}
}

fn notif_loop(mut conn: Client<UnixStream>, music_dir: String) {
	loop {
		conn.wait(&[Player]).unwrap();
		let status = conn.status();
		if matches!(status, Err(Error::Io(IoError { .. }))) {
			println!("Error: Connection was probaby broken, attempting to get a new connection...");
			break;
		}
		if status.unwrap().state != Play {
			continue;
		}
		let song = conn.currentsong().unwrap().unwrap();
		let (parsed_tags, cover_path) = parse_info(song, &music_dir);
		send_notif(parsed_tags, cover_path);
	}
}

fn send_notif(parsed_tags: String, cover_path: String) {
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

fn get_conn() -> Client<UnixStream> {
	println!("Attempting connection...");
	loop {
		let sock = UnixStream::connect(home_dir().unwrap().join(".config/mpd/socket"));
		if let Ok(socket) = sock {
			let conn = Client::new(socket);
			if let Ok(connection) = conn {
				break connection;
			}
		}
		println!("Connection failed, retrying in one second");
		sleep(Duration::from_secs(1));
	}
}

fn parse_info(song: Song, music_dir: &str) -> (String, String) {
	let mut parsed_tags = String::new();
	let file = PathBuf::from(song.file);

	let cover_path = format!(
		"{}/{}/cover.jpg",
		music_dir,
		file.parent().unwrap_or(&PathBuf::new()).to_string_lossy()
	);

	if let Some(value) = song.title {
		parsed_tags.push_str(&*format!("\n<b>Title:</b>\t<span>{}</span>", &value));
	} else {
		//Insert file name in the event song lacks title tag
		parsed_tags.push_str(&*format!("\n<b>Title:</b>\t<span>{}</span>", &file.file_stem().unwrap().to_string_lossy()));
	}

	let tags = song.tags;

	//tags will only contain duration if song lacks tags
	if tags.len() == 1 {
		return (parsed_tags, cover_path);
	}

	for (key, value) in tags.into_iter() {
		match &*key {
			"Artist" => parsed_tags.push_str(&*format!("\n<b>Artist:</b>\t<span>{}</span>", value)),
			"Album" => parsed_tags.push_str(&*format!("\n<b>Album:</b>\t<span>{}</span>", value)),
			_ => (),
		}
	}
	(parsed_tags, cover_path)
}
