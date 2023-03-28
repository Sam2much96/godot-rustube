// Rust GDNative implementation of this Godot tutorial:
// https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html#a-custom-node

use rustube;
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

    #[method]
    async fn _download_video(&self, #[base] owner: TRef<'_, HTTPRequest>, url : String) -> PoolArray<T>{
    //godot_print!("downloaded video to {:?}", );
    
       
        rustube::download_best_quality(&url).await.unwrap()
        

    }

    /* Manually Register the Methods*/
    fn register(builder: &ClassBuilder<Self>) {
    builder.add_signal(Signal {
        name: "_download_video_completed",
        args: &[SignalArgument {
            name: "video_path",
            default: Variant::new(),
            export_info: ExportInfo::new(VariantType::String),
        }],
    });
  }
}



struct HttpRequestRef(Ref<HTTPRequest>);

//impl<'a> OwnerArg<'a, HTTPRequest, Shared> for HttpRequestRef {}

fn init(handle: InitHandle) {
    handle.add_class::<RustubeNode>();
}

godot_init!(init);
