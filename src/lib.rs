#[derive(Debug, PartialEq)]
pub struct BSTNode<T> {
    pub value: T,
    pub left: Option<Box<BSTNode<T>>>,
    pub right: Option<Box<BSTNode<T>>>
}

impl<T> BSTNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let value = i8::MAX;
        let node = BSTNode::new(value);
        assert_eq!(node.value, value);
        assert_eq!(node.left, None);
        assert_eq!(node.right, None);
    }
}
