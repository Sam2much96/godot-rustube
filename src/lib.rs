use rustube;
use gdnative::tasks::Async;
use gdnative::export::Varargs;
use gdnative::object::TRef;
use gdnative::prelude::*;
use gdnative::api::HTTPRequest;

#[derive(NativeClass)]
#[register_with(Self::register)]
#[inherit(HTTPRequest)]
pub struct RustubeNode;

#[methods]
impl RustubeNode {
    fn new(_owner: TRef<HTTPRequest>) -> Self {
        RustubeNode
    }

    #[export]
    #[method]
    fn _download_video(&self, #[base] owner: TRef<'_, HTTPRequest>, url: String) -> Async<String> {
        async move {
            rustube::download_best_quality(&url)
                .await
                .unwrap()
                .to_str()
                .expect("Path to VideoFIle")
                .to_string()
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder.method("_download_video", Self::_download_video_async).done();
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
}

fn init(handle: InitHandle) {
    handle.add_class::<RustubeNode>();
}

godot_init!(init);
