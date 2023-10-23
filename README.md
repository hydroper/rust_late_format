# Late substitution

This crate provides a simple way of substituting parameters in an arbitrary string, where parameter names are arbitrary.

This is an alternative to using complex template engines.

## Example

```rust
use late_substitution::LateSubstitution;
use maplit::hashmap;

let user_string: String = "some user string: {id}".into();
assert_eq!(
    "some user string: x",
    user_string.late_substitution(hashmap!{"id".into() => "x".into()})
);

let user_string: String = r#"some user string: {"id"}"#.into();
assert_eq!(
    "some user string: id",
    user_string.late_substitution(hashmap!{"id".into() => "x".into()})
);

let user_string: String = r#"some user string: {  "id"  }"#.into();
assert_eq!(
    "some user string: id",
    user_string.late_substitution(hashmap!{"id".into() => "x".into()})
);

let user_string: String = "some user string: {id}".into();
assert_eq!(
    "some user string: None",
    user_string.late_substitution(hashmap!{})
);
```