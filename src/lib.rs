//! Variable substitution.

#![allow(clippy::implicit_hasher)]

use failure::{ensure, Fallible};
use std::collections::HashMap;

/// Substitute variables in a template string.
///
/// Given an input string `template`, replace tokens of the form `${foo}` with
/// values provided in `variables`.
pub fn substitute<T>(template: T, variables: &HashMap<String, String>) -> Fallible<String>
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
        output = output.replace(&from, &v)
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
pub fn validate_vars(variables: &HashMap<String, String>) -> Fallible<()> {
    for (k, v) in variables {
        validate(k, "key")?;
        validate(v, "value")?;
    }
    Ok(())
}

/// Check whether `value` contains invalid characters.
fn validate<S>(value: S, kind: &str) -> Fallible<()>
where
    S: AsRef<str>,
{
    let forbidden = &["$", "{", "}"];
    for c in forbidden {
        ensure!(
            !value.as_ref().contains(c),
            format!(
                "variable {} '{}' contains forbidden character '{}'",
                kind,
                value.as_ref(),
                c
            )
        );
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
}
