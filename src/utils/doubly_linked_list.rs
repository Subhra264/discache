use super::gen_arena::GenArena;

pub struct DoublyLinkedList<T> {
    arena: GenArena<Node<T>>,
    head: Option<usize>,
    tail: Option<usize>,
}

struct Node<T> {
    value: T,
    prev: Option<usize>,
    next: Option<usize>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            arena: GenArena::new(),
            head: None,
            tail: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DoublyLinkedList {
            arena: GenArena::with_capacity(capacity),
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, elem: T) -> Result<(), &'static str> {
        match self.arena.push(Node {
            value: elem,
            prev: self.tail,
            next: None,
        }) {
            Ok(index) => {
                if let Some(tail) = self.tail {
                    let prev_node = self.arena.at_mut(tail).unwrap();
                    prev_node.next = Some(index);
                } else {
                    self.head = Some(index);
                }
                self.tail = Some(index);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn shift(&mut self, elem: T) -> Result<(), &'static str> {
        match self.arena.push(Node {
            value: elem,
            prev: None,
            next: self.head,
        }) {
            Ok(index) => {
                if let Some(head) = self.head {
                    let next_node = self.arena.at_mut(head).unwrap();
                    next_node.prev = Some(index);
                } else {
                    self.tail = Some(index);
                }
                self.head = Some(index);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn top(&self) -> Option<&T> {
        if let Some(head) = self.head {
            match self.arena.at(head) {
                Some(node) => Some(&node.value),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn bottom(&self) -> Option<&T> {
        if let Some(tail) = self.tail {
            match self.arena.at(tail) {
                Some(node) => Some(&node.value),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn remove_bottom(&mut self) -> Option<T> {
        if let Some(tail) = self.tail {
            let bottom = self.arena.at(tail).unwrap();
            let prev = bottom.prev;
            let removed_node = self.arena.remove(tail);
            if let Some(prev) = prev {
                self.arena.at_mut(prev).unwrap().next = None;
                self.tail = Some(prev);
            } else {
                self.head = None;
                self.tail = None;
            }
            match removed_node {
                Some(node) => Some(node.value),
                _ => None,
            }
        } else {
            None
        }
    }
}
