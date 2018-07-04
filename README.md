# MailX

Tries to guess the email of a person based on his name and surname.

It tries common combinations and let you know if they are real or not.

## Requirements

This projects needs `libresolv.so` in order to run as it relies on [resolv](https://crates.io/crates/resolv) library.

## Compile with Docker

    docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.23.0 cargo run -- -d gmail.com -n john -s doe


## Ideas

- [ ] Use only one conexion.
- [ ] Generate binaries.
- [ ] Use external file of emails.
- [ ] Jump to another MX if current fails.
- [ ] Make it more configurable
    - Change from from args
    - Pass specific email to check
- [ ] Add more visual output (Colors).
- [ ] Add Tests.
- [ ] Improve errors.
- [ ] Add pipeline option.