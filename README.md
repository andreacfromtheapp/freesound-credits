# Freesound credits

A command lines utility to credit [Freesound](https://freesound.org) samples in
music projects.

## Notes

- It only works with Ableton and Renoise projects.
- It only works with samples adhering to Freesound naming standard.

## Usage

The command line comes with a straight-forward usage function:

```bash
Simple program to generate Freesound credits in a usable markdown file

Usage: freesound_credits --path <PATH> --title <TITLE> --date <DATE> --artist <ARTIST>

Options:
  -p, --path <PATH>      Path to the samples directory
  -t, --title <TITLE>    Song title (quote multiple words)
  -d, --date <DATE>      Song release date (quote multiple words)
  -a, --artist <ARTIST>  Song artist (quote multiple words)
  -h, --help             Print help
  -V, --version          Print version
```

### Ableton

When running against Ableton projects, pass the `--path` to the
`Samples/Imported` directory.

### Renoise

Renoise `xrns` projects need to be extracted with `unzip` first. Once unzipped
you will find a `Song.xml` file and a `SamplesData` directory containing each
`Instrument`. Pass the `--path` to the `SamplesData` directory.
