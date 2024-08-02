# Freesound credits

A simple utility to credit [Freesound](https://freesound.org) samples in my
music projects.

## Notes (don't expect much)

- It only works with Ableton and Renoise projects.
- It only works with samples adhering to Freesound naming standard.
- It is a quick "rust for fun" just to satisfy a personal need.
- The code most certainly stinks and could be improved in the future.

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

### Ableton

When running against Ableton projects, pass the `--path` to the
`Samples/Imported` directory.

### Renoise

Renoise `xrns` projects need to be extracted with `unzip` first. Once unzipped
you will find a `Song.xml` file and a `SamplesData` directory containing each
`Instrument`. Pass the `--path` to the `SamplesData` directory.
