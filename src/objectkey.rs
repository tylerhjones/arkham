use std::cmp::{ Eq, PartialEq };

#[derive(Eq, PartialEq, Debug)]
pub struct ObjectKey {
    object_type: String,
    named: String,
}

impl ObjectKey {
    fn new(object_type: &str) -> ObjectKey {
        ObjectKey {
            object_type: object_type.to_string(),
            named: "".to_string(),
        }
    }

    fn named(&mut self, name: &str) {
        self.named = name.to_string()
    }
}

#[cfg(test)]
mod test {
    use objectkey::ObjectKey;

    #[test]
    fn default_named_is_empty() {
        let key = ObjectKey::new("foo");
        assert_eq!(key.named, "");
    }

    #[test]
    fn object_equality_default_named_eq() {
        let typed = "typed";
        assert_eq!(
            ObjectKey::new(typed),
            ObjectKey::new(typed));
    }

    #[test]
    fn object_equality_default_named_ne() {
        assert_ne!(
            ObjectKey::new("typed"),
            ObjectKey::new(""));
    }

    #[test]
    fn object_equality_named_eq() {
        let typed = "typed";
        let name = "name";

        assert_eq!(
            ObjectKey::new(typed).named(name),
            ObjectKey::new(typed).named(name));
    }

    #[test]
    fn object_equality_named_ne() {
        let typed = "typed";
        debug_assert_ne!(
            ObjectKey::new(typed).named("name"),
            ObjectKey::new(typed).named("bar"));
    }
}
