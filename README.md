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

Variable names can contain dots for namespacing (e.g. `${project.DATABASE_URL}`),
allowing you to organize variables with prefixes or hierarchical naming schemes.

[`envsubst`]: https://www.gnu.org/software/gettext/manual/html_node/envsubst-Invocation.html

## Example

```rust
use std::collections::HashMap;

let template = "Connect to ${project.DATABASE_URL}";
let mut context = HashMap::new();
context.insert("project.DATABASE_URL".to_string(), "postgres://localhost:5432/mydb".to_string());

let result = envsubst::substitute(template, &context).unwrap();
assert_eq!(result, "Connect to postgres://localhost:5432/mydb");
```

## License

Licensed under either of

 * MIT license - <http://opensource.org/licenses/MIT>
 * Apache License, Version 2.0 - <http://www.apache.org/licenses/LICENSE-2.0>

at your option.
