# envsubst

[![crates.io](https://img.shields.io/crates/v/envsubst.svg)](https://crates.io/crates/envsubst)
[![Documentation](https://docs.rs/envsubst/badge.svg)](https://docs.rs/envsubst)

A simple Rust library for variables substitution.

This library provide helper functions for string manipulation,
taking values from a context **env**ironment map and **subst**ituting
all matching placeholders.

Its name and logic is similar to the [`envsubst`] GNU utility, but
this only supports braces-delimited variables (i.e. `${foo}`) and
takes replacement values from an explicit map of variables.

[`envsubst`]: https://www.gnu.org/software/gettext/manual/html_node/envsubst-Invocation.html

## License

Licensed under either of

 * MIT license - <http://opensource.org/licenses/MIT>
 * Apache License, Version 2.0 - <http://www.apache.org/licenses/LICENSE-2.0>

at your option.
