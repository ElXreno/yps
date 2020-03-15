#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

extern crate glob;
extern crate youtube_dl;

use clap::{App, AppSettings, Arg};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

use env_logger::Env;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    trace!("Parsing arguments...");

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("playlist_url")
                .help("URL of playlist")
                .index(1)
                .required(true)
                .value_name("URL"),
        )
        .arg(
            Arg::with_name("destination_folder")
                .help("Destination folder")
                .index(2)
                .required(true)
                .value_name("FOLDER"),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .help("Format for downloading")
                .takes_value(true)
                .value_name("FORMAT")
                .default_value("bestaudio"),
        )
        .arg(
            Arg::with_name("audio_format")
                .long("audio-format")
                .help("Audio format (opus, ogg for instance)")
                .takes_value(true)
                .value_name("FORMAT"),
        )
        .arg(
            Arg::with_name("output_file_template")
                .short("t")
                .long("template")
                .help("Output file template")
                .takes_value(true)
                .value_name("TEMPLATE")
                .default_value("%(title)s-%(id)s.%(ext)s"),
        )
        .arg(
            Arg::with_name("remove_unknown_files")
                .long("remove-unknown-files")
                .help("Remove files that not exists in playlist (currently requires '%(id)s' in output pattern)"),
        )
        .arg(
            Arg::with_name("add_metadata")
                .long("add-metadata")
                .help("Add metadata into supported files"),
        )
        .get_matches();

    let playlist_url = matches.value_of("playlist_url").unwrap();
    let destination_folder = matches.value_of("destination_folder").unwrap();
    let format = matches.value_of("format").unwrap();
    let audio_format = matches.value_of("audio_format");
    let output_file_template = matches.value_of("output_file_template").unwrap();
    let remove_unknown_files = matches.is_present("remove_unknown_files");
    let add_metadata = matches.is_present("add_metadata");

    {
        trace!("Checking destination folder...");

        let destination_folder = PathBuf::from(&destination_folder);
        if !destination_folder.exists() {
            match std::fs::create_dir_all(destination_folder) {
                Ok(()) => println!("Destination dir created successfully!"),
                Err(e) => {
                    error!("'{}'", e);
                    exit(1);
                }
            }
        } else if !destination_folder.is_dir() {
            error!("Destination folder is not a folder! Exiting...");
            exit(1);
        }
    }

    trace!("Starting sync...");

    sync(
        playlist_url,
        destination_folder,
        format,
        audio_format,
        output_file_template,
        remove_unknown_files,
        add_metadata,
    )
}

fn sync(
    playlist_url: &str,
    destination_folder: &str,
    format: &str,
    audio_format: Option<&str>,
    output_file_template: &str,
    remove_unknown_files: bool,
    add_metadata: bool,
) -> () {
    info!("Fetching info about url...");

    let output = YoutubeDl::new(playlist_url)
        .flat_playlist(true)
        .run()
        .unwrap();
    match output {
        YoutubeDlOutput::Playlist(playlist) => {
            let mut videos = playlist.entries.unwrap();

            info!("Fetched playlist, items: {}", videos.iter().count());

            let files = glob::glob(&format!("{}/*", destination_folder)).unwrap();

            for file in files {
                let file = &file.unwrap();

                if file.is_dir() {
                    continue;
                }

                debug!("Checking file '{:?}'", file);

                let is_exists = videos
                    .iter()
                    .position(|s| file.to_str().unwrap().contains(&s.id));

                if is_exists.is_some() {
                    info!(
                        "'{}' already exists, skipping...",
                        videos.get(is_exists.unwrap()).unwrap().title
                    );
                    videos.remove(is_exists.unwrap());
                } else {
                    let filename = file.file_name().unwrap().to_str().unwrap();

                    if remove_unknown_files {
                        warn!("Unknown file '{}', removing...", filename);

                        if std::fs::remove_file(file).is_err() {
                            error!("Failed to remove '{}'!", filename);
                        }
                    } else {
                        warn!("Unknown file '{}', skipping...", filename);
                    }
                }
            }

            for video in videos {
                info!("Downloading '{}'...", video.title);

                let url = format!("https://www.youtube.com/watch?v={}", video.id);

                let mut builder = YoutubeDl::new(url);

                builder
                    .format(format)
                    .download(true)
                    .output_template(format!("{}/{}", destination_folder, output_file_template))
                    .add_metadata(add_metadata);

                if let Some(audio_format) = audio_format {
                    builder.audio_format(audio_format);
                }

                let result = builder.run().unwrap();

                if let YoutubeDlOutput::None = result {
                    info!("Done!")
                }
            }
        }
        YoutubeDlOutput::SingleVideo(_) => {
            error!("It's a video, not playlist!");
        }
        YoutubeDlOutput::None => {}
    }

    info!("Work complete!")
}
