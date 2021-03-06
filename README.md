# to

> A CLI utility for bookmarking directories with tab completion.

[![Travis branch][travis-badge]][travis-url]
[![Coverage Status][coveralls-badge]][coveralls-url]

[coveralls-badge]: https://coveralls.io/repos/github/jxson/to-directory/badge.svg
[coveralls-url]: https://coveralls.io/github/jxson/to-directory
[travis-badge]: https://img.shields.io/travis/jxson/to-directory/master.svg?style=flat-square
[travis-url]: https://travis-ci.org/jxson/to-directory


This is a [Rust] implementation of the excellent shell script [autochthe/to]. I ([jxson]) created this utility as a way to learn the [Rust programming language][Rust] and hopefully provide a stable cross-platform experience.

**NOTE:** Contributors with Windows experience wanted, open an [issue] and I will add you to the project.

# Stability: Experimental

Expect the unexpected. Please provide [feedback][issue] on the CLI, code APIs and your use-case.

> This code is under active, exploratory development. Any documentation below should be considered aspirational and is not yet reflective how how to work with this tool.

# Installation

Add the prebuilt binary to your path.

    curl prebuilt.tar > $PREFIX/bin

Add the initialization command to your profile.

    if which to-directory > /dev/null; then eval "$(to-directory --init)"; fi

Source your profile to pick up the new configuration. For example, if the line above was added to your ~/.bashrc you can:

    source ~/.bashrc

Now every time you log in, the abbreviated `to` command will be available and allow you to quickly cd into your saved directory bookmarks. Run the help command to learn how to get started:

    to --help

# Development

## Debugging

This project uses `error-chain`, a crate that enables the behavior of being able to see the cascade causes for a given error when it occurs (among a bunch of other handy features). By default the `to` command and it's companion Rust binary `to-directory` attempt to adhere to the rule of silence, which is good for users but not ideal during development and debugging.

To enable verbose errors use the `#` flag:

    to --#

To control logging levels and structured logging output use the `#` flag:

    to --log-level=debug --log-output=json

Note these are different from the `--verbose` flag in that they allow finer grain control and visibility over the user friendly, verbose output.

Backtraces

    RUST_BACKTRACE=1 cargo run

[autochthe/to]: https://github.com/autochthe/to
[jxson]: https://twitter.com/jxson
[Rust]: https://www.rust-lang.org
[issue]: https://github.com/jxson/to-directory/issues/new
