# yps 

[![github actions](https://github.com/ElXreno/yps/workflows/Rust/badge.svg)](https://github.com/ElXreno/yps/actions)
[![dependency status](https://deps.rs/repo/github/elxreno/yps/status.svg)](https://deps.rs/repo/github/elxreno/yps)

**yps - YouTube Playlist Sync utility writen in Rust.**

---

```
yps 0.1.0
ElXreno <elxreno@gmail.com>


USAGE:
    yps [FLAGS] [OPTIONS] <URL> <FOLDER>

FLAGS:
        --add-metadata            Add metadata into supported files
    -h, --help                    Prints help information
        --remove-unknown-files    Remove files that not exists in playlist (currently requires '%(id)s' in output
                                  pattern)
    -V, --version                 Prints version information

OPTIONS:
        --audio-format <FORMAT>    Audio format (opus, ogg for instance)
    -f, --format <FORMAT>          Format for downloading [default: bestaudio]
    -t, --template <TEMPLATE>      Output file template [default: %(title)s-%(id)s.%(ext)s]

ARGS:
    <URL>       URL of playlist
    <FOLDER>    Destination folder
```

## Example:
### Simple:
```bash
yps https://www.youtube.com/playlist?list=PLmPVxv5DEDa1W3UN7rujKcABkra9k-Jjh ~/Music/youtube-playlist
```

### Custom:
```bash
yps --remove-unknown-files -t '%(title)s-%(id)s_%(format_id)s.%(ext)s' --format bestvideo+bestaudio --add-metadata \ 
    https://www.youtube.com/playlist?list=PLmPVxv5DEDa1W3UN7rujKcABkra9k-Jjh \
    ~/Music/youtube-playlist
```

### Debugging:
```bash
RUST_LOG=trace yps https://www.youtube.com/playlist?list=PLmPVxv5DEDa1W3UN7rujKcABkra9k-Jjh ~/Music/youtube-playlist
```

More info about output template you can find [here](https://github.com/ytdl-org/youtube-dl/blob/master/README.md#output-template).