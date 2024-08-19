# Contributing

Thank you for investing your time in contributing to this project!

**Contributions of all sizes welcome and valued here.** :pray:

In this guide you will get an overview of the contribution workflow from
opening an issue, creating a PR, reviewing, and merging the PR.

To get an overview of the project's best practices, tooling, and automation,
read the [rationale](#rationale). :information_source:

## Conduct

This project adheres to the [Rust Code of Conduct](./CODE_OF_CONDUCT.md). This
describes the _minimal_ behavior expected from all contributors.

## New contributor guide

Some resources to help you get started with open source contributions:

- [Finding ways to contribute to open source on
GitHub](https://docs.github.com/en/get-started/exploring-projects-on-github/finding-ways-to-contribute-to-open-source-on-github)
- [Set up
Git](https://docs.github.com/en/get-started/getting-started-with-git/set-up-git)
- [Collaborating with pull
requests](https://docs.github.com/en/github/collaborating-with-pull-requests)

## Contributing in issues

For any issue, the three fundamental ways an individual can contribute:

1. By opening the issue for discussion: For instance, if you believe that you
have discovered a bug.

2. By helping to triage the issue: Providing supporting details (a test case
that demonstrates a bug), providing suggestions on how to deal with the issue,
or ensuring that the issue has appropriate tags.

3. By helping to resolve the issue: Typically either in the form of
demonstrating that the issue reported ain't a problem after all, or more often,
by opening a Pull Request that changes some bit of something in a concrete and
reviewable manner.

**Anybody can partake in any stage of contribution**. This project encourages
you to partake in the discussion around bugs and partake in reviewing pull
requests.

### Asking for general help

If you have reviewed existing documentation and still have questions or
problems, you can [open a
discussion](https://github.com/gacallea/freesound-credits/discussions/new)
asking for help.

In exchange for receiving help, this project asks that you contribute back a
documentation PR that helps others avoid the problems that you encountered.

### Submitting a bug report

If you spot a problem, [search if an issue already
exists](https://docs.github.com/en/github/searching-for-information-on-github/searching-on-github/searching-issues-and-pull-requests#search-by-the-title-body-or-comments).
If a related issue doesn't exist, you can open a new issue using a relevant
[issue form](https://github.com/gacallea/freesound-credits/issues/new/choose).
Please fill out this form, following the template to the best of your ability.

The two most important pieces of information: a description of the behavior you
see and a test case to recreate the problem. See [How to create a Minimal,
Complete, and Verifiable example](https://stackoverflow.com/help/mcve).

### Triaging a bug report

Common practice includes having a discussion around open issues. Some
contributors may have differing opinions about the issue, including whether the
behavior constitute a bug or a feature. This discussion should stay focused,
helpful, and professional.

Short, clipped responses that give neither added context nor supporting detail
ain't helpful nor professional. To most, such responses seem annoying and
unfriendly.

This project encourages contributors to help one another make forward progress
as much as possible. Empowering one another to solve issues collaboratively.
Wether you feel either doesn't qualify as a problem, or if you happen upon
information that you feel incongruous. When commenting on an issue, explain why
you feel that way. With supporting context. Willing to concede misconstruing.
By doing so, often leads the correct outcome much faster.

### Resolving a bug report

In the majority of cases, resolving issues involve opening a Pull Request. The
process for opening and reviewing a Pull Request resembles that of opening and
triage issues. Although it carries with it a necessary review and approval
workflow that ensures that the proposed changes meet the minimal quality and
functional guidelines of the project.

## Making changes

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

### Installing requirements

> [!IMPORTANT]
> Contributors necessarily need installing and perusing tooling
> locally guaranteeing contracts before committing any changes.

- If necessary, [set up Rust](https://www.rust-lang.org/tools/install) with
`rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install [`cargo
auditable`](https://github.com/rust-secure-code/cargo-auditable): `cargo
install cargo-auditable --locked`
  - Set up a [shell alias for `cargo
  auditable`](https://github.com/rust-secure-code/cargo-auditable?tab=readme-ov-file#can-i-make-cargo-always-build-with-cargo-auditable)
- Install [`cargo deny`](https://embarkstudios.github.io/cargo-deny/): `cargo
install cargo-deny --locked`
- Install [`cargo-spellcheck`](https://github.com/drahnr/cargo-spellcheck):
`cargo install cargo-spellcheck --locked`
- Install [`trufflehog`](https://trufflesecurity.com/trufflehog): `brew install
trufflehog`
  - Otherwise, download a binary from [`trufflehog` release
  page](https://github.com/trufflesecurity/trufflehog/releases).
- Install [`pre-commit`](https://pre-commit.com): `pip install pre-commit`

### Making changes locally

- 1 Fork the repository.

- Using GitHub Desktop:
  - [Getting started with GitHub
  Desktop](https://docs.github.com/en/desktop/installing-and-configuring-github-desktop/getting-started-with-github-desktop)
  will guide you through setting up Desktop.
  - Once you set up Desktop, you can use it to [fork the
  repository](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/cloning-and-forking-repositories-from-github-desktop)!

- Using the command line:
  - [Fork the
  repository](https://docs.github.com/en/github/getting-started-with-github/fork-a-repo#fork-an-example-repository)
  so that you can make your changes without affecting the original project
  until you're ready to merge them.

- 2 Change into the cloned repository: `cd freesound-credits`.

- 3 Install [Git Hooks](https://githooks.com) with `pre-commit install
--install-hooks`

- 4 Create a working branch and start with your changes!

## Pull requests

Pull Requests are the way concrete changes are made to the code, documentation,
and dependencies in the repository.

Even tiny pull requests (e.g: one character pull request fixing a typo in API
documentation) highly appreciated. Before making a large change, first open an
issue describing the change to seek feedback and guidance. This will increase
the likelihood of the PR getting merged.

When you completed the changes, create a pull request, also known as a PR.

- Don't forget to [link PR to
issue](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue),
should you be solving one.
- Enable the checkbox to [allow maintainer
edits](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/allowing-changes-to-a-pull-request-branch-created-from-a-fork)
. Once you submit your PR, a team member will review your proposal. The team
may ask questions or request farther information.
- The team may ask for farther changes before merging a PR, either using
[suggested
changes](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/incorporating-feedback-in-your-pull-request)
or pull request comments. You can apply suggested changes directly through the
UI. You can make any other changes in your fork, then commit them to your
branch.
- As you update your PR and apply changes, mark each conversation as
[resolved](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/commenting-on-a-pull-request#resolving-conversations).
- If you run into any merge issues, checkout this [git
tutorial](https://github.com/skills/resolve-merge-conflicts) to help you
resolve merge conflicts and other issues.

### Performing spell-check on the codebase

To perform spell-check on the codebase, run:

```shell
cargo spellcheck checkout
```

For details of how to use the spellcheck tool, visit
[https://github.com/drahnr/cargo-spellcheck](https://github.com/drahnr/cargo-spellcheck)

Should the command decline a word, you should backtick the declined word if
code related. If not, the rejected word should go into `spellcheck.dic` file.

When you add a word into the file, you should also update the first line which
tells the spellcheck tool the total number of words included in the file

### Testing

If the change being proposed alters code (as opposed to only documentation for
example), it is either adding new functionality to the project or it is fixing
existing, broken functionality. In both of these cases, the pull request should
include one or more tests to ensure that the project does not regress in the
future. There are three ways to write tests: unit tests, integration tests, and
documentation tests.

#### Integration tests

Integration tests go in the same crate as the code they are testing. Each sub
crate should have a `dev-dependency` on `the crate` itself. This makes all
utilities available to use in tests, no matter the crate being tested.

The best strategy for writing a new integration test is to look at existing
integration tests in the crate and follow the style.

#### Fuzz tests

Some of our crates include a set of fuzz tests, this will be marked by a
directory `fuzz`. It is a good idea to run fuzz tests after each change.
To get started with fuzz testing you'll need to install
[cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

`cargo install --locked cargo-fuzz`

To list the available fuzzing harnesses you can run:

```bash
$ cargo fuzz list
fuzz_linked_list
````

Running a fuzz test is as simple as;

`cargo fuzz run fuzz_linked_list`

> [!NOTE]
> Keep in mind that by default when running a fuzz test the fuzz harness will
> run forever and will only exit if you `ctrl-c` or it finds a bug.

#### Documentation tests

Ideally, every API has at least one [documentation test] that demonstrates how to
use the API. Documentation tests are run with `cargo test --doc`. This ensures
that the example is correct and provides additional test coverage.

The trick to documentation tests is striking a balance between being succinct
for a reader to understand and actually testing the API.

Same as with integration tests, when writing a documentation test, the full
crate is available. This is especially useful for getting access to the
runtime to run the example.

The documentation tests will be visible from both the crate specific
documentation **and** the project facade documentation via the re-export. The
example should be written from the point of view of a user that is using the
crate. As such, the example should use the API via the facade and not by
directly referencing the crate.

### Committing updates

Best practice recommend to keep your changes as logically grouped as possible
within individual commits. There is no limit to the number of commits any
single Pull Request may have, and many contributors find it easier to review
changes that are split across multiple commits.

That said, if you have a number of commits that are "checkpoints" and don't
represent a single logical change, please squash those together.

Note that multiple commits often get squashed when they are landed (see the
notes about [commit squashing](#commit-squashing)).

#### Commit message guidelines

Before committing any changes, make sure to run `pre-commit run --all-files`
and that it returns no issues. Should it present any warning or issue: please
fix, rinse, and repeat. Once happy with them, commit the changes. Make sure to
use "[Conventional
Commits](https://www.conventionalcommits.org/en/v1.0.0/#why-use-conventional-commits)".

A good commit message should describe what changed and why.

- 1 The first line should:

- contain a short description of the change (preferably 50 characters or less,
and no more than 72 characters)
  - be entirely in lowercase with the exception of proper nouns, acronyms, and
  the words that refer to code, like function/variable names
  - start with an imperative verb
  - not have a period at the end
  - be prefixed with the name of the module being changed; usually this is the
  same as the M-* label on the PR

  Examples:

  - time: introduce `Timeout` and deprecate `Deadline`
  - codec: export `Encoder`, `Decoder`, `Framed*`
  - ci: fix the FreeBSD ci configuration

- 2 Keep the second line blank.
- 3 Wrap all other lines at 72 columns (except for long URLs).
- 4 If your patch fixes an open issue, you can add a reference to it at the end
of the log. Use the `Fixes: #` prefix and the issue number. For other
references use `Refs: #`. `Refs` may include multiple issues, separated by a
comma.

  Examples:

  - `Fixes: #1337`
  - `Refs: #1234`

Sample complete commit message:

```txt
module: explain the commit in one line

Body of commit message is a few lines of text, explaining things in more
detail, possibly giving some background about the issue being fixed, etc.

The body of the commit message can be several paragraphs, and please do proper
word-wrap and keep columns shorter than about 72 characters or so. That way,
`git log` will show things nicely even when it is indented.

Fixes: #1337
Refs: #453, #154
```

### Opening the Pull Request

From within GitHub, opening a new Pull Request will present you with a
[template] that should be filled out. Please try to do your best at filling out
the details, but feel free to skip parts if you're not sure what to put.

[template]: .github/PULL_REQUEST_TEMPLATE.md

### Discuss and update

You will probably get feedback or requests for changes to your Pull Request.
This is a big part of the submission process so don't be discouraged! Some
contributors may sign off on the Pull Request right away, others may have more
detailed comments or feedback. This is a necessary part of the process in order
to evaluate whether the changes are correct and necessary.

**Any community member can review a PR and you might get conflicting
feedback**. Keep an eye out for comments from code owners to provide guidance
on conflicting feedback.

**Once the PR is open, do not rebase the commits**. See [Commit
Squashing](#commit-squashing) for more details.

### Commit Squashing

In most cases, **do not squash commits that you add to your Pull Request during
the review process**. When the commits in your Pull Request land, they may be
squashed into one commit per logical change. Metadata will be added to the
commit message (including links to the Pull Request, links to relevant issues,
and the names of the reviewers). The commit history of your Pull Request,
however, will stay intact on the Pull Request page.

## Reviewing Pull Requests

**Any community member is welcome to review any pull request**.

All contributors who choose to review and provide feedback on Pull
Requests have a responsibility to both the project and the individual making
the contribution. Reviews and feedback must be helpful, insightful, and geared
towards improving the contribution as opposed to simply blocking it. If there
are reasons why you feel the PR should not land, explain what those are. Do not
expect to be able to block a Pull Request from advancing simply because you say
"No" without giving an explanation. Be open to having your mind changed. Be
open to working with the contributor to make the Pull Request better.

Reviews that are dismissive or disrespectful of the contributor or any other
reviewers are strictly counter to the Code of Conduct.

When reviewing a Pull Request, the primary goals are for the codebase to
improve and for the person submitting the request to succeed. **Even if a Pull
Request does not land, the submitters should come away from the experience
feeling like their effort was not wasted or unappreciated**. Every Pull Request
from a new contributor is an opportunity to grow the community.

### Review a bit at a time

Do not overwhelm new contributors.

It is tempting to micro-optimize and make everything about relative
performance, perfect grammar, or exact style matches. Do not succumb to that
temptation.

Focus first on the most significant aspects of the change:

1. Does this change make sense for the project?
2. Does this change make the project better, even if only incrementally?
3. Are there clear bugs or larger scale issues that need attending to?
4. Is the commit message readable and correct? If it contains a breaking change
is it clear enough?

Note that only **incremental** improvement is needed to land a PR. This means
that the PR does not need to be perfect, only better than the status quo.
Follow up PRs may be opened to continue iterating.

When changes are necessary, _request_ them, do not _demand_ them, and **do not
assume that the submitter already knows how to add a test or run a benchmark**.

Specific performance optimization techniques, coding styles and conventions
change over time. The first impression you give to a new contributor never
does.

Nits (requests for small changes that are not essential) are fine, but try to
avoid stalling the Pull Request. Most nits can typically be fixed by the team
Collaborator landing the Pull Request but they can also be an opportunity for
the contributor to learn a bit more about the project.

It is always good to clearly indicate nits when you comment: e.g. `Nit: change
foo() to bar(). But this is not blocking.`

If your comments were addressed but were not folded automatically after new
commits or if they proved to be mistaken, please, [hide
them](https://help.github.com/articles/managing-disruptive-comments/#hiding-a-comment)
with the appropriate reason to keep the conversation flow concise and relevant.

### Be aware of the person behind the code

Be aware that _how_ you communicate requests and reviews in your feedback can
have a significant impact on the success of the Pull Request. Yes, we may land
a particular change that makes the project better, but the individual might
just not want to have anything to do with the project ever again. The goal is
not just having good code.

### Abandoned or Stalled Pull Requests

If a Pull Request appears to be abandoned or stalled, it is polite to first
check with the contributor to see if they intend to continue the work before
checking if they would mind if you took it over (especially if it just has nits
left). When doing so, it is courteous to give the original contributor credit
for the work they started (either by preserving their name and email address in
the commit log, or by using an `Author:` meta-data tag in the commit.

## Minimum Supported Rust Version (MSRV)

- All ≥1.0.0 releases will support at least a 6-month old Rust compiler
release.
- The MSRV will only be increased on 1.x releases.

## Versioning Policy

With ≥1.0.0:

- Patch (1.\_.x) releases _should only_ contain bug fixes or documentation
changes. Besides this, these releases should not substantially change runtime
behavior.
- Minor (1.x) releases may contain new functionality, MSRV increases (see
above), minor dependency updates, deprecations, and larger internal
implementation changes.

This is as defined by [Semantic Versioning 2.0](https://semver.org/) and
automated with [`release-plz`](https://release-plz.ieni.dev).

## Rationale

This project adheres to [Conventional
Commits](https://www.conventionalcommits.org/en/v1.0.0/), [Semantic
Versioning](https://semver.org/spec/v2.0.0.html), and [Keep a
Changelog](https://keepachangelog.com/en/1.0.0/). It also strives, in [a
constantly evolving
process](https://www.schneier.com/essays/archives/2000/04/the_process_of_secur.html),
to apply [AppSec](https://www.ibm.com/topics/application-security) [during the
entire development life cycle](https://www.youtube.com/watch?v=hDvz8KivY_U), as
best as it can.

Conventional Commits, Semantic Versioning, and Keep a Changelog, can profoundly
intertwine and, when so, they must seamlessly work in concert to fully
appreciate their benefits. With Conventional Commits as the Gordian Knot of
automation arising from combining these best practices, guaranteeing that all
commits adhere to it becomes paramount.

### Guaranteeing contract

Manually enforcing best practices and guaranteeing contracts in a code base and
onto contributors befalls under the inadequate and cumbersome category. Thus,
this project relies on both [local tooling](#installing-requirements) together
with CI pipelines, to make sure any changes pushed to this repository don't
break contract.

### Fully automated releases

Appropriately implementing the afore mentioned best practices opens the door to
[Fully Automated Releases](https://blog.orhun.dev/automated-rust-releases/).
For this purpose, This project relies on
[release-plz](https://release-plz.ieni.dev). To [automate
changelog](https://release-plz.ieni.dev/docs/changelog-format). To handle
dependency updates. To handle [version
management](https://release-plz.ieni.dev/docs/semver-check) and
[crates.io](https://crates.io) releases. Upon successful release pipelines,
`release-plz` invokes [cargo-dist](https://opensource.axo.dev/cargo-dist/) to
create GitHub releases and packaging for various platforms, along with binary
artifacts and installers.
