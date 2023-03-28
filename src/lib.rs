// Rust GDNative implementation of this Godot tutorial:
// https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html#a-custom-node

use rustube;
use gdnative::api::{EditorPlugin, Resource, Script, Texture};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(EditorPlugin)]
struct CustomNode;

#[methods]
impl CustomNode {
    fn new(_owner: TRef<EditorPlugin>) -> Self {
        CustomNode
    }

    #[export]
    fn _enter_tree(&self, #[base] owner: TRef<EditorPlugin>) {
        // Initialization of the plugin goes here.
        // Add the new type with a name, a parent type, a script and an icon.
        let script = unsafe { load::<Script>("res://my_button.gdns", "Script").unwrap() };
        let texture = unsafe {
            load::<Texture>("res://making_plugins-custom_node_icon.png", "Texture").unwrap()
        };
        owner.add_custom_type("MyButton", "Button", script, texture);
    }

    #[export]
    fn _exit_tree(&self, #[base] owner: TRef<EditorPlugin>) {
        // Clean-up of the plugin goes here.
        // Always remember to remove it from the engine when deactivated.
        owner.remove_custom_type("MyButton");
    }
}

#[derive(NativeClass)]
#[inherit(Button)]
struct MyButton;

#[methods]
impl MyButton {
    fn new(_owner: TRef<Button>) -> Self {
        MyButton
    }

    #[export]
    fn _enter_tree(&self, #[base] owner: TRef<Button>) {
        owner
            .connect("pressed", owner, "clicked", VariantArray::new_shared(), 0)
            .unwrap();
    }

    #[export]
    fn clicked(&self, #[base] owner: TRef<Button>) {
        godot_print!("You clicked me!");
    }
}

unsafe fn load<T>(path: &str, hint: &str) -> Option<Ref<T, Shared>>
where
    T: GodotObject<Memory = RefCounted> + SubClass<Resource>,
{
    let resource = ResourceLoader::godot_singleton().load(path, hint, false)?;
    let resource = resource.assume_safe().claim();
    resource.cast::<T>()
}

fn init(handle: InitHandle) {
    handle.add_tool_class::<CustomNode>();
    handle.add_tool_class::<MyButton>();
}

godot_init!(init);
