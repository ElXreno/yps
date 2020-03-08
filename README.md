# yps 

[![github actions](https://github.com/ElXreno/yps/workflows/Rust/badge.svg)](https://github.com/ElXreno/yps/actions)
[![dependency status](https://deps.rs/repo/github/elxreno/yps/status.svg)](https://deps.rs/repo/github/elxreno/yps)

**yps - YouTube Playlist Sync utility writen in Rust.**

---

## Usage:
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

## Example:
### Simple:
```bash
yps https://www.youtube.com/playlist?list=PLmPVxv5DEDa1W3UN7rujKcABkra9k-Jjh ~/Music/youtube-playlist
```

### Custom:
```bash
yps --remove-unknown-files -p '%(title)s-%(id)s_%(format_id)s.%(ext)s' --format bestvieo+bestaudio \ 
    https://www.youtube.com/playlist?list=PLmPVxv5DEDa1W3UN7rujKcABkra9k-Jjh \
    ~/Music/youtube-playlist
```

More info about output pattern you can find [here](https://github.com/ytdl-org/youtube-dl/blob/master/README.md#output-template).