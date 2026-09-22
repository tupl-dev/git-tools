use crate::todo_item::TodoItem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents tool configuration.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// Collection of [`TodoItem`] and their associated branch.
    items: HashMap<String, Vec<TodoItem>>,
}

impl Config {
    /// Adds a new [`TodoItem`] to a branch.
    #[inline]
    pub fn add(&mut self, branch: &str, item: TodoItem) {
        self.items.entry(branch.to_owned()).or_default().push(item);
    }

    /// Clears [`TodoItem`].
    pub fn clear(&mut self, branch: &str, complete: bool) {
        let Some(items) = self.items.get_mut(branch) else {
            return;
        };
        if complete {
            items.retain(|item| !item.completed);
        } else {
            items.clear();
        }

        if items.is_empty() {
            self.items.remove(branch);
        }
    }

    /// Deletes [`TodoItem`] from a branch.
    pub fn delete(&mut self, branch: &str, indexes: Vec<usize>) {
        let Some(items) = self.items.get_mut(branch) else {
            return;
        };
        let mut sorted = indexes;
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        sorted.dedup();

        for index in sorted {
            if index < items.len() {
                items.remove(index);
            }
        }

        if items.is_empty() {
            self.items.remove(branch);
        }
    }

    /// Gets all [`TodoItem`] in a branch.
    #[inline]
    pub fn get(&self, branch: &str) -> impl Iterator<Item = &TodoItem> {
        self.items.get(branch).into_iter().flat_map(|items| items.iter())
    }

    /// Toggles completion status of [`TodoItem`] in a branch.
    pub fn toggle(&mut self, branch: &str, indexes: Vec<usize>) {
        let Some(items) = self.items.get_mut(branch) else {
            return;
        };
        for index in indexes {
            if let Some(item) = items.get_mut(index) {
                item.toggle();
            }
        }
    }
}

#[allow(clippy::indexing_slicing, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut config = Config::default();
        let item = TodoItem::new("Test item");
        config.add("main", item.clone());
        assert_eq!(config.items.get("main").unwrap().len(), 1);
        assert_eq!(config.items.get("main").unwrap()[0], item);
    }

    #[test]
    fn test_clear() {
        let mut config = Config::default();
        config.add("main", TodoItem::new("main 1"));
        config.add("test", TodoItem::new("test 1"));
        config.clear("main", false);
        assert_eq!(config.items.len(), 1);
        assert!(config.items.contains_key("test"));

        let mut config = Config::default();
        config.add("main", TodoItem::new("main 1"));
        config.add("main", TodoItem::new("main 2"));
        config.add("test", TodoItem::new("test 1"));
        config.toggle("main", vec![0]);
        config.clear("main", true);
        assert_eq!(config.items.len(), 2);
        assert!(config.items.contains_key("main"));
        assert!(config.items.contains_key("test"));
    }

    #[test]
    fn test_delete() {
        let mut config = Config::default();
        config.add("main", TodoItem::new("test 1"));
        config.add("main", TodoItem::new("test 2"));
        config.delete("main", vec![0]);
        assert_eq!(config.items.get("main").unwrap()[0].content, "test 2");
        config.delete("main", vec![0]);
        assert!(!config.items.contains_key("main"));

        let mut config = Config::default();
        config.add("main", TodoItem::new("test 1"));
        config.add("main", TodoItem::new("test 2"));
        config.delete("main", vec![1, 0]);
        assert!(!config.items.contains_key("main"));
    }

    #[test]
    fn test_get() {
        let mut config = Config::default();
        config.add("main", TodoItem::new("test 1"));
        let items = config.get("main").collect::<Vec<_>>();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].content, "test 1");
    }

    #[test]
    fn test_toggle() {
        let mut config = Config::default();
        config.add("main", TodoItem::new("test 1"));
        let items = config.get("main").collect::<Vec<_>>();
        assert!(!items[0].completed);

        config.toggle("main", vec![0]);
        let items = config.get("main").collect::<Vec<_>>();
        assert!(items[0].completed);
    }
}
