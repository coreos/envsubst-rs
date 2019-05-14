//! Variable substitution.

#![allow(clippy::implicit_hasher)]

use failure::Fallible;
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
        let from = format!("${{{}}}", k);
        output = output.replace(&from, &v)
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::substitute;
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
}
