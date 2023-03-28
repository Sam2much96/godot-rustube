// Rust GDNative implementation of this Godot tutorial:
// https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html#a-custom-node

use rustube;
use gdnative::prelude::*;
use gdnative::api::HTTPRequest;

#[derive(NativeClass)]
#[inherit(HTTPRequest)]
pub struct RustubeNode;

#[methods]
impl RustubeNode {
    fn new(_owner: Node) -> Self {
        RustubeNode
    }

    #[export]
    async fn &_download_video(&self, #[base] owner: HTTPRequest, url : String) -> PoolArray<T>{
    //godot_print!("downloaded video to {:?}", );
    rustube::download_best_quality(&url).await.unwrap()

    }

}


fn init(handle: InitHandle) {
    handle.add_class::<RustubeNode>();
}

godot_init!(init);
