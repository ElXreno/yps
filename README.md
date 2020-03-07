# yps - YouTube-Playlist-Sync ![Rust](https://github.com/ElXreno/yps/workflows/Rust/badge.svg)

Please, do not use this trash code! It's a experiment!

```
yps 0.1.0
ElXreno <elxreno@gmail.com>


USAGE:
    yps [FLAGS] [OPTIONS] <playlist_url> <destination_folder>

FLAGS:
    -h, --help                    Prints help information
        --remove-unknown-files    Remove files that not exists in playlist (currently requires '%(id)s' in output
                                  pattern)
    -V, --version                 Prints version information

OPTIONS:
    -p, --pattern <file_pattern>    Output file pattern [default: %(title)s-%(id)s.%(ext)s]
    -f, --format <format>           Format for downloading [default: bestaudio]

ARGS:
    <playlist_url>          URL of playlist
    <destination_folder>    Destination folder
```