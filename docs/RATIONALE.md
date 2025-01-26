# Rationale

## OrgMode

> [OrgMode](https://orgmode.org) is one of the best tools for keeping on
> track with what needs to be done and when. And, contrary to myth, it
> doesn\'t need [Emacs](https://www.youtube.com/watch?v=cxoE2FhOIgI),
> just a text editor. Org can be used with a variety of other tools and
> editors once you understand the basics.
>
> Org, at its very heart, is a structured text file. It has headers,
> subheaders, and keywords that allow other tools to parse files into
> agendas and to-do lists.
>
> -- <https://opensource.com/article/19/1/productivity-tool-org-mode>

Contrary to myth, you don\'t need
[Emacs](https://www.youtube.com/watch?v=cxoE2FhOIgI) to adopt
[OrgMode](https://orgmode.org) - unless you want to. Most flat-text
editor have plugins that help create and manage Org files:

-   <https://vscode-org-mode.github.io/vscode-org-mode/>
-   <https://github.com/jceb/vim-orgmode>
-   <https://github.com/nvim-orgmode/orgmode>
-   <https://github.com/nvim-neorg/neorg>

## Conventional Commits

This project adheres to [Conventional
Commits](https://www.conventionalcommits.org/en/v1.0.0/), [Semantic
Versioning](https://semver.org/spec/v2.0.0.html), and [Keep a
Changelog](https://keepachangelog.com/en/1.0.0/). They can profoundly
intertwine and, when so, they must seamlessly work in concert to fully
appreciate their benefits. With Conventional Commits central to
automation arising from combining these best practices, guaranteeing
that all commits adhere to it becomes paramount. Manually enforcing best
practices and guaranteeing contracts in a code base befalls under the
inadequate and cumbersome category. Thus, this project relies on both
[local tooling](#local-tooling) together with CI pipelines, to make
sure - as best as they can - that changes pushed to this repository
don\'t break contract.

## Semantic Versioning

As defined by [Semantic Versioning 2.0](https://semver.org/):

-   Patch (1.\_.x) releases *should solely* contain bug fixes or
    documentation changes. Besides, these releases shouldn\'t change
    runtime behavior.

-   Minor (1.x) releases may contain new functionality, minor dependency
    updates, deprecations, and larger internal implementation changes.

> \[!IMPORTANT\] This project automates versioning and adopts [fully
> automated releases](#fully-automated-releases) with
> [`release-plz`{.verbatim}](https://release-plz.ieni.dev).

## Commits and Pull Requests

For all information about commits and pull requests, please refer to the
[Contributing](file:///docs/CONTRIBUTING.org#pull-requests) guidelines.

## Intentional Code

Always strive to write [intentional
Code](https://www.youtube.com/watch?v=8j4fhsLcT4k).
`Intentional Code`{.verbatim} is much more than avoiding obscure and
incomprehensible variables and functions names; however, these are a
good start give meaning to code.

## Documentation

Always write [documentation
comments](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) and
spellcheck them to avoid typos and mistakes.

## Application Security

This project strives, in [a constantly evolving
process](https://www.schneier.com/essays/archives/2000/04/the_process_of_secur.html),
to apply
[`AppSec`{.verbatim}](https://www.ibm.com/topics/application-security)
[during the entire development life
cycle](https://www.youtube.com/watch?v=hDvz8KivY_U), as best as it can.

### Minimum Supported Rust Version

The project strives to stay up to date and on par with Rust releases, to
avoid legacy and stagnation, as `AppSec`{.verbatim} best practices
dictate.

## Local Tooling

Besides standardized [tooling](https://www.rust-lang.org/tools) and
[conventions](https://doc.rust-lang.org/beta/style-guide/index.html),
default `rustfmt`{.verbatim} and `clippy`{.verbatim}, this project
relies on [RustSec](https://rustsec.org) to inspect supply chain, and
[`trufflehog`{.verbatim}](https://trufflesecurity.com) to avoid sharing
secrets in the code base. Likewise, this project relies on
[`markdownlint-cli2`{.verbatim}](https://github.com/DavidAnson/markdownlint-cli2)
and
[`pre-commit-hooks`{.verbatim}](https://github.com/pre-commit/pre-commit-hooks).
All conveniently enforced with [Git Hooks](https://githooks.com).

## Fully Automated Releases

Appropriately implementing Conventional Commits enables [fully automated
releases](https://blog.orhun.dev/automated-rust-releases/). For this
purpose, this project relies on
[release-plz](https://release-plz.ieni.dev). To [automate
changelog](https://release-plz.ieni.dev/docs/changelog/format). To
handle dependency updates, [semantic version
management](https://release-plz.ieni.dev/docs/semver-check), and
[crates.io](https://crates.io) releases. Upon successful pipelines,
`release-plz`{.verbatim} invokes
[cargo-dist](https://opensource.axo.dev/cargo-dist/) to create GitHub
releases and packaging for various platforms, along with binary
artifacts and installers.
