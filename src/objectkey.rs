use std::cmp::{ Eq, PartialEq };

#[derive(Eq, PartialEq, Debug)]
struct ObjectKey {
    object_type: String,
    named: Option<String>,
}

impl ObjectKey {
    fn new(object_type: &str) -> ObjectKey {
        ObjectKey {
            object_type: object_type.to_string(),
            named: None,
        }
    }

    fn named(&mut self, name: &str) {
        self.named = Some(name.to_string())
    }
}

#[cfg(test)]
mod test {
    use objectkey::ObjectKey;
    use std::option::Option::None;

    #[test]
    fn default_named_is_none() {
        let key = ObjectKey::new("foo");
        assert_eq!(key.named, None);
    }

    #[test]
    fn object_equality_default_named() {
        let typed = "typed";
        assert_eq!(
            ObjectKey::new(typed),
            ObjectKey::new(typed));

        assert_ne!(
            ObjectKey::new(typed),
            ObjectKey::new(""));
    }

    #[test]
    fn object_equality_named() {
        let typed = "typed";
        let name = "name";
        assert_eq!(
            ObjectKey::new(typed).named(name),
            ObjectKey::new(typed).named(name));

        assert_ne!(
            ObjectKey::new(typed).named(name),
            ObjectKey::new(typed).named("bar"));
    }
}
