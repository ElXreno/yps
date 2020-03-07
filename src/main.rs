extern crate glob;
extern crate youtube_dl;

use youtube_dl::{YoutubeDl, YoutubeDlOutput};

const MUSIC_FOLDER: &str = "/home/elxreno/tmp/yt-dl-tmp";
const FILE_PATTERN: &str = "%(title)s-%(id)s.%(ext)s";

fn main() {
    env_logger::init();

    let output =
        YoutubeDl::new("https://www.youtube.com/playlist?list=PLbq7XaEqQjg8aEvsvU7E1nQo_JIONI8je")
            .flat_playlist(true)
            .run()
            .unwrap();

    match output {
        YoutubeDlOutput::Playlist(playlist) => {
            let mut playlist_entries = playlist.entries.unwrap();

            let exists_videos = glob::glob(&format!("{}/*", MUSIC_FOLDER)).unwrap();

            for exists_video in exists_videos {
                let exists = playlist_entries.iter().position(|s| {
                    exists_video
                        .as_ref()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .contains(&s.id)
                });

                if exists.is_some() {
                    println!(
                        "'{}' already exists, skipping...",
                        playlist_entries.get(exists.unwrap()).unwrap().title
                    );

                    playlist_entries.remove(exists.unwrap());
                } else {
                    print!(
                        "Unknown file '{}', removing... ",
                        &exists_video
                            .as_ref()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                    );
                    if std::fs::remove_file(&exists_video.unwrap()).is_ok() {
                        println!("Done!");
                    } else {
                        println!("Error!");
                    }
                }
            }

            for video in playlist_entries {
                println!("Downloading '{}'...", video.title);

                let url = format!("https://www.youtube.com/watch?v={}", video.id);
                let video_info = YoutubeDl::new(url)
                    .format("bestaudio")
                    .download(true)
                    .output_pattern(format!("{}/{}", MUSIC_FOLDER, FILE_PATTERN))
                    .run()
                    .unwrap();

                if let YoutubeDlOutput::None = video_info {
                    println!("Downloaded '{}'!", video.title)
                }
            }
        }
        YoutubeDlOutput::SingleVideo(video) => {
            println!("It's a video! Uploader: {:#?}", &video.uploader);
        }
        YoutubeDlOutput::None => {}
    }
}
