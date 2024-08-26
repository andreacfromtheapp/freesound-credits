# Rationale

## Conventional commits

This project adheres to [Conventional
Commits](https://www.conventionalcommits.org/en/v1.0.0/), [Semantic
Versioning](https://semver.org/spec/v2.0.0.html), and [Keep a
Changelog](https://keepachangelog.com/en/1.0.0/). They can profoundly
intertwine and, when so, they must seamlessly work in concert to fully
appreciate their benefits. With Conventional Commits central to automation
arising from combining these best practices, guaranteeing that all commits
adhere to it becomes paramount. Manually enforcing best practices and
guaranteeing contracts in a code base befalls under the inadequate and
cumbersome category. Thus, this project relies on both [local
tooling](#local-tooling) together with CI
pipelines, to make sure - as best as they can - that changes pushed to this
repository don't break contract.

## Semantic versioning

As defined by [Semantic Versioning 2.0](https://semver.org/):

- Patch (1.\_.x) releases _should solely_ contain bug fixes or documentation
changes. Besides, these releases shouldn't change runtime behavior.

- Minor (1.x) releases may contain new functionality, minor dependency updates,
deprecations, and larger internal implementation changes.

> [!IMPORTANT]
> This project automates versioning and adopts [fully automated
> releases](#fully-automated-releases) with
> [`release-plz`](https://release-plz.ieni.dev).

## Commits and pull requests

For all information about commits and pull requests, please refer to the
[Contributing](/docs/CONTRIBUTING.md#pull-requests) guidelines.

## Intentional code

Always strive to write [intentional
Code](https://www.youtube.com/watch?v=8j4fhsLcT4k). Avoid obscure and
incomprehensible variables and function name and give meaning to code.

## Documentation

Always write documentation comments and spellcheck them to avoid typos and
mistakes.

## Application security

This project strives, in [a constantly evolving
process](https://www.schneier.com/essays/archives/2000/04/the_process_of_secur.html),
to apply [`AppSec`](https://www.ibm.com/topics/application-security) [during
the entire development life
cycle](https://www.youtube.com/watch?v=hDvz8KivY_U), as best as it can.

### Minimum supported rust version

The project strives to stay up to date and on par with Rust releases, to avoid
legacy and stagnation, as `AppSec` best practices dictate.

## Local tooling

Besides standardized [tooling](https://www.rust-lang.org/tools) and
[conventions](https://doc.rust-lang.org/beta/style-guide/index.html), default
`rustfmt` and `clippy`, this project relies on [RustSec](https://rustsec.org)
to inspect supply chain, and [`trufflehog`](https://trufflesecurity.com) to
avoid sharing secrets in the code base. To fix [the crate documentation
comments](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)
and avoid typos in the source tree, this project relies on `cargo-spellcheck`.
Likewise, this project relies on
[`markdownlint-cli2`](https://github.com/DavidAnson/markdownlint-cli2) and
[`pre-commit-hooks`](https://github.com/pre-commit/pre-commit-hooks). All
conveniently enforced with [Git Hooks](https://githooks.com).

## Fully automated releases

Appropriately implementing Conventional Commits enables [fully automated
releases](https://blog.orhun.dev/automated-rust-releases/). For this purpose,
this project relies on [release-plz](https://release-plz.ieni.dev). To
[automate changelog](https://release-plz.ieni.dev/docs/changelog/format). To
handle dependency updates, [semantic version
management](https://release-plz.ieni.dev/docs/semver-check), and
[crates.io](https://crates.io) releases. Upon successful pipelines,
`release-plz` invokes [cargo-dist](https://opensource.axo.dev/cargo-dist/) to
create GitHub releases and packaging for various platforms, along with binary
artifacts and installers.
