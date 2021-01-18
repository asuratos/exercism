use std::iter::FromIterator;

pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            data: elem,
            next: None,
        }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList::<T> { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut count: usize = 0;
        let mut curr = self.head.as_ref();

        while curr.is_some() {
            curr = curr.unwrap().next.as_ref();
            count += 1;
        }

        count
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Node {
            data: _element,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let data = node.data;
            self.head = node.next;
            data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut out = SimpleLinkedList::new();
        while let Some(data) = self.pop() {
            out.push(data);
        }
        out
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut out = SimpleLinkedList::new();
        _iter.into_iter().for_each(|c| out.push(c));
        out
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut output = Vec::new();
        while self.head.is_some() {
            output.push(self.pop().unwrap());
        }
        output.reverse();
        output
    }
}
