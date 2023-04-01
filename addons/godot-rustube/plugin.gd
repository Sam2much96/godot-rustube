tool
extends EditorPlugin


var _editor_view

#func _init():
	#add_autoload_singleton('DocsHelper', "res://addons/algodot/Documentation/Scripts/DocsHelper.gd")
	#*********For Built in Documentation**********#
	
	#_add_custom_editor_view()
	
	#"Adds a Custom Tab for Documentations"
	#get_editor_interface().get_editor_viewport().add_child(_editor_view)
	#make_visible(false)

func _enter_tree():

	var gui = get_editor_interface().get_base_control()
	var node_icon = gui.get_icon("Node", "EditorIcons")

	add_custom_type(
		"RustubeNode",
		"Node",
		preload("res://addons/godot-rustube/custom_node.gdns"),
		node_icon
	)
	#add_autoload_singleton("AsyncExecutorDriver", "res://addons/algodot/gdnative/async_executor.gdns")




#func _exit_tree():
	
	#remove_custom_type("Algod")
	#remove_autoload_singleton("AsyncExecutorDriver") #causes leaked memory bug

#************For Builtin Documentation***********#
	


#***********For Builtin Documentation***********#
func get_plugin_name()-> String:
	return "RustubeNode"



