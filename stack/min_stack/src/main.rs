use std::cmp::Ord; 

#[derive(Clone)]
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}
impl MinStack {
    fn new() -> Self {
        MinStack { 
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, t: i32) {
        if let Some(min) = self.min_top() {
            let minimum = min.min(t);
            self.min_stack.push(minimum);
        }
        else {
            self.min_stack.push(t);
        }
        self.stack.push(t); 
    }

    fn pop(&mut self) {
        self.min_stack.pop();
        self.stack.pop(); 
    }

    fn top(&mut self) -> Option<i32> {
        if self.stack.is_empty() {
            return None;
        }

        let top = &self.stack[self.stack.len() - 1]; 
        Some(*top) 
    }

    fn min_top(&mut self) -> Option<i32> {
        if self.min_stack.is_empty() {
            return None; 
        }

        let min_top = &self.min_stack[self.min_stack.len() - 1]; 
        Some(*min_top)
    }

    fn get_min(&mut self) -> Option<i32> {
        self.min_top()
    }

}

fn main() {
    let mut minStack = MinStack::new(); 
    minStack.push(1); 
    minStack.push(2);
    minStack.push(0); 
    if let Some(min) = minStack.get_min() {
        println!("The current minimum value is {}", min); 
    }

    minStack.pop();
    if let Some(top) = minStack.top() {
        println!("The current top value is {}", top); 
    }
    
    if let Some(min) = minStack.get_min() {
        println!("The current minimum value is {}", min); 
    }
}
