use std::collections::HashMap;

pub trait Memory<T: Clone> {
    fn add(&mut self, id: String, val: T);
    fn get(&self, id: &str) -> Option<&T>;
    fn get_mut(&mut self, id: &str) -> Option<&mut T>;

    fn update(&mut self, id: String, val: T) -> Option<T> {
        let old_val = self.get_mut(&id)?;
        let saved_val = old_val.clone();

        *old_val = val;

        Some(saved_val)
    }

    fn push_scope(&mut self);
    fn pop_scope(&mut self);
}

#[derive(Debug)]
pub struct HashMemory<T> {
    scopes: Vec<HashMap<String, T>>,
}

impl<T> Default for HashMemory<T> {
    /// Creates a global scope by default
    fn default() -> Self {
        Self {
            scopes: vec![HashMap::default()],
        }
    }
}

impl<T: Clone> Memory<T> for HashMemory<T> {
    fn add(&mut self, id: String, val: T) {
        self.scopes
            .last_mut()
            .unwrap(/* There sohuld always be at least a global scope */)
            .insert(id, val);
    }

    /// Resolves variables using dynamic scoping
    fn get(&self, id: &str) -> Option<&T> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(id) {
                return Some(val);
            }
        }

        None
    }

    fn get_mut(&mut self, id: &str) -> Option<&mut T> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(val) = scope.get_mut(id) {
                return Some(val);
            }
        }

        None
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::default())
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::memory::{HashMemory, Memory};

    #[test]
    fn test_global_scope() {
        let memory = HashMemory::<u32>::default();
        assert_eq!(memory.scopes.len(), 1);
        assert_eq!(memory.scopes.first(), Some(&HashMap::from([])))
    }

    #[test]
    fn test_push_scope() {
        let mut memory = HashMemory::<u32>::default();
        memory.add("x".to_string(), 1);

        memory.push_scope();

        memory.add("y".to_string(), 2);
        assert_eq!(memory.scopes.len(), 2);
        assert_eq!(
            memory.scopes.first(),
            Some(&HashMap::from([("x".to_string(), 1)]))
        );
        assert_eq!(
            memory.scopes.last(),
            Some(&HashMap::from([("y".to_string(), 2)]))
        )
    }

    #[test]
    fn test_dynamic_scoping() {
        let mut memory = HashMemory::<u32>::default();
        memory.add("x".to_string(), 1);

        memory.push_scope();
        assert_eq!(memory.get("x"), Some(&1));
    }

    #[test]
    fn test_dynamic_scoping_shadowing() {
        let mut memory = HashMemory::<u32>::default();
        memory.add("x".to_string(), 1);

        memory.push_scope();

        memory.add("x".to_string(), 10);
        assert_eq!(memory.get("x"), Some(&10));
    }

    #[test]
    fn test_dynamic_scoping_empty_global() {
        let mut memory = HashMemory::<u32>::default();

        memory.push_scope();

        memory.add("x".to_string(), 10);
        assert_eq!(memory.get("x"), Some(&10));
    }
}
