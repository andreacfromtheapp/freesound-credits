# Contributing

Thank you for investing your time in contributing to this project!
**Contributions of all sizes welcome and valued here.** :pray:

In this guide you will get an overview of the contribution workflow from opening
an issue, creating a PR, reviewing, and merging the PR.

## Rationale

For an overview of the project's MSRV, best practices, tooling, automation, and
more, make sure to read the [Rationale](docs/RATIONALE.md).

## Conduct

This project adheres to the [Code of Conduct](docs/CODE_OF_CONDUCT.md). This
describes the _minimal_ behavior expected from all contributors.

## New contributor guide

Some resources to help you get started with open source contributions:

- [Finding ways to contribute to open source on GitHub](https://docs.github.com/en/get-started/exploring-projects-on-github/finding-ways-to-contribute-to-open-source-on-github)
- [Set up Git](https://docs.github.com/en/get-started/getting-started-with-git/set-up-git)
- [Collaborating with pull requests](https://docs.github.com/en/github/collaborating-with-pull-requests)

## Contributing in issues

For any issue, the three fundamental ways an individual can contribute:

1. By opening the issue for discussion: For instance, if you believe that you
    have discovered a bug.

2. By helping to triage the issue: Providing supporting details (a test case
    that demonstrates a bug), providing suggestions on how to deal with the
    issue, or ensuring that the issue has appropriate tags.

3. By helping to resolve the issue: Typically either in the form of
    demonstrating that the issue reported ain't a problem after all, or more
    often, by opening a Pull Request that changes some bit of something in a
    concrete and reviewable manner.

**Anybody can partake in any stage of contribution**. This project encourages
you to partake in the discussion around bugs and partake in reviewing pull
requests.

### Asking for general help

If you have reviewed existing documentation and still have questions or
problems, you can
[open a discussion](https://github.com/andreacfromtheapp/freesound-credits/discussions/new)
asking for help.

In exchange for receiving help, this project asks that you contribute back a
documentation PR that helps others avoid the problems that you encountered.

### Submitting a bug report

If you spot a problem,
[search if an issue already exists](https://docs.github.com/en/github/searching-for-information-on-github/searching-on-github/searching-issues-and-pull-requests#search-by-the-title-body-or-comments).
If a related issue doesn't exist, you can open a new issue using a relevant
[issue form](https://github.com/andreacfromtheapp/freesound-credits/issues/new/choose).
Please fill out this form, following the template to the best of your ability.

The two most important pieces of information: a description of the behavior you
see and a test case to recreate the problem. See
[How to create a Minimal, Complete, and Verifiable example](https://stackoverflow.com/help/mcve).

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
you feel that way. With supporting context. Willing to concede misconstruing. By
doing so, often leads the correct outcome much faster.

### Resolving a bug report

In the majority of cases, resolving issues involve opening a Pull Request. The
process for opening and reviewing a Pull Request resembles that of opening and
triage issues. Although it carries with it a necessary review and approval
workflow that ensures that the proposed changes meet the minimal quality and
functional guidelines of the project.

## Making changes

To learn more about local tooling requirements and why this project relies on
them, read more about it [here](docs/RATIONALE.md#local-tooling).

### Installing requirements

> [!IMPORTANT] Contributors necessarily need installing and perusing tooling
> locally guaranteeing contracts before committing any changes.

- If necessary, [set up Rust](https://www.rust-lang.org/tools/install) with
  `rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install
  [`cargo auditable`](https://github.com/rust-secure-code/cargo-auditable):
  `cargo install cargo-auditable --locked`
  - Set up a
    [shell alias for `cargo auditable`](https://github.com/rust-secure-code/cargo-auditable?tab=readme-ov-file#can-i-make-cargo-always-build-with-cargo-auditable)
- Install [`cargo deny`](https://embarkstudios.github.io/cargo-deny/):
  `cargo install cargo-deny --locked`
- Install [`trufflehog`](https://trufflesecurity.com/trufflehog):
  `brew install trufflehog`
  - Otherwise, download a binary from
    [`trufflehog` release page](https://github.com/trufflesecurity/trufflehog/releases).
- Install [`pre-commit`](https://pre-commit.com): `pip install pre-commit`

### Making changes locally

- 1 Fork the repository.

- Using GitHub Desktop:

  - [Getting started with GitHub Desktop](https://docs.github.com/en/desktop/installing-and-configuring-github-desktop/getting-started-with-github-desktop)
    will guide you through setting up Desktop.
  - Once you set up Desktop, you can use it to
    [fork the repository](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/cloning-and-forking-repositories-from-github-desktop)!

- Using the command line:

  - [Fork the repository](https://docs.github.com/en/github/getting-started-with-github/fork-a-repo#fork-an-example-repository)
    so that you can make your changes without affecting the original project
    until you're ready to merge them.

- 2 Change into the cloned repository: `cd github`.

- 3 Install [Git Hooks](https://githooks.com) with
  `pre-commit install --install-hooks`

- 4 Create a working branch and start with your changes!

## Pull requests

Pull Requests: the concrete way to apply changes to the code, documentation, and
dependencies in the repository.

Even tiny pull requests (e.g: one character pull request fixing a typo in API
documentation) highly appreciated. Before making a large change, first open an
issue describing the change to seek feedback and guidance. This will increase
the likelihood of the PR getting merged.

When you completed the changes, create a pull request, also known as a PR.

- Don't forget to
  [link PR to issue](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue),
  should you solve one.
- Enable the checkbox to
  [allow maintainer edits](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/allowing-changes-to-a-pull-request-branch-created-from-a-fork)
  . Once you submit your PR, a team member will review your proposal. The team
  may ask questions or request farther information.
- The team may ask for farther changes before merging a PR, either using
  [suggested changes](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/incorporating-feedback-in-your-pull-request)
  or pull request comments. You can apply suggested changes directly through the
  UI. You can make any other changes in your fork, then commit them to your
  branch.
- As you update your PR and apply changes, mark each conversation as
  [resolved](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/commenting-on-a-pull-request#resolving-conversations).
- If you run into any merge issues, checkout this
  [git tutorial](https://github.com/skills/resolve-merge-conflicts) to help you
  resolve merge conflicts and other issues.

### Performing pre-commit checks on the codebase

Before committing any changes, make sure to run:

```shell
pre-commit run --all-files
```

**Should it present any warning or issue: please fix, rinse, and repeat.**

### Testing

If the proposed change alters code, either adding new functionality to the
project or fixing existing, broken functionality: include tests to guarantee
that the project has no regressions in the future.

You can achieve this with three ways to write tests: unit tests, integration
tests, and documentation tests.

1. Unit tests

    [Unit tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests)
    go in the same source code file as the data structures and procedures
    they're testing. To run all tests or a subset of tests, refer
    [to The Book](https://doc.rust-lang.org/book/ch11-02-running-tests.html).

    > [!IMPORTANT] Write tests that fail for code that one shouldn't implement,
    > to avoid future regressions and unwanted behaviors.

2. Integration tests

    Typically,
    [integration tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests)
    go in the same crate as the code they test. That said, this project sole
    possible integration test is to run `freesound-credits` against a DAW sample
    folder. Make sure to check this always works when making meaningful changes.

3. Documentation tests

    Ideally, every API has at least one
    [documentation test](https://doc.rust-lang.org/rustdoc/documentation-tests.html)
    that demonstrates how to use the API.

    Documentation tests run with `cargo test --doc`. This ensures that the
    example congruously carries out and provides test coverage.

    When writing documentation tests strike a balance between brevity for a
    reader to understand and code actually testing the API.

    Same as with integration tests, when writing a documentation test, full
    access to the crate is available. Notably useful for getting access to the
    runtime to run the example.

    The documentation tests visibility spans from both the crate specific
    documentation **and** the project facade documentation via the re-export.
    Write examples from the point of view of a user using the crate. As such,
    the example should use the API via the facade and not by directly
    referencing the crate.

### Committing updates

Best practice recommend to keep your changes as logically grouped as possible
within individual commits. The project imposes no limit to the number of commits
any single Pull Request may have. Most contributors find it easier to review
changes split across several commits.

That said, if you have several "checkpoints" commits that don't represent a
single logical change, please squash those together.

> [!NOTE] Several commits often get squashed during merges nonetheless See: the
> notes about [commit squashing](#commit-squashing).

1. Commit message guidelines

    Make sure to use
    [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#why-use-conventional-commits).

    A good commit message should describe what changed and why.

    - 1 The first line should:

    - contain a short description of the change (preferably 50 characters or
      less, and no more than 72 characters)

      - composed entirely of lowercase except for proper nouns, acronyms, and
        the words that refer to code, like function/variable names
      - start with an imperative verb
      - not have a full stop at the end
      - prefixed with the name of the changed module; the same as the M-\* label
        on the PR

      Examples:

      - time: introduce `Timeout` and deprecate `Deadline`
      - ci: fix the FreeBSD ci configuration

    - 2 Keep the second line blank.

    - 3 Wrap all other lines at 72 columns (except for long URLs).

    - 4 If your patch fixes an open issue, you can add a reference to it at the
      end of the log. Use the `Fixes: #` prefix and the issue number. For other
      references use `Refs: #`. `Refs` may include issues, separated by a comma.

      Examples:

      - `Fixes: #1337`
      - `Refs: #1234`

    Example of a complete commit message:

    ```txt
    module: explain the commit in one line

    Body of commit message is a few lines of text, explaining things in more
    detail, possibly giving some background about the issue being fixed, etc.

    The body of the commit message can be several paragraphs, and please do
    proper word-wrap and keep columns shorter than about 72 characters or so.
    That way, =git log= will show things nicely even when it is indented.

    Fixes: #1337 Refs: #453, #154
    ```

### Opening the pull request

From within GitHub, opening a new Pull Request will present you with a
[template](.github/PULL_REQUEST_TEMPLATE/pull_request_template.md). Please try
to do your best at filling out the details, but feel free to skip parts if ain't
sure what to put.

### Discuss and update

You will probably get feedback or requests for changes to your Pull Request.
Commonly, a big part of the submission process. Don't discourage! Some
contributors may sign off on the Pull Request right away, others may have more
detailed comments or feedback. This is a necessary part of the process in order
to appraise whether the changes subsist correct and necessary.

**Any community member can review a PR and you might get conflicting feedback**.
Keep an eye out for comments from code owners to provide guidance on conflicting
feedback.

**Once the PR is open, don't rebase the commits**. See
[Commit Squashing](#commit-squashing) for more details.

### Commit squashing

**Don't squash commits that you add to your Pull Request during the review
process**.

When the commits in your Pull Request land, they may be squashed into one commit
per logical change. Metadata will be added to the commit message (including
links to the Pull Request, links to relevant issues, and the names of the
reviewers). The commit history of your Pull Request. Nonetheless, will stay
intact on the Pull Request page.

## Reviewing pull requests

**Any community member welcome to review any pull request**.

All contributors who choose to review and give feedback on Pull Requests have a
responsibility to both the project and the individual making the contribution.
Reviews and feedback must be helpful, insightful, and geared towards improving
the contribution as opposed to blocking it. Should you have reasons why you feel
the PR shouldn't land, explain what those are. Don't expect to be able to block
a Pull Request from advancing just because you say "No" without giving an
explanation. Stay open to having your mind changed. Stay open to working with
the contributor to make the Pull Request better.

Dismissive or disrespectful reviews of the contributor or any other reviewers
disincline with the [Code of Conduct](docs/CODE_OF_CONDUCT.md).

When reviewing a Pull Request, the primary goals consist of for the codebase to
improve and for the person submitting the request to succeed. _Even if a Pull
Request doesn't land, the submitters should come away from the experience
feeling like their effort wasn't wasted or unappreciated_. Treat every Pull
Request from a new contributor as an opportunity to grow the community.

### Review a bit at a time

**Avoid overwhelming new contributors**.

As tempting as micro-optimizing and make everything about relative performance,
perfect grammar, or exact style matches, may be: don't succumb to that
temptation.

Focus first on the most significant aspects of the change:

1. Does this change make sense for the project?
2. Does this change make the project better, even if only incrementally?
3. Are there clear bugs or larger scale issues that need attending to?
4. Is the commit message readable and correct? If it has a breaking change is
    it clear enough?

Note that **incremental** improvement suffices to land a PR. This means that the
PR doesn't need to meet perfection. _Better than the status quo_ qualifies. One
can open follow up Pull Requests to continue iterating.

When changes prove necessary, _request_ them, don't _demand_ them, and **don't
assume that the submitter already knows how to add a test or run a benchmark**.

Specific performance optimization techniques, coding styles and conventions
change over time. The first impression you give to a new contributor never does.

Nits (non-essential requests for small changes) accepted, but try to avoid
stalling the Pull Request. While the Team Collaborator landing the Pull Request
can typically fix most nits, also consider them an opportunity for the
contributor to learn a bit more about the project.

> [!NOTE] Always denote nits when you comment: e.g. =nit: change foo() to bar().
> But this is not blocking.=

If addressed comments ain't folded automatically after new commits or if they
proved mistaken, please,
[hide them](https://help.github.com/articles/managing-disruptive-comments/#hiding-a-comment)
with the appropriate reason to keep the conversation flow concise and relevant.

### The person behind the code

Be aware that _how_ you communicate requests and reviews in your feedback can
have a significant impact on the success of the Pull Request. Yes, landing a
particular change may improve the project, but the individual might just not
want to have anything to do with the project ever again. Having good code ain't
the sole goal.

### Abandoned or stalled pull requests

Should Pull Request appear abandoned or stalled, courteously first check with
the contributor to see if they intend to continue the work. Before checking if
they would mind if you took it over (achingly if it just has nits left). When
doing so, courteously give the original contributor credit for the work they
started (either by preserving their name and email address in the commit log, or
by using an `Author:` meta-data tag in the commit.
