#[macro_export]
macro_rules! define_node {(
    $(#[$meta_s:meta])*
    pub struct $name: ident {
        $(
            $(#[$meta_v:meta])*
            $variant: ident : $ty: ty,
        )*
    }) => {
        define_node!(
            $(#[$meta_s])*
                pub struct $name <> {
                $(
                    $(#[$meta_v])*
                    $variant: $ty,
                )*
            }
        );
    };
    (
    $(#[$meta_s:meta])*
    pub struct $name: ident < $( $N:ident $(: $b0:ident $(+$b:ident)* )? ),* > {
        $(
            $(#[$meta_v:meta])*
            $variant: ident : $ty: ty,
        )*
    }) => {
        $(#[$meta_s])*
        #[derive(Debug)]
        pub struct $name< $( $N $(: $b0 $(+$b)* )? ),* > {
            node_base: altseed2::node::BaseNode,
            $(
                $(#[$meta_v])*
                $variant: $ty,
            )*
        }

        impl< $( $N $(: $b0 $(+$b)* )? ),* > altseed2::node::HasBaseNode for $name< $( $N ),* > {
            fn node_base(&self) -> &BaseNode {
                &self.node_base
            }

            fn node_base_mut(&mut self) -> &mut altseed2::node::BaseNode {
                &mut self.node_base
            }
        }
    };
}

#[macro_export]
macro_rules! create_node {(
    $name: ident {
        $(
            $variant: ident : $e: expr,
        )*
    }) => {
        $name {
            node_base: altseed2::node::BaseNode::default(),
            $($variant : $e,)*
        }
    };
}
