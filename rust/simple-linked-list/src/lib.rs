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

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
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
        if self.is_empty() {
            self.head = Some(Box::new(Node::new(_element)));
        } else {
            let mut curr = self.head.as_mut().unwrap();
            while curr.next.is_some() {
                curr = curr.next.as_mut().unwrap();
            }

            curr.next = Some(Box::new(Node::new(_element)));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        } else {
            let mut curr = self.head.as_mut().unwrap();
            let mut nxt_ptr = curr.next.as_mut().unwrap();
            loop {
                if nxt_ptr.next.is_none() {
                    break;
                }
                curr = nxt_ptr;
                nxt_ptr = curr.next.as_mut().unwrap();
            }
            curr.next = None;

            return Some(nxt_ptr.data);
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
