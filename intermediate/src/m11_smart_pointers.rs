#[cfg(test)]
mod tests {
    use std::rc::{ Rc, Weak };
    use std::cell::RefCell;
    use std::vec;

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_box_smart_pointers() {

        #[derive(Debug)]
        struct Node {
            id: u32,
            next: Option<Box<Node>>
        }

        let nodes: Box<Node> = Box::new(
            Node {
                id: 0,
                next: Some(Box::new(
                    Node {
                        id: 1, 
                        next: Some(Box::new(
                            Node {
                                id: 2,
                                next: None
                            }
                        ))
                    }
                ))
            }
        );

        dbg!(nodes);
    }

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_reference_counter() {
        // Problem::
        // let mut x = 50;
        // let y = &x;
        // x = 70; <- issue
        // dbg!(y);

        // Solution::
        let x = Rc::new(RefCell::new(50)); // Keeping track of all the pointers using the Reference counter
        let y = Rc::clone(&x); // Creating a pointer to the existing data
        let z = Rc::clone(&x);

        *x.borrow_mut() = 70; // When x is changed, everything thats referencing it should also change in its value

        dbg!(y.borrow());
        dbg!(x.borrow());
        dbg!(z.borrow());
    }

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_circular_reference() {
        // Step 1:
        // #[derive(Debug)]
        // struct House {
        //     address_number: u16,
        //     street: String,
        //     furniture: Vec<Furniture>
        // }

        // #[derive(Debug)]
        // struct Furniture {
        //     id: String,
        //     description: String,
        //     house: House
        // }

        // let house_1 = House {
        //     address_number: 1,
        //     street: "coding avenue".to_string(),
        //     furniture: vec!()
        // };

        // let table = Furniture {
        //     id: "table1".to_string(),
        //     description: "kitchen table".to_string(),
        //     house: house_1
        // };

        // let desk = Furniture {
        //     id: "desk1".to_string(),
        //     description: "office desk".to_string(),
        //     house: house_1 // <- use of moved value: `house_1` ISSUE!!!
        // };

        // Step 2: Circular Reference Example
        // #[derive(Debug)]
        // struct House {
        //     address_number: u16,
        //     street: String,
        //     furniture: RefCell<Vec<Rc<Furniture>>>
        // }

        // #[derive(Debug)]
        // struct Furniture {
        //     id: String,
        //     description: String,
        //     house: Rc<House>
        // }

        // let house_1 = Rc::new(House {
        //     address_number: 1,
        //     street: "coding avenue".to_string(),
        //     furniture: RefCell::new(vec!())
        // });

        // let table = Rc::new(Furniture {
        //     id: "table1".to_string(),
        //     description: "kitchen table".to_string(),
        //     house: Rc::clone(&house_1)
        // });

        // let desk = Rc::new(Furniture {
        //     id: "desk1".to_string(),
        //     description: "office desk".to_string(),
        //     house: Rc::clone(&house_1)
        // });

        // house_1.furniture.borrow_mut().push(Rc::clone(&table));
        // house_1.furniture.borrow_mut().push(Rc::clone(&desk));

        // dbg!(house_1);

        // Step 3:
        #[derive(Debug)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>
        }

        #[derive(Debug)]
        struct Furniture {
            id: String,
            description: String,
            // Similar to '&', Weak pointer can reference an object without owning them. 
            // But unlike '&', a weak pointer does not prevent an object from being dropped.
            // When a normal reference '&' is created, The rust borrow checker ensures that 
            // the referenced object is not dropped for the duration of the reference. 
            // This is not the case with the Weak pointer.
            house: Weak<House>
        }

        let house_1 = Rc::new(House {
            address_number: 1,
            street: "coding avenue".to_string(),
            furniture: RefCell::new(vec!())
        });

        let table = Rc::new(Furniture {
            id: "table1".to_string(),
            description: "kitchen table".to_string(),
            house: Rc::downgrade(&house_1)
        });

        let desk = Rc::new(Furniture {
            id: "desk1".to_string(),
            description: "office desk".to_string(),
            house: Rc::downgrade(&house_1)
        });

        house_1.furniture.borrow_mut().push(Rc::clone(&table));
        house_1.furniture.borrow_mut().push(Rc::clone(&desk));
        
        dbg!(house_1);
    }
}