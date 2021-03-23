#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn it_push() {
        let mut linked: LinkedList<i32> = LinkedList::default();
        linked.push(1);
        linked.push(2);

        let p = linked.pop();
        assert_eq!(p.unwrap().elem, 2);
        let p = linked.pop();
        assert_eq!(p.unwrap().elem, 1);

        linked.push(1);
        linked.push(2);
        assert_eq!(Some(2), linked.next());
        assert_eq!(Some(1), linked.next());
        assert_eq!(None, linked.next());
    }
    #[test]
    fn it_remove() {
        let mut linked: LinkedList<i32> = LinkedList::default();
        linked.push(1);
        linked.push(2);
        linked.remove(1);
        assert_eq!(Some(2), linked.next());
        assert_eq!(None, linked.next());
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }
}
impl<T: PartialEq + Clone> LinkedList<T> {
    // insert node to head
    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        if self.tail.is_none() {
            self.tail = Some(Box::clone(&node))
        }
        self.head = Some(node);
    }
    pub fn remove(&mut self, elem: T) {
        let mut node = &mut self.head;
        loop {
            if node.is_some() {
                if node.as_ref().unwrap().elem == elem {
                    *node = node.take().unwrap().next;
                    break;
                } else {
                    node = &mut node.as_mut().unwrap().next;
                }
            } else {
                break;
            }
        }
    }
    pub fn pop(&mut self) -> Link<T> {
        let p = self.head.take();
        let p1 = p.clone();
        if p.is_some() {
            self.head = p.unwrap().next;
        } else {
            self.head = None;
        }
        p1
    }
}
impl<T: PartialEq + Clone> Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.pop() {
            Some(e.elem)
        } else {
            None
        }
    }
}
pub fn execute() {}
