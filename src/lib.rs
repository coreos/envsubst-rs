//! Variables substitution in string templates.
//!
//! This library provide helper functions for string manipulation,
//! taking values from a context **env**ironment map and **subst**ituting
//! all matching placeholders.
//!
//! Its name and logic is similar to the [`envsubst`] GNU utility, but
//! this only supports braces-delimited variables (i.e. `${foo}`) and
//! takes replacement values from an explicit map of variables.
//!
//! Variable names can contain dots for namespacing (e.g. `${project.DATABASE_URL}`),
//! allowing you to organize variables with prefixes or hierarchical naming schemes.
//!
//! [`envsubst`]: https://www.gnu.org/software/gettext/manual/html_node/envsubst-Invocation.html
//!
//! ## Example
//!
//! ```rust
//! let base_url = "${protocol}://${hostname}/${endpoint}";
//! assert!(envsubst::is_templated(base_url));
//!
//! let mut context = std::collections::HashMap::new();
//! context.insert("protocol".to_string(), "https".to_string());
//! context.insert("hostname".to_string(), "example.com".to_string());
//! context.insert("endpoint".to_string(), "login".to_string());
//! assert!(envsubst::validate_vars(&context).is_ok());
//!
//! let final_url = envsubst::substitute(base_url, &context).unwrap();
//! assert!(!envsubst::is_templated(&final_url));
//! assert_eq!(final_url, "https://example.com/login");
//! ```
//!
//! ## Namespaced Variables
//!
//! ```rust
//! let template = "Database: ${project.DATABASE_URL}";
//! let mut context = std::collections::HashMap::new();
//! context.insert("project.DATABASE_URL".to_string(), "postgres://localhost:5432/mydb".to_string());
//!
//! let result = envsubst::substitute(template, &context).unwrap();
//! assert_eq!(result, "Database: postgres://localhost:5432/mydb");
//! ```

#![allow(clippy::implicit_hasher)]

use std::collections::HashMap;

/// Library errors.
#[derive(thiserror::Error, Debug)]
#[error("envsubst error: {0}")]
pub struct Error(String);

/// Substitute variables in a template string.
///
/// Given an input string `template`, replace tokens of the form `${foo}` with
/// values provided in `variables`.
pub fn substitute<T>(template: T, variables: &HashMap<String, String>) -> Result<String, Error>
where
    T: Into<String>,
{
    let mut output = template.into();
    if variables.is_empty() {
        return Ok(output);
    }

    for (k, v) in variables {
        validate(k, "key")?;
        validate(v, "value")?;

        let from = format!("${{{}}}", k);
        output = output.replace(&from, v)
    }

    Ok(output)
}

/// Check whether input string contains templated variables.
pub fn is_templated<S>(input: S) -> bool
where
    S: AsRef<str>,
{
    let start = input.as_ref().find("${");
    let end = input.as_ref().find('}');

    match (start, end) {
        (Some(s), Some(e)) => s < e,
        _ => false,
    }
}

/// Validate variables for substitution.
///
/// This check whether substitution variables are valid. In order to make
/// substitution deterministic, the following characters are not allowed
/// within variables names nor values: `$`, `{`, `}`.
pub fn validate_vars(variables: &HashMap<String, String>) -> Result<(), Error> {
    for (k, v) in variables {
        validate(k, "key")?;
        validate(v, "value")?;
    }
    Ok(())
}

/// Check whether `value` contains invalid characters.
fn validate<S>(value: S, kind: &str) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let forbidden = &["$", "{", "}"];
    for c in forbidden {
        if value.as_ref().contains(c) {
            let err_msg = format!(
                "variable {} '{}' contains forbidden character '{}'",
                kind,
                value.as_ref(),
                c
            );
            return Err(Error(err_msg));
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn basic_subst() {
        let template = "foo ${VAR} bar";
        let mut env = HashMap::new();
        env.insert("VAR".to_string(), "var".to_string());

        let out = substitute(template, &env).unwrap();
        let expected = "foo var bar";
        assert_eq!(out, expected);
    }

    #[test]
    fn template_check() {
        let plain = "foo";
        assert!(!is_templated(plain));

        let template = "foo ${VAR} bar";
        assert!(is_templated(template));

        let starting = "foo${";
        assert!(!is_templated(starting));

        let ending = "foo}";
        assert!(!is_templated(ending));
    }

    #[test]
    fn basic_empty_vars() {
        let template = "foo ${VAR} bar";
        let env = HashMap::new();

        let out = substitute(template, &env).unwrap();
        assert_eq!(out, template);
    }

    #[test]
    fn dollar_bracket() {
        let template = "foo ${ bar";
        let mut env = HashMap::new();
        env.insert("VAR".to_string(), "var".to_string());

        let out = substitute(template, &env).unwrap();
        assert_eq!(out, template);
    }

    #[test]
    fn invalid_vars() {
        let template = "foo ${VAR} bar";
        let mut env = HashMap::new();
        env.insert("${VAR}".to_string(), "var".to_string());

        substitute(template, &env).unwrap_err();

        let mut env = HashMap::new();
        env.insert("VAR".to_string(), "${VAR}".to_string());
    }

    #[test]
    fn namespaced_vars() {
        let template = "Database URL: ${project.DATABASE_URL}";
        let mut env = HashMap::new();
        env.insert("project.DATABASE_URL".to_string(), "postgres://localhost:5432/mydb".to_string());

        let out = substitute(template, &env).unwrap();
        let expected = "Database URL: postgres://localhost:5432/mydb";
        assert_eq!(out, expected);
    }

    #[test]
    fn multiple_namespaced_vars() {
        let template = "${app.name} - ${app.version} - ${db.host}:${db.port}";
        let mut env = HashMap::new();
        env.insert("app.name".to_string(), "MyApp".to_string());
        env.insert("app.version".to_string(), "1.0.0".to_string());
        env.insert("db.host".to_string(), "localhost".to_string());
        env.insert("db.port".to_string(), "5432".to_string());

        let out = substitute(template, &env).unwrap();
        let expected = "MyApp - 1.0.0 - localhost:5432";
        assert_eq!(out, expected);
    }

    #[test]
    fn deeply_namespaced_vars() {
        let template = "${services.backend.api.url}";
        let mut env = HashMap::new();
        env.insert("services.backend.api.url".to_string(), "https://api.example.com".to_string());

        let out = substitute(template, &env).unwrap();
        let expected = "https://api.example.com";
        assert_eq!(out, expected);
    }
}
