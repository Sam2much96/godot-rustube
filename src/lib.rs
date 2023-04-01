use rustube;
//use gdnative::object::TRef;
use gdnative::prelude::*;
//use gdnative::api::HTTPRequest;


#[allow(non_camel_case_types)]
#[derive(NativeClass)]
#[inherit(Node)]
pub struct RustubeNode;

pub trait Method<RustubeNode>{}
#[methods]
impl RustubeNode {
    fn new(_owner: &Node) -> Self {
        godot_print!("Initializing Rustube");
        RustubeNode{}
    }

    
    #[method]
    async fn main() {
    let url = "https://youtube.com/shorts/YCwou4oX12I?feature=share";
    godot_print!("downloaded video to {:?}", rustube::download_best_quality(&url).await.unwrap());
    }

    /*
    fn register(builder: &ClassBuilder<Self>) {
        /* Registers the Method*/
        builder.method(stringify(Self::_download_video), Self::_download_video_async).done();
    }

    
    async fn _download_video_async(
        &self,
        owner: TRef<'_, HTTPRequest>,
        args: Varargs<'_>,
    ) -> Variant {
        let url = args.get_string(0).unwrap();
        let result = self._download_video(owner, url).await;
        Variant::from_str(&result)
    }

    */
    
}

fn init(handle: InitHandle) {
    handle.add_class::<RustubeNode>();
}

godot_init!(init);
