# Teletext

A telegram bot to transform text

[![Travis](https://img.shields.io/travis/jerk-rs/teletext.svg?style=flat-square)](https://travis-ci.org/jerk-rs/teletext)

# Available commands:

- `/square`
- `/star`
- `/sw`
- `/qstar`

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

You can add `ALL_PROXY='socks5h://user:pass@host:port'`
to `.env` file in order to use a socks5 proxy.

# Changelog

### 0.1.3 (27.05.2018)

- Add sw command.

### 0.1.2 (27.05.2018)

- Handle bot name.

### 0.1.1 (26.05.2018)

- Check text len before conversion.

### 0.1.0 (26.05.2018)

- First release.

## LICENSE

The MIT License (MIT)
