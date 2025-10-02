# Freesound credits

A command line utility to help you credit [Freesound](https://freesound.org)
samples for a given project more easily.

[![Commitlint](https://github.com/andreacfromtheapp/freesound-credits/actions/workflows/commitlint.yml/badge.svg)](https://github.com/andreacfromtheapp/freesound-credits/actions/workflows/commitlint.yml)
[![Trufflehog](https://github.com/andreacfromtheapp/freesound-credits/actions/workflows/trufflehog.yml/badge.svg)](https://github.com/andreacfromtheapp/freesound-credits/actions/workflows/trufflehog.yml)
[![CI](https://github.com/andreacfromtheapp/freesound-credits/actions/workflows/ci.yml/badge.svg)](https://github.com/andreacfromtheapp/freesound-credits/actions/workflows/ci.yml)
[![CD](https://github.com/andreacfromtheapp/freesound-credits/actions/workflows/release-plz.yml/badge.svg)](https://github.com/andreacfromtheapp/freesound-credits/actions/workflows/release-plz.yml)
[![Release](https://github.com/andreacfromtheapp/freesound-credits/actions/workflows/release.yml/badge.svg)](https://github.com/andreacfromtheapp/freesound-credits/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/freesound%2Dcredits.svg)](https://crates.io/crates/freesound-credits)
[![License:
Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Rationale

Giving credits in the [Creative Commons](https://creativecommons.org) community
is often the sole requirement to freely use samples in your creations. This tool
helps you abide to [Copyleft](https://en.wikipedia.org/wiki/Copyleft) and credit
the generous artists whom contribute to [Freesound](https://freesound.org),
helping your creativity.

Giving credits may have been a cumbersome and tedious process you begrudged.
This may have been stopping you from giving credits altogether. Not anymore!

Thanks to `freesound-credits` it takes seconds now! Read on for instructions.

## Install

### Releases

All currently available builds for Linux, macOS, and Windows, are available on
[the releases page](https://github.com/andreacfromtheapp/freesound-credits/releases)

### Cargo

Use [`cargo binstall`](https://github.com/cargo-bins/cargo-binstall) to install
`freesound-credits` directly from GitHub:

```shell
cargo binstall freesound-credits
```

To compile `freesound-credits` from source, run:

```shell
cargo install freesound-credits
```

### Homebrew

On Linux and macOS homebrew is also available:

```shell
brew install andreacfromtheapp/tap/freesound-credits
```

## Usage

The command line comes with a straight-forward usage function:

```bash
freesound-credits -h
```

### Default template

By default, `freesound-credits` will use the following template, to use with
[Zola's](https://www.getzola.org/) front-matter. This is hardcoded into the
`set_frontmatter` function, to facilitate usage without needing unnecessary
files on users' filesystem. Unless a user wants to use and manage a custom
template.

```toml
title="{song_title} Credits"
date={song_date}
author="{song_artist}"

[taxonomies]
tags=["Freesound", "{song_artist}", "Credits""]
```

### Custom template

You can customize the template per your needs, then save it and use it with the
`-f` command option. If necessary, use the absolute path to the template. The
template **must** be saved as a [TOML](https://toml.io/en/) file.

> [!IMPORTANT]
>
> When customizing the template, you must use the expected variables within the
> template:
>
> `{song_title}`, `{song_date}`, and `{song_artist}`

### Usage example

1. Open your favorite terminal.
2. `cd` to the folder where you want to save the credits file.
3. Run `freesound-credits` on your desired samples folder.
4. If necessary, use the absolute path to the samples folder.
5. If necessary, use the absolute path to the custom template.

```bash
# Uses default frontmatter
freesound-credits -s samples/ -t "My Song" -d 2023-05-15 -a "Artist" -w

# Uses a custom template
freesound-credits -s samples/ -t "My Song" -d 2023-05-15 -a "Artist" -w -f template.toml
```

#### Ableton Example

Running against an Ableton `Samples/Imported` directory, will generate a
markdown file named `field-notes-credits.md` in the directory where the command
is run.

```bash
freesound-credits -s Samples/Imported/ -t "My Song" -a "Artist" -d 2023-05-13 -w -f template.toml
```

## Supported filenames

`freesound-credits` matches samples adhering to Freesound naming standard that
kept their original samples names per downloads from the platform. For example:

- new standard with double underscore: `69604__timkahn__subverse_whisper.wav`
- old standard with single underscore: `2166_suburban_grilla_bowl_struck.flac`

## Supported DAWs

`freesound-credits` should work for any samples stored in a flat folder.
`freesound-credits` filters out metadata files associated with audio imports. To
add more DAWs and associated metadata or extraction, please
[file an issue](https://github.com/andreacfromtheapp/freesound-credits/issues/new?assignees=&labels=enhancement&projects=&template=add_a_new_daw.yml&title=feat%28DAW%29%3A+add+).

|     DAW     |   Samples Folder   |  Metadata   | Extraction |                              Notes                               |
| :---------: | :----------------: | :---------: | :--------: | :--------------------------------------------------------------: |
|   Ableton   | `Samples/Imported` |   `.asd`    |     no     |                                                                  |
|   Reaper    |   `Audio Files`    | `.reapeaks` |     no     |                                                                  |
|   Renoise   |   `SamplesData`    |     no      |  `unzip`   |                [extraction](#renoise-extraction)                 |
| Logic Pro X |   `Audio Files`    |     no      |     no     | [Package vs Folder](https://www.youtube.com/watch?v=33zVydB4MiI) |

### Adding more DAWs

I did the best I could with what I have. I've tested it with Ableton, Reaper,
and Renoise projects on macOS.

Logic Pro X uses no metadata and a clean audio files folder, thus I decided to
add it but it hasn't been tested. However, any DAW with these characteristics
should work out of the box.

I would appreciate anybody confirming more DAWs or
[adding new ones](https://github.com/andreacfromtheapp/freesound-credits/issues/new?assignees=&labels=enhancement&projects=&template=add_a_new_daw.yml&title=feat%28DAW%29%3A+add+).
When you do request or add a new DAW, please keep the
[above table](#supported-daws) and the [extra steps](#extra-daws-steps) section
in mind for any extra information.

## Extra DAWs steps

### Renoise extraction

Extract with `unzip your_project.xrns` first. Once unzipped, you will find a
`Song.xml` file and a `SamplesData` directory containing each `Instrument`.

## Contributing

Thanks for your help improving the project! :balloon: Happy to have you! Peruse
the [contributing guide](./docs/CONTRIBUTING.md) to help you get involved in the
project.
