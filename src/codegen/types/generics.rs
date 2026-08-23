use std::collections::HashMap;

pub struct GenericResolver {
    names: HashMap<String, String>,
    next_index: usize,
}

impl GenericResolver {
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
            next_index: 0,
        }
    }

    pub fn resolve(&mut self, name: &str) -> String {
        if let Some(existing) = self.names.get(name) {
            return existing.clone();
        }

        let generated = format!("T{}", self.next_index);

        self.next_index += 1;
        self.names.insert(name.to_string(), generated.clone());

        generated
    }

    pub(crate) fn all(&self) -> Vec<String> {
        let mut values: Vec<_> = self.names.values().cloned().collect();

        values.sort_by_key(|name| {
            name.strip_prefix('T')
                .and_then(|n| n.parse::<usize>().ok())
                .unwrap_or(usize::MAX)
        });

        values
    }
}

impl Default for GenericResolver {
    fn default() -> Self {
        Self::new()
    }
}
