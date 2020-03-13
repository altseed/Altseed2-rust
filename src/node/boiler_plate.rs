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
            node_base: altseed2::node::NodeBase,
            $(
                $(#[$meta_v])*
                $variant: $ty,
            )*
        }

        impl< $( $N $(: $b0 $(+$b)* )? ),* > altseed2::node::HasNodeBase for $name< $( $N ),* > {
            fn node_base(&self) -> &NodeBase {
                &self.node_base
            }

            fn node_base_mut(&mut self) -> &mut altseed2::node::NodeBase {
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
            node_base: altseed2::node::NodeBase::default(),
            $($variant : $e,)*
        }
    };
}

macro_rules! define_drawn_node {(
    $(#[$meta_s:meta])*
    pub struct $name: ident: $type: ty {
        $(
            $(#[$meta_v:meta])*
            $variant: ident : $ty: ty,
        )*
    }) => {
        define_drawn_node!(
            $(#[$meta_s])*
                pub struct $name <>: $type {
                $(
                    $(#[$meta_v])*
                    $variant: $ty,
                )*
            }
        );
    };
    (
    $(#[$meta_s:meta])*
    pub struct $name: ident < $( $N:ident $(: $b0:ident $(+$b:ident)* )? ),* >: $type: ty {
        $(
            $(#[$meta_v:meta])*
            $variant: ident : $ty: ty,
        )*
    }) => {
        define_node!{
            $(#[$meta_s])*
            pub struct $name< $( $N $(: $b0 $(+$b)* )? ),* > {
                instance: $type,
                trans: Transform,
                z_order: i32,
            }
        }
        impl< $( $N $(: $b0 $(+$b)* )? ),* > $name< $( $N $(: $b0 $(+$b)* )? ),* > {
            pub fn transform(&self) -> &crate::prelude::Transform {
                &self.trans
            }

            pub fn transform_mut(&mut self) -> &mut crate::prelude::Transform {
                &mut self.trans
            }

            pub fn set_pos(&mut self, pos: crate::prelude::Vector2<f32>) -> &mut Self {
                *self.trans.pos_mut() = pos;
                self
            }

            pub fn set_scale(&mut self, scale: crate::prelude::Vector2<f32>) -> &mut Self {
                *self.trans.scale_mut() = scale;
                self
            }

            pub fn set_angle(&mut self, angle: f32) -> &mut Self {
                *self.trans.angle_mut() = angle;
                self
            }

            /// 描画順を取得します。
            pub fn get_z_order(&self) -> i32 {
                self.z_order
            }

            /// 描画順を設定します。
            pub fn set_z_order(&mut self, z_order: i32) -> &mut Self {
                self.z_order = z_order;
                self
            }

            /// マテリアルを取得します。
            pub fn get_material(&mut self) -> Rc<RefCell<crate::prelude::Material>> {
                self.instance.get_material().unwrap()
            }

            /// マテリアルを設定します。
            pub fn set_material(&mut self, mat: Rc<RefCell<crate::prelude::Material>>) -> &mut Self {
                self.instance.set_material(mat);
                self
            }
        }


        impl< $( $N $(: $b0 $(+$b)* )? ),* > crate::node::DrawnNode for $name< $( $N $(: $b0 $(+$b)* )? ),* > {
            fn transform(&self) -> &crate::prelude::Transform {
                $name::transform(self)
            }

            fn transform_mut(&mut self) -> &mut crate::prelude::Transform {
                $name::transform_mut(self)
            }

            fn get_z_order(&self) -> i32 {
                $name::get_z_order(self)
            }

            fn set_z_order(&mut self, z_order: i32) -> &mut dyn DrawnNode {
                $name::set_z_order(self, z_order)
            }

            fn get_material(&mut self) -> Rc<RefCell<crate::prelude::Material>> {
                $name::get_material(self)
            }

            fn set_material(&mut self, mat: Rc<RefCell<crate::prelude::Material>>) -> &mut dyn DrawnNode {
                $name::set_material(self, mat)
            }
        }

    };
}
