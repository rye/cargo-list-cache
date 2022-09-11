# `cargo-list-cache`

`cargo-list-cache` prints out all the files in your `${CARGO_HOME}/registry/cache` subdirectory.
This is primarily useful when used with other tools like `less` or ripgrep.

I mainly wrote this so I could see _which packages I have downloaded already_.
When writing new crates or doing anything involving changing dependency lists in an area with poor or no free access to the Internet, this can help with re-using what you already have downloaded. (Cargo has an `--offline` flag which this is intended to help you take advantage of.)


## Usage

```console
$ cargo install cargo-list-cache
$ cargo list-cache
```

Yep. No command-line flags.
