pub struct List<T> {
  elements: Vec<T>,
}

impl<T> List<T> {

  pub fn new() -> Self {
    List {elements: Vec::new()}
  }

  pub fn push(&mut self, element: T) {
    self.elements.push(element);
  }

  pub fn remove_index(&mut self, index: usize) -> Option<T> {
    if index < self.elements.len() {
      Some(self.elements.remove(index))
    } else {
      None
    }
  }

  pub fn remove(&mut self, target: &T) -> Option<T> where T: PartialEq, {
    if let Some(index) = self.elements.iter().position(|e| e == target) {
      Some(self.elements.remove(index))
    } else {
      None
    }
  }

  pub fn find(&self, target: &T) -> Option<&T> where T: PartialEq, {
    self.elements.iter().find(|&e| e == target)
  }

  pub fn is_empty(&self) -> bool {
    self.elements.is_empty()
  }

}