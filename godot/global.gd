extends Node3D

func _input(event):
	if event.is_action_pressed("Quit"):
		get_tree().root.propagate_notification(NOTIFICATION_WM_CLOSE_REQUEST);
		get_tree().quit();
	if event.is_action_pressed("ui_page_up"):
		$TopView.make_current();
	if event.is_action_pressed("ui_page_down"):
		$Probe/Camera.make_current();

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	pass
