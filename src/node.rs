pub mod node {
  use std::rc::Rc;

  pub type Node<T> = Option<Rc<Box<InnerNode<T>>>>;
  pub struct InnerNode<T> {
    prev: Node<T>,
    next: Node<T>,
    data: Rc<T>
  }

  impl<T> InnerNode<T>{
    pub fn new(data: T) -> InnerNode<T>
    {
      InnerNode {
        prev: None,
        next: None,
        data: Rc::new(data)
      } 
    }

    pub fn get_data(&self) -> Rc<T>
    {
      self.data.clone()
    }

    pub fn get_prev(&self) -> Node<T>
    {
      self.prev.clone()
    }
    pub fn get_next(&self) -> Node<T>
    {
      self.next.clone()
    }
    pub fn set_prev(&mut self, new_prev: &Node<T>) -> Node<T>
    {
      let old_prev: Node<T> = self.prev.clone();
      self.prev = new_prev.clone();
      old_prev
    }
    pub fn set_next(&mut self, new_next: &Node<T>) -> Node<T>
    {
      let old_next: Node<T> = self.next.clone();
      self.next = new_next.clone();
      old_next
    }
  }
}