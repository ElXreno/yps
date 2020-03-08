extern crate glob;
extern crate youtube_dl;
#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

fn main() {
    env_logger::init();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("playlist_url")
                .help("URL of playlist")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("destination_folder")
                .help("Destination folder")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .help("Format for downloading")
                .default_value("bestaudio"),
        )
        .arg(
            Arg::with_name("output_file_template")
                .short("t")
                .long("template")
                .help("Output file template")
                .default_value("%(title)s-%(id)s.%(ext)s"),
        )
        .arg(
            Arg::with_name("remove_unknown_files")
                .long("remove-unknown-files")
                .help("Remove files that not exists in playlist (currently requires '%(id)s' in output pattern)"),
        )
        .get_matches();

    let playlist_url = matches.value_of("playlist_url").unwrap();
    let destination_folder = matches.value_of("destination_folder").unwrap();
    let format = matches.value_of("format").unwrap();
    let output_file_template = matches.value_of("output_file_template").unwrap();
    let remove_unknown_files = matches.is_present("remove_unknown_files");

    sync(
        playlist_url,
        destination_folder,
        format,
        output_file_template,
        remove_unknown_files,
    )
}

fn sync(
    playlist_url: &str,
    destination_folder: &str,
    format: &str,
    output_file_template: &str,
    remove_unknown_files: bool,
) -> () {
    println!("Fetching info...");

    let output = YoutubeDl::new(playlist_url)
        .flat_playlist(true)
        .run()
        .unwrap();
    match output {
        YoutubeDlOutput::Playlist(playlist) => {
            let mut playlist_entries = playlist.entries.unwrap();

            let exists_videos = glob::glob(&format!("{}/*", destination_folder)).unwrap();

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
                        "Unknown file '{}' ",
                        &exists_video
                            .as_ref()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                    );

                    if remove_unknown_files {
                        print!(", removing... ");

                        if std::fs::remove_file(&exists_video.unwrap()).is_ok() {
                            println!("Done!");
                        } else {
                            println!("Error!");
                        }
                    } else {
                        println!(", skipping...");
                    }
                }
            }

            for video in playlist_entries {
                println!("Downloading '{}'... ", video.title);

                let url = format!("https://www.youtube.com/watch?v={}", video.id);
                let video_info = YoutubeDl::new(url)
                    .format(format)
                    .download(true)
                    .output_template(format!("{}/{}", destination_folder, output_file_template))
                    .run()
                    .unwrap();

                if let YoutubeDlOutput::None = video_info {
                    println!("Done!")
                }
            }
        }
        YoutubeDlOutput::SingleVideo(video) => {
            println!("It's a video! Uploader: {:#?}", &video.uploader);
        }
        YoutubeDlOutput::None => {}
    }
}
