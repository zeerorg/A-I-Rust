

pub struct NodeDepth<T> {
    pub node: T,
    pub depth: i32
}

impl<T> NodeDepth<T> {
    pub fn new(data: T, depth: i32) -> NodeDepth<T> {
        NodeDepth {
            node: data,
            depth: depth
        }
    }
}