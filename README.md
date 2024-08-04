# freesound_credits

A command line utility to credit [Freesound](https://freesound.org) samples for
a given project.

## Notes

- It only works with samples adhering to Freesound naming standard.
- It is a quick "rust for fun" just to satisfy a personal need.

## Install

Until I figure out GitHub Actions to build a release from a tag, do the
following:

- download a release or clone this repository.
- unzip and/or `cd` into it
- run `cargo build --release`
- copy the resulting binary in a location available in your `$PATH`
  - e.g: `cp target/release/freesound_credits /usr/local/bin/`

## Usage

The command line comes with a straight-forward usage function:

```text
Simple program to generate Freesound credits in a usable markdown file

Usage: freesound_credits [OPTIONS] --path <PATH> --title <TITLE> --date <DATE> --artist <ARTIST>

Options:
  -p, --path <PATH>      Path to the samples directory
  -t, --title <TITLE>    Song title (quote multiple words)
  -d, --date <DATE>      Song release date (quote multiple words)
  -a, --artist <ARTIST>  Song artist (quote multiple words)
  -z, --zola             Include Zola frontmatter atop the markdown file
  -h, --help             Print help
  -V, --version          Print version
```

### Usage example

1. open your terminal.
2. `cd` to a folder where you want to save the credits file.
3. run `freesound_credits` against your desired samples folder.

## DAW sample folders

### Ableton

When running against Ableton projects, pass the `--path` to the
`Samples/Imported` directory.

### Renoise

Renoise `xrns` projects need to be extracted with `unzip` first. Once unzipped
you will find a `Song.xml` file and a `SamplesData` directory containing each
`Instrument`. Pass the `--path` to the `SamplesData` directory.

### Reaper

When running against Reaper projects, pass the `--path` to the
`Audio Files` directory.

### Logic Pro X

Depending on how you saved your project ([Package vs
Folder](https://www.youtube.com/watch?v=33zVydB4MiI))
, pass the `--path` to the `Audio Files` directory.
