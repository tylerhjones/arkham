use objectkey::ObjectKey;
use std::collections::HashMap;
use futures::Future;
use std::iter::Map;

trait Provider {
    fn get_type(&self) -> String;
}

struct MyService {
    database: HashMap<String, String>
}

impl MyService {
    fn new() -> MyService {
        MyService {
            database: HashMap::new()
        }
    }
}

impl Provider for MyService {
    fn get_type(&self) -> String {
        "MyService".to_string()
    }
}

#[cfg(test)]
mod test {
    use objectkey::ObjectKey;
    use std::collections::HashMap;
    use provider::{MyService,Provider};
    use std::string::String;

    #[test]
    fn demonstrate_providing_my_service() {
        let service = MyService::new();

        let mut map: HashMap<String, Provider> = HashMap::new();

        map.insert(service.get_type(), Box::new(service));

        assert_eq!(map.keys().len(), 1);
    }
}
