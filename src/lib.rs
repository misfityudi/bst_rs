#[derive(Debug, PartialEq)]
pub struct BST<T> {
    pub value: T,
    pub left: Option<Box<BST<T>>>,
    pub right: Option<Box<BST<T>>>,
}

impl<T> BST<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let value = i8::MAX;
        let node = BST::new(value);
        assert_eq!(node.value, value);
        assert_eq!(node.left, None);
        assert_eq!(node.right, None);
    }

    #[test]
    fn insert() {}
}
