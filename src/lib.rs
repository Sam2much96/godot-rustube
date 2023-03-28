// Rust GDNative implementation of this Godot tutorial:
// https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html#a-custom-node

use rustube;
use gdnative::export::OwnerArg;
use gdnative::object::TRef;
use gdnative::prelude::*;
use gdnative::api::HTTPRequest;

#[derive(NativeClass)]
#[inherit(HTTPRequest)]
pub struct RustubeNode;



#[methods]
impl RustubeNode {
    fn new(_owner: TRef<HTTPRequest>) -> Self {
        RustubeNode
    }

    #[export]
    fn _download_video(&self, #[base] owner: TRef<HTTPRequest>, url : String) -> PoolArray<T>{
    //godot_print!("downloaded video to {:?}", );
    
       async {
        rustube::download_best_quality(&url).await.unwrap()
       } 

    }


}

impl NativeClassMethods  for RustubeNode{

    fn nativeclass_register(_: &ClassBuilder<Self>) { todo!() }
}


struct HttpRequestRef(Ref<HTTPRequest>);

//impl<'a> OwnerArg<'a, HTTPRequest, Shared> for HttpRequestRef {}

fn init(handle: InitHandle) {
    handle.add_class::<RustubeNode>();
}

godot_init!(init);
