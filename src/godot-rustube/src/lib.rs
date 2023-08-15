use gdnative::prelude::*;
use rusty_ytdl::*;
//use gdnative::tasks::Async;

//simplifies async methods with custom macros
use macros::*;
use std::rc::Rc;
use gdnative::tasks::{Async, AsyncMethod, Spawner};
use gdnative::tasks::Yield;

// Futures Trait
use futures::Future as future;
use futures::task::Poll;
use std::pin::Pin;

//GDNative VideoStream Class
use gdnative::api::VideoStream;

//Required Deps
use futures::task::Context;

#[allow(non_camel_case_types)]
#[derive(NativeClass)]
#[inherit(Node)]

/*Define the Struct*/
pub struct RustubeNode{
    pub name: Rc<String>, //required for the custom async macro implementation
    pub stream : Option<VideoStream>
}

//pub trait Method<RustubeNode> {}
/* Rust YTDL compiles without dependencies */
#[methods]
impl RustubeNode {
    fn new(_base: &Node) -> Self {
        godot_print!("Initializing Rustube");
        RustubeNode{
            //placeholder accounts for algod node requirement in custom macro
            algod: Rc::new(String::new()) 
        }
    }

    fn _ready(&self, _base: &Node) {
        godot_print!("Testing Native Plugin");
        //RustubeNode::test()
    }
    
    /* Async Modve Original PR: https://github.com/godot-rust/gdnative/pull/709*/
    //#[method]
    //#[method(async)]
       
   

    }


/*Futures Trait for Download Method */

trait MyTrait {

    type Output = VideoStream;
    
}

/* It Implements GDNative Futures Trait && Rust Futures Trait*/
impl <T : MyTrait> future for RustubeNode {
    

    fn download()-> VideoStream{
        todo!()
    }

    
    // Required method
    fn poll(
    self: Pin<&mut Yield<T>>,
    cx: &mut Context<'_>
    ) -> Poll<<Yield<T> as future>::Output>{
        todo!()

    }

}

asyncmethods!(name, node, this,
    fn download(_ctx, _args)  {
      let download = async move {
          let url = "https://www.youtube.com/watch?v=FZ8BxMU3BYc"; // FZ8BxMU3BYc works too!
          let video = Video::new(url).unwrap();

          let stream = video.stream().await.unwrap();

          while let Some(chunk) = stream.chunk().await.unwrap() {
            // Do what you want with chunks
            godot_print!("{:#?}", chunk);
          }

          // Or direct download to path
          let path = std::path::Path::new(r"test.mp3");

          video.download(path).await.unwrap();
        //Ok(())
        };
    
    }    
);

/*  Async Reciepe 1*/

thread_local! {
    static EXECUTOR: &'static SharedLocalPool = {
        Box::leak(Box::new(SharedLocalPool::default()))
    };
}

use tokio::task::LocalSet;

#[derive(Default)]
struct SharedLocalPool {
    local_set: LocalSet,
}

impl futures::task::LocalSpawn for SharedLocalPool {
    fn spawn_local_obj(
        &self,
        future: futures::task::LocalFutureObj<'static, ()>,
    ) -> Result<(), futures::task::SpawnError> {
        self.local_set.spawn_local(future);

        Ok(())
    }
}


use tokio::runtime::{Builder, Runtime};

#[derive(NativeClass)]
#[inherit(Node)]
struct AsyncExecutorDriver {
    runtime: Runtime,
}

impl AsyncExecutorDriver {
    fn new(_base: &Node) -> Self {
        AsyncExecutorDriver {
            runtime: Builder::new_current_thread()
                .enable_io()    // optional, depending on your needs
                .enable_time()  // optional, depending on your needs
                .build()
                .unwrap(),
        }
    }
}

#[methods]
impl AsyncExecutorDriver {
    #[method]
    fn _process(&self, #[base] _base: &Node, _delta: f64) {
        EXECUTOR.with(|e| {
            self.runtime
                .block_on(async {
                    e.local_set
                        .run_until(async {
                            tokio::task::spawn_local(async {}).await
                        })
                        .await
                })
                .unwrap()
        })
    }
}


/*
/* Async Recepie Original PR */
fn register_methods(builder: &ClassBuilder<RustubeNode>) {
    builder
        .build_method("download", Async::new(RustubeNode))
        .done();
}
*/


#[gdnative::init::callbacks]
impl GDNativeCallbacks for RustubeNode {
     fn nativescript_init(handle: InitHandle) {
    
        handle.add_class::<RustubeNode>();    
        
        gdnative::tasks::register_runtime(&handle);
        gdnative::tasks::set_executor(EXECUTOR.with(|e| *e));
        
        handle.add_class::<AsyncExecutorDriver>();
    }

}

