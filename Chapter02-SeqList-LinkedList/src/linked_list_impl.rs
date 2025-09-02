struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

pub struct SiLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SiLinkedList<T> {
    pub fn new() -> Self {
        SiLinkedList { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let node = *boxed_node;
            self.head = new_node.next;
            node.elem
        })
    }

    pub fn peak(&self) -> Option<T> {
        self.head.as_ref().map(|node| *node.elem)
    }

    pub fn peak_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
