use std::result::Result;

const STACK_SIZE: usize = 8;

#[derive(Debug)]
struct Stack {
    array: [Option<i32>; STACK_SIZE],
    size: usize, //#elements, also the next index to store element
    limit: usize,
}

impl Stack {
    fn new() -> Stack {
        Stack { 
            array: [None; STACK_SIZE],
            size: 0,
            limit: STACK_SIZE,
        }
    }

    fn from_array(arr: [i32; STACK_SIZE]) -> Stack {
        let mut new_stack = Stack::new();

        for i in 0..STACK_SIZE {
            new_stack.array[i] = Some(arr[i]);
            new_stack.size += 1;
        }
        
        new_stack
    }


    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, data: i32) {
        if self.size == self.limit {
            ();
        } else {
            self.array[self.size] = Some(data);
            self.size += 1;
        }
    }

    fn pop(&mut self) -> Result<Option<i32>, &str> {
        if self.is_empty() {
            Err("Stack is Empty")
        } else {
            Ok(self.array[self.size - 1])
        }
    }

}

fn main() {
    let mut stack1 = Stack::new();
    assert_eq!(stack1.is_empty(), true);
    stack1.push(1);
    println!("{:?}", stack1);
    let x = match stack1.pop() {
        Ok(i) => i,
        Err(_) => None,
    };
    
    println!("popped element is {:?}", x);

    let arr1 = [1, 2, 3, 4, 5, 0, 0, 0];
    let stack2 = Stack::from_array(arr1);
    println!("{:?}", stack2);
}

