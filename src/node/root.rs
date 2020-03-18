use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::node::Node;

use crate as altseed2;
use crate::{create_node, define_node};

define_node! {
    pub struct RootNode {
        phantom: PhantomData<()>,
    }
}

impl Node for RootNode {}

impl RootNode {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(create_node! {
            RootNode { phantom: PhantomData }
        }))
    }
}
