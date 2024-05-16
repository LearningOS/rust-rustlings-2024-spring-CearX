/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        // 将元素入栈时，将其入队到 q1 中
        self.q1.enqueue(elem); 
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
		//Err("Stack is empty")
        // 出栈时，我们需要将元素从 q1 转移到 q2，直到 q1 中只剩下一个元素为止
        // 这最后一个元素就是要出栈的元素
        if !self.q1.is_empty() {
            // 如果 q1 不为空，尝试从中弹出元素
            while let Ok(val) = self.q1.dequeue() {
                if self.q1.is_empty() {
                    return Ok(val);
                } else {
                    self.q2.enqueue(val);
                }
            }
        } else if !self.q2.is_empty() {
            // 如果 q1 为空，但 q2 不为空，尝试从 q2 中弹出元素
            while let Ok(val) = self.q2.dequeue() {
                if self.q2.is_empty() {
                    return Ok(val);
                } else {
                    self.q1.enqueue(val);
                }
            }
        }
    
        // 如果 q1 和 q2 都为空，则返回栈为空的错误消息
        Err("Stack is empty")
    }
    
    pub fn is_empty(&self) -> bool {
		//TODO
        //true
        // 当且仅当 q1 和 q2 都为空时，栈为空
        self.q1.is_empty() && self.q2.is_empty() 
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}