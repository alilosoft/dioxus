use dioxus::prelude::*;
use dioxus_core::*;
use dioxus_native_core::prelude::*;
use dioxus_native_core::{
    dioxus::DioxusState,
    node_ref::{AttributeMaskBuilder, NodeMaskBuilder, NodeView},
    real_dom::RealDom,
    Dependancy, Pass, SendAnyMap,
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::any::TypeId;
use std::sync::{Arc, Mutex};

macro_rules! dep {
    ( child( $name:ty, $dep:ty ) ) => {
        impl Pass for $name {
            type ParentDependencies = ();
            type ChildDependencies = $dep;
            type NodeDependencies = ();

            const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::ALL;

            fn pass<'a>(
                &mut self,
                _: NodeView,
                _: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
                parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
                _: Option<Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>>,
                _: &SendAnyMap,
            ) -> bool {
                self.0 += 1;
                true
            }

            fn create<'a>(
                node_view: NodeView<()>,
                node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
                parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
                children: Option<Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>>,
                context: &SendAnyMap,
            ) -> Self {
                let mut myself = Self::default();
                myself.pass(node_view, node, parent, children, context);
                myself
            }
        }
    };

    ( parent( $name:ty, $dep:ty ) ) => {
        impl Pass for $name {
            type ParentDependencies = $dep;
            type ChildDependencies = ();
            type NodeDependencies = ();

            const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::ALL;

            fn pass<'a>(
                &mut self,
                _: NodeView,
                _: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
                parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
                _: Option<Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>>,
                _: &SendAnyMap,
            ) -> bool {
                self.0 += 1;
                true
            }

            fn create<'a>(
                node_view: NodeView<()>,
                node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
                parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
                children: Option<Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>>,
                context: &SendAnyMap,
            ) -> Self {
                let mut myself = Self::default();
                myself.pass(node_view, node, parent, children, context);
                myself
            }
        }
    };

    ( node( $name:ty, $dep:ty ) ) => {
        impl Pass for $name {
            type ParentDependencies = $dep;
            type ChildDependencies = ();
            type NodeDependencies = ();

            const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::ALL;

            fn pass<'a>(
                &mut self,
                _: NodeView,
                _: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
                parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
                _: Option<Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>>,
                _: &SendAnyMap,
            ) -> bool {
                self.0 += 1;
                true
            }

            fn create<'a>(
                node_view: NodeView<()>,
                node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
                parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
                children: Option<Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>>,
                context: &SendAnyMap,
            ) -> Self {
                let mut myself = Self::default();
                myself.pass(node_view, node, parent, children, context);
                myself
            }
        }
    };
}

macro_rules! test_state{
    ( state: ( $( $state:ty ),* ) ) => {
        #[test]
        fn state_reduce_initally_called_minimally() {
            #[allow(non_snake_case)]
            fn Base(cx: Scope) -> Element {
                render!{
                    div {
                        div{
                            div{
                                p{}
                            }
                            p{
                                "hello"
                            }
                            div{
                                h1{}
                            }
                            p{
                                "world"
                            }
                        }
                    }
                }
            }

            let mut vdom = VirtualDom::new(Base);

            let mutations = vdom.rebuild();

            let mut dom: RealDom = RealDom::new(Box::new([$( <$state>::to_type_erased() ),*]));
            let mut dioxus_state = DioxusState::create(&mut dom);

            dioxus_state.apply_mutations(&mut dom, mutations);
            dom.update_state(SendAnyMap::new(), false);

            dom.traverse_depth_first(|n| {
                $(
                    assert_eq!(n.get::<$state>().unwrap().0, 1);
                )*
            });
        }
    }
}

mod node_depends_on_child_and_parent {
    use super::*;
    #[derive(Debug, Clone, Default, PartialEq)]
    struct Node(i32);
    dep!(node(Node, (Child, Parent)));

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Child(i32);
    dep!(child(Child, (Child,)));

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Parent(i32);
    dep!(parent(Parent, (Parent,)));

    test_state!(state: (Child, Node, Parent));
}

mod child_depends_on_node_that_depends_on_parent {
    use super::*;
    #[derive(Debug, Clone, Default, PartialEq)]
    struct Node(i32);
    dep!(node(Node, (Parent,)));

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Child(i32);
    dep!(child(Child, (Node,)));

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Parent(i32);
    dep!(parent(Parent, (Parent,)));

    test_state!(state: (Child, Node, Parent));
}

mod parent_depends_on_node_that_depends_on_child {
    use super::*;
    #[derive(Debug, Clone, Default, PartialEq)]
    struct Node(i32);
    dep!(node(Node, (Child,)));

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Child(i32);
    dep!(child(Child, (Child,)));

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Parent(i32);
    dep!(parent(Parent, (Node,)));

    test_state!(state: (Child, Node, Parent));
}

mod node_depends_on_other_node_state {
    use super::*;
    #[derive(Debug, Clone, Default, PartialEq)]
    struct Node1(i32);
    dep!(node(Node1, (Node2,)));

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Node2(i32);
    dep!(node(Node2, ()));

    test_state!(state: (Node1, Node2));
}

mod node_child_and_parent_state_depends_on_self {
    use super::*;
    #[derive(Debug, Clone, Default, PartialEq)]
    struct Node(i32);
    dep!(node(Node, ()));

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Child(i32);
    dep!(child(Child, (Child,)));

    #[derive(Debug, Clone, Default, PartialEq)]
    struct Parent(i32);
    dep!(parent(Parent, (Parent,)));

    test_state!(state: (Child, Node, Parent));
}
