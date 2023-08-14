use gdnative::prelude::*;
use rusty_ytdl::*;
//use gdnative::tasks::Async;


#[allow(non_camel_case_types)]
#[derive(NativeClass)]
#[inherit(Node)]
pub struct RustubeNode;

//pub trait Method<RustubeNode> {}
/* Rust YTDL compiles without dependencies */
#[methods]
impl RustubeNode {
    fn new(_base: &Node) -> Self {
        godot_print!("Initializing Rustube");
        RustubeNode{}
    }

    fn _ready(&self, _base: &Node) {
        godot_print!("Testing Native Plugin");
        //RustubeNode::test()
    }

    #[method]  
    async fn download(&self, url: String) {
      //let video_url = "https://www.youtube.com/watch?v=FZ8BxMU3BYc"; // FZ8BxMU3BYc works too!
      let video = Video::new(url).unwrap();

      let stream = video.stream().await.unwrap();

      while let Some(chunk) = stream.chunk().await.unwrap() {
        // Do what you want with chunks
        godot_print!("{:#?}", chunk);
      }

      // Or direct download to path
      let path = std::path::Path::new(r"test.mp3");

      video.download(path).await.unwrap();
        }
    }

/*  Async Reciepe*/

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



#[gdnative::init::callbacks]
impl GDNativeCallbacks for RustubeNode {
     fn nativescript_init(handle: InitHandle) {
    
        handle.add_class::<RustubeNode>();    
        
        gdnative::tasks::register_runtime(&handle);
        gdnative::tasks::set_executor(EXECUTOR.with(|e| *e));
        handle.add_class::<AsyncExecutorDriver>();
    }

}

