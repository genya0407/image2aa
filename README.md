# png2aa

Convert PNG image file to ascii art.

## Versions

This package requires rustc version `nightly-2017-12-21`.

## Run

### CLI tool

Run with `cargo run`:

```shell-session
$ cargo run --release --bin png2aa -- -i path/to/image.ong
```

Or, build binary:

```shell-session
$ cargo build --release --bin png2aa
$ ./target/release/png2aa -i path/to/image.png
```

### Web application

Run with `cargo run`:

```shell-session
$ cargo run --release --bin png2aa-web
```

Then, go to [http://localhost:8000](http://localhost:8000).

Or, build binary:

```shell-session
$ cargo build --release --bin png2aa-web
$ ./target/release/png2aa-web
```
