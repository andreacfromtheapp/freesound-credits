# freesound_credits

A command line utility to credit [Freesound](https://freesound.org) samples for
a given project.

## Install

This is a command line utility. To install:

- download and unzip the release for your system.
- copy the binary to a location available in your `$PATH`
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

1. open your favorite terminal.
2. `cd` to the folder where to save the credits file.
3. run `freesound_credits` on your desired samples folder.
4. if necessary, use the absolute path to the samples folder.

#### Ableton Example

Running against an Ableton `Samples/Imported` directory, will generate a
markdown file named `field-notes-credits.md` in the current directory.

```bash
freesound_credits -p Samples/Imported/ -t "Field Notes" -a "Aner Andros" -d "2017-10-28" -z
```

## Supported filenames

`freesound_credits` matches samples adhering to Freesound naming standard that
kept their original samples names per downloads from the platform. For example:

- new standard with double underscore: `69604__timkahn__subverse_whisper.wav`
- old standard with single underscore: `2166_suburban_grilla_bowl_struck.flac`

## Supported DAWs

`freesound_credits` should work for any samples stored in a flat folder.
`freesound_credits` filters out metadata files associated with audio imports.
To add more DAWs and associated metadata or extraction, please file an issue.

| DAW | Samples Folder | Metadata | Extraction | Notes |
| :----: | :----: | :----: | :----: | :----: |
| Ableton | `Samples/Imported` | `.asd` | no | |
| Renoise | `SamplesData` | no | `unzip` | [extraction](#renoise-extraction) |
| Repaer  | `Audio Files` | `.reapeaks` | no | |
| Logic Pro X  | `Audio Files` | no | no | [Package vs Folder](https://www.youtube.com/watch?v=33zVydB4MiI) |

## Extra DAWs steps

### Renoise extraction

Extract with `unzip your_project.xrns` first. Once unzipped, you will find a
`Song.xml` file and a `SamplesData` directory containing each `Instrument`.
