// Rust GDNative implementation of this Godot tutorial:
// https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html#a-custom-node

use rustube;
use gdnative::export::OwnerArg;
use gdnative::prelude::*;
use gdnative::api::HTTPRequest;

#[derive(NativeClass)]
#[inherit(HTTPRequest)]
pub struct RustubeNode;

#[methods]
impl RustubeNode {
    fn new(_owner: Ref<HTTPRequest>) -> Self {
        RustubeNode
    }

    #[export]
    async fn &_download_video(&self, #[base] owner: Ref<HTTPRequest>, url : String) -> PoolArray<T>{
    //godot_print!("downloaded video to {:?}", );
    rustube::download_best_quality(&url).await.unwrap()

    }

}

/* Traits */
impl NativeClassMethods  for RustubeNode{

    fn nativeclass_register(_: &ClassBuilder<Self>) { todo!() }
}

impl  OwnerArg<'_, HTTPRequest, Shared> for gdnative::prelude::Ref<HTTPRequest>{

}

fn init(handle: InitHandle) {
    handle.add_class::<RustubeNode>();
}

godot_init!(init);
