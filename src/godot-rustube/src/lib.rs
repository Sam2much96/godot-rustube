use gdnative::prelude::*;
//use gdnative::tasks;
use rustube::{Id, VideoFetcher};

#[allow(non_camel_case_types)]
#[derive(NativeClass)]
#[inherit(Node)]
pub struct RustubeNode;

//pub trait Method<RustubeNode> {}
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

    /*

    #[method]
    async fn main() {
        let mut url  : String = "https://youtube.com/shorts/YCwou4oX12I?feature=share";
        godot_print!(
            "downloaded video to {:?}",
            rustube::download_best_quality(&url).await.unwrap()
        );
    }

    #[method]
    async fn test() {
        let mut id = Id::from_raw("https://youtube.com/shorts/YCwou4oX12I?feature=share").unwrap();
        let mut descrambler = VideoFetcher::from_id(id.into_owned())
            .unwrap()
            .fetch()
            .await
            .unwrap();

        let mut view_count = descrambler.video_details().view_count;
        let mut title = descrambler.video_title();
        println!("The video `{}` was viewed {} times.", title, view_count);
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
    */
}
/* No Valid Init Library */

#[gdnative::init::callbacks]
impl GDNativeCallbacks for RustubeNode {
     fn nativescript_init(handle: InitHandle) {
        handle.add_class::<RustubeNode>();
    }

}
//godot_init!(init);
