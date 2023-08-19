pub mod doubly_linked_list {
  use crate::node::node::Node;


  pub struct DoublyLinkedList<T>{
    head: Node<T>,
    tail: Node<T>,
    length: i32
  }

  impl <T>DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T>
    {
      DoublyLinkedList { 
        head: None, 
        tail: None,
        length: 0 
      }
    }

    pub fn size(&self) -> i32 {
      self.length
    }

    pub fn push(&self) {
      
    }

    pub fn pop(&self) -> Node<T>{

    }

    pub fn delete(&self) -> Node<T>{

    }

    pub fn update(&self, node: Node<T>) -> T{

    }

    pub fn seach(&self, data: T) -> Node<T>{

    }

  }
}