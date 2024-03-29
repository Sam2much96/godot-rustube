use gdnative::prelude::*;
use rusty_ytdl::*;
//use gdnative::tasks::Async;

//simplifies async methods with custom macros
//use gdnative::tasks::Yield;
//use gdnative::tasks::{Async, AsyncMethod, Spawner};
//use macros::*;
use std::rc::Rc;

// Futures Trait
//use futures::task::Poll;
use futures::Future as future;
use std::pin::Pin;

//GDNative Futures
//use gdnative::tasks::Context as gdnativeContext;

//GDNative VideoStream Class
//use gdnative::api::VideoStream;
//use gdnative::api::VideoStreamWebm;
use gdnative::api::VideoStreamTheora;
//use gdnative::api::VideoPlayer;


//Required Deps
//use futures::task::Context as futuresContext;
//use std::ops::Deref;

#[allow(non_camel_case_types)]
#[derive(NativeClass)]
#[inherit(Node)]

/*Define the Struct*/
pub struct RustubeNode {
    pub name: Rc<String>, // Introduce a Reference Counter for Rustube Node //required for the custom async macro implementation
    pub stream: Option<VideoStreamTheora>,
}

//pub trait Method<RustubeNode> {}
/* Rust YTDL compiles without dependencies */
#[methods]
impl RustubeNode {
    fn new(_base: &Node) -> Self {
        godot_print!("Initializing Rustube");
        RustubeNode {
            //Video Download Name


            //placeholder accounts for algod node requirement in custom macro
            name: Rc::new(String::from("Hello, Rustube!")),
            stream: None,
        }
    }

    fn _ready(&self, _base: &Node) {
        godot_print!("Testing Native Plugin");
    }

    /* Method Compiles But Breaks Library Init*/
    //#[method]
    //fn test(&self, #[base] _base: &Node){
    //    Self::download();
    //  }

    /* Async Modve Original PR: https://github.com/godot-rust/gdnative/pull/709*/

}

/*Defines Futures Trait for Download Method */
// 
// Bug : 
//      (1) Download Method Breaks the Codebase Init
//      (2) Codebase duplicates Two Async Methods. Document and Implement Both 
//      (3) No method in Rustube.gd to Consume the Downloaded File
//      (4) No From or Into Variant Trait for Rustube for Strings and VideoStreams
//      (5) Download Doesn't Receive the Url
//      (6) No Documentation
//
#[allow(non_camel_case_types)]
/* Defines the trait for asynchronous operations */
trait MyTrait {
    type VideoStream;
    type Future: future<Output = Self::VideoStream>; // Use associated type for Future
                                                     // Remove unnecessary associated types

    fn download() -> Self::VideoStream; // Use associated type for Future
}


/* Implement trait bound `VideoStream: From<rusty_ytdl::Video> */

/* Implements the trait for RustubeNode */
/* Implement Lifetime */
impl MyTrait for RustubeNode {
    type VideoStream = String; // Assuming VideoStream is your actual type
    type Future = Pin<Box<dyn future<Output = Self::VideoStream>>>; // Implement Future type


    fn download() -> Self::VideoStream {
        
        let t = String::new();
        let _ =Box::pin(async move {
            // Implement your async download logic here
            //todo!()
                        /* Download Video Async Logic */
            let video_url = "https://www.youtube.com/watch?v=FZ8BxMU3BYc"; // FZ8BxMU3BYc works too!

            godot_print!("Video Url: {}",&video_url);

            let video = Video::new(video_url).unwrap();

            let stream = video.stream().await.unwrap();

            while let Some(chunk) = stream.chunk().await.unwrap() {
                // Do what you want with chunks
                godot_print!("{:#?}", chunk);
            }

            // Or direct download to path
            let path = std::path::Path::new(r"test.mp3");

            video.download(path).await.unwrap();

            /*
            //
            // Or with options
            //

            let video_options = VideoOptions {
                quality: VideoQuality::Lowest,
                filter: VideoSearchOptions::Audio,
                ..Default::default()
            };

            let video = Video::new_with_options(video_url, video_options).unwrap();

            let stream = video.stream().await.unwrap();

            while let Some(chunk) = stream.chunk().await.unwrap() {
                // Do what you want with chunks
                println!("{:#?}", chunk);
            }

            // Or direct download to path
            let path = std::path::Path::new(r"test.mp3");

            //video.download(path).await.unwrap();
            video.download(path).await.unwrap();
            */
        });
        t
        //let y = t.deref();
        //y
    }
}

/*
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
*/

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
                .enable_io() // optional, depending on your needs
                .enable_time() // optional, depending on your needs
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
                        .run_until(async { tokio::task::spawn_local(async {}).await })
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
