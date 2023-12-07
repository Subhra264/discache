use super::gen_arena::GenArena;

pub struct DoublyLinkedList<T> {
    arena: GenArena<Node<T>>,
    head: usize,
    tail: usize,
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
            head: 0,
            tail: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DoublyLinkedList {
            arena: GenArena::with_capacity(capacity),
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, elem: T) -> Result<(), &'static str> {
        self.arena.push(Node {
            value: elem,
            prev: None,
            next: None,
        })
    }
}
