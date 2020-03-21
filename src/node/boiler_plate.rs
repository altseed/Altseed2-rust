#[macro_export]
/// [Node](node/trait.Node.html)トレイトを実装した構造体定義するために使用するマクロです。Debugトレイトが自動実装されます。
/// # Examples
/// ```no_run
/// use altseed2::*;
/// use altseed2::prelude::*;
///
/// define_node! {
///     pub struct SampleNode {
///         // ここで構造体のフィールドを書きます。
///     }
/// }
/// impl Node for SampleNode { }
/// ```
macro_rules! define_node {
    // フィールドが空の場合
    ($(#[$meta_s:meta])*
    pub struct $name: ident {

    }) => {
        $(#[$meta_s])*
        #[derive(Debug)]
        pub struct $name {
            __node_base: altseed2::node::BaseNode,
        }

        impl altseed2::node::HasBaseNode for $name {
            fn node_base(&self) -> &altseed2::node::BaseNode {
                &self.__node_base
            }

            fn node_base_mut(&mut self) -> &mut altseed2::node::BaseNode {
                &mut self.__node_base
            }
        }
    };
    // 基本の実装
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
            __node_base: altseed2::node::BaseNode,
            $(
                $(#[$meta_v])*
                $variant: $ty,
            )*
        }

        impl< $( $N $(: $b0 $(+$b)* )? ),* > altseed2::node::HasBaseNode for $name< $( $N ),* > {
            fn node_base(&self) -> &altseed2::node::BaseNode {
                &self.__node_base
            }

            fn node_base_mut(&mut self) -> &mut altseed2::node::BaseNode {
                &mut self.__node_base
            }
        }
    };
    // フィールドの区切られ方で変わるので
    ($(#[$meta_s:meta])*
    pub struct $name: ident {
        $(
            $(#[$meta_v:meta])*
            $variant: ident : $ty: ty
        ),*
    }) => {
        define_node!(
            $(#[$meta_s])*
            pub struct $name <> {
                $(
                    $(#[$meta_v])*
                    $variant: $ty
                ),*
            }
        );
    };
    ($(#[$meta_s:meta])*
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
            $variant: ident : $ty: ty
        ),*
    }) => {
        define_node!(
            $(#[$meta_s])*
            pub struct $name < $( $N $(: $b0 $(+$b)* )? ),* > {
                $(
                    $(#[$meta_v])*
                    $variant: $ty
                ),*,
            }
        );
    };
}

/// [define_node](macro.define_node!.html)マクロを使って定義した`Node`を作成するためのマクロです。
/// # Examples
/// ```no_run
/// use altseed2::*;
/// use altseed2::prelude::*;
/// use std::{rc::Rc, cell::RefCell};
///
/// define_node! {
///     pub struct SampleNode {
///         foo: i32,
///     }
/// }
/// impl Node for SampleNode { }
///
/// impl SampleNode {
///     pub fn new() -> Rc<RefCell<Self>> {
///         Rc::new(RefCell::new(create_node!(
///             SampleNode {
///                 foo: 128,
///             }
///         )))
///     }
/// }
/// ```
#[macro_export]
macro_rules! create_node {
    ($name: ident { }) => {
        $name {
            __node_base: altseed2::node::BaseNode::default(),
        }
    };
    ($name: ident {
        $(
            $variant: ident : $e: expr,
        )*
    }) => {
        $name {
            __node_base: altseed2::node::BaseNode::default(),
            $($variant : $e,)*
        }
    };
    ($name: ident {
        $(
            $variant: ident : $e: expr
        ),*
    }) => {
        create_node!(
            $name {
                $($variant: $e),* ,
            }
        )
    };
}
