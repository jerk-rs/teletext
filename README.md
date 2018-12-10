# Teletext

A telegram bot to transform text

[![Travis](https://img.shields.io/travis/jerk-rs/teletext.svg?style=flat-square)](https://travis-ci.org/jerk-rs/teletext)

# Available commands:

- `/arrow`
- `/huify`
- `/square`
- `/star`
- `/sw`

# Running

```sh
$ git clone https://github.com/rossnomann/teletext && cd teletext
$ rustup override set nightly
$ cargo build --release
$ # or download from releases page on github
$ cat <<EOF > .env
TELETEXT_TOKEN=your-bot-token
EOF
$ /target/release/teletext
```

# Changelog

### 0.1.4 (10.12.2018)

- Migrated to teleborg.
- Added replies support.
- Added `/huify` command.
- Renamed `/qstar` to `/arrow`.

### 0.1.3 (27.05.2018)

- Added `/sw` command.

### 0.1.2 (27.05.2018)

- Handle bot name.

### 0.1.1 (26.05.2018)

- Check text len before conversion.

### 0.1.0 (26.05.2018)

- First release.

## LICENSE

The MIT License (MIT)
