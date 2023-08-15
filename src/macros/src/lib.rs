pub use paste::paste;

/* 
    Macro for simplifying AsyncMethod impl


    In this part of the code, the macro is generating implementations of the AsyncMethod trait for each method defined using the asyncmethods macro. Let's break it down step by step:
    impl AsyncMethod<Algodot> for [<__ $fn>] { ... }: This block of code implements the AsyncMethod trait for a specific type, which is generated using the paste macro. The type name is based on the name of the method being defined.
    fn spawn_with(&self, spawner: Spawner<'_, Algodot>) { ... }: This defines the spawn_with method required by the AsyncMethod trait. The spawner parameter is used to spawn asynchronous tasks.
    spawner.spawn(|$ctx, $this, mut $args| { ... });: This line uses the spawn method of the provided spawner to initiate the execution of an asynchronous task. The closure inside spawn contains the code that will be executed asynchronously.
    $ctx: This is a parameter representing the context of the asynchronous task. It might contain information about the context in which the asynchronous method is being executed.
    $this: This is a parameter representing the instance of the type on which the asynchronous method is being called. It provides access to the instance's data and methods.
    $args: This is a parameter representing the arguments passed to the asynchronous method. It allows you to access and modify the method's input arguments.
    let ($algod, $node) = $this.map(|algodot, node| { ... }).unwrap();: This line uses the map method to transform the value of $this. The transformation is applied to a tuple (algodot, node) where algodot is an Rc clone of a shared data member and node is acquired through the claim method. The unwrap method is used to extract the transformed values from the Option returned by map.
    $block: This placeholder represents the block of code associated with the method being defined. Inside this block, you have access to $algod, $node, $ctx, and $args, which are all parameters representing the context and data required for the asynchronous task.


    In summary, this part of the code defines how the asynchronous tasks are spawned and executed. It uses the spawner to initiate asynchronous execution of the provided block of code, which has access to the context, instance, and arguments of the asynchronous method. The code within the block can manipulate these parameters and execute asynchronous logic within the context of the Godot game engine.
*/

#[macro_export]
macro_rules! asyncmethods {
    ($name:ident, $node:ident, $this:ident, $( fn $fn:ident($ctx:ident, $args:ident) $block:block) *) => {
        $crate::paste! {
            $(
                #[allow(non_camel_case_types)]
                struct [<__ $fn>];

                impl AsyncMethod<RustubeNode> for [<__ $fn>] {
                    fn spawn_with(&self, spawner: Spawner<'_, RustubeNode>) {
                        spawner.spawn(|$ctx, $this, mut $args| {
                            #[allow(unused_variables)]
                            /* Maps and transforms $this into a tupule of  algodot and node*/
                            let ($name, $node) = $this.map(|name, node| {
                                (Rc::clone(&RustubeNode::name), node.claim())
                            }).unwrap();
                            

                            $block

                        });
                    }
                }
            ) *

            fn register_methods(builder: &ClassBuilder<RustubeNode>) {
                $ (
                    builder.method(stringify!($fn), Async::new([<__ $fn >])).done();
                ) *
            }
        }
    };
}

/// Converts from `Result<T, E>` to `Option<T>`, printing the error to godot's stderr.
#[macro_export]
macro_rules! godot_unwrap {
    ($res:ident) => {
        match $res {
            Ok(ok) => Some(ok),
            Err(err) => {
                godot_error!("{:?}", err);
                None
            }
        }
    };
}
