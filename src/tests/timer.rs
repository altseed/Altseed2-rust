mod timer {
    use altseed2::prelude::*;
    use altseed2::*;
    use std::{cell::RefCell, rc::Rc};

    define_node! {
        pub struct TimerNode {
            count: i32,
            limit: i32,
        }
    }

    impl TimerNode {
        pub fn new(limit: i32) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(create_node!(TimerNode { count: 0, limit: limit })))
        }
    }

    impl Node for TimerNode {
        fn on_added(&mut self, _: &mut Engine) -> AltseedResult<()> {
            println!("Started");
            Ok(())
        }

        fn on_updated(&mut self, engine: &mut Engine) -> AltseedResult<()> {
            if self.count == self.limit {
                engine.close();
                println!("Finished");
                return Ok(());
            }

            self.count += 1;

            Ok(())
        }
    }
}