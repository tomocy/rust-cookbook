use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Dictionary(HashMap<String, Vec<String>>);

impl Dictionary {
    fn new() -> Self {
        Self(HashMap::new())
    }
}
