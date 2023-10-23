use std::collections::HashMap;
use lazy_regex::*;

/// The `LateSubstitution` trait allows substituting string parameters
/// of arbitrary name that is computed at runtime.
///
/// `LateSubstitution` is implemented for the `String` and `&str` types by default.
///
/// The substitution syntax accepts curly braces forms:
/// 
/// ```plain
/// {param_name}     # parameter to replace
/// {"escaped"}      # escaped sequence
/// ```
///
/// Syntax description:
///
/// - Whitespace is allowed around the parameter name or escaped form, such as
/// `{ "foo" }` versus `{"foo"}`.
/// - `{param_name}` expands to either an argument given in the map (whose key string is `param_name`) or
/// the string `None` if not present. The parameter name may contain any of the following characters:
/// ```plain
/// A-Z a-z 0-9 . - _ $
/// ```
/// - `{"escaped"}` expands to the string `escaped`. It is often
/// used for escaping the curly braces.
///
/// # Example
/// 
/// ```
/// use as3_parser::util::LateSubstitution;
/// use maplit::hashmap;
/// let user_string: String = "some user string: {id}".into();
/// assert_eq!("some user string: x", user_string.late_substitution(hashmap!{"id".into() => "x".into()}));
/// 
/// // if a string contains curly braces, they must be escaped.
/// let escaped: String = r#"{"{"}"#.into();
/// ```
///
pub trait LateSubstitution {
    fn late_substitution(&self, arguments: HashMap<String, String>) -> String;
}

impl LateSubstitution for &str {
    fn late_substitution(&self, arguments: HashMap<String, String>) -> String {
        regex_replace_all!(
            r#"(?x)
            \{\s*(
                ([a-zA-Z_0-9\-\.\$]+)   | # parameter
                ("([^\u{22}])*")          # escaped
            )\s*\}
            "#,
            self,
            |_, s: &str, _, _, _| {
                if s.starts_with('"') {
                    return s[1..s.len() - 1].to_owned().clone();
                }
                arguments.get(s).map_or("None".to_owned(), |v| v.clone())
            }
        ).into_owned()
    }
}

impl LateSubstitution for String {
    fn late_substitution(&self, arguments: HashMap<String, String>) -> String {
        self.as_str().late_substitution(arguments)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn substitution() {
        let user_string: String = "some user string: {id}".into();
        assert_eq!("some user string: x", user_string.late_substitution(hashmap!{"id".into() => "x".into()}));
        let user_string: String = r#"some user string: {"id"}"#.into();
        assert_eq!("some user string: id", user_string.late_substitution(hashmap!{"id".into() => "x".into()}));
        let user_string: String = r#"some user string: {  "id"  }"#.into();
        assert_eq!("some user string: id", user_string.late_substitution(hashmap!{"id".into() => "x".into()}));
        let user_string: String = "some user string: {id}".into();
        assert_eq!("some user string: None", user_string.late_substitution(hashmap!{}));
    }
}