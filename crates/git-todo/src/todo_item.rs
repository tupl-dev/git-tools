use serde::{Deserialize, Serialize};

/// Represents a TODO item.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoItem {
    /// Item content.
    pub content: String,

    /// Whether the item is completed.
    pub completed: bool,
}

impl TodoItem {
    /// Creates a new [`TodoItem`].
    #[inline]
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            completed: false,
        }
    }

    /// Toggles the completion status of the item.
    #[inline]
    pub const fn toggle(&mut self) {
        self.completed = !self.completed;
    }

    /// Converts the [`TodoItem`] to a string representation.
    pub fn to_string(&self, index: usize) -> String {
        let complete = if self.completed { "x" } else { " " };
        format!("[{}] {}: {}", complete, index, self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let item = TodoItem::new("Test item".to_owned());
        assert_eq!(item.content, "Test item");
        assert!(!item.completed);
    }

    #[test]
    fn test_toggle() {
        let mut item = TodoItem::new("Test item".to_owned());
        item.toggle();
        assert!(item.completed);

        item.toggle();
        assert!(!item.completed);
    }

    #[test]
    fn test_to_string() {
        let item = TodoItem::new("Test item".to_owned());
        assert!(item.to_string(1).contains('1'));
        assert!(item.to_string(99).contains("99"));
    }
}
