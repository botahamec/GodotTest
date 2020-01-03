extends KinematicBody2D

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var accel: Vector2
var veloc: Vector2

# Called when the node enters the scene tree for the first time.
func _ready():
	accel = Vector2(0, 0)
	veloc = Vector2(0, 0)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _physics_process(delta):
	if delta == 0: return
	var grav = 9.80665 / (32 * delta)
	accel = Vector2(0, grav)
	if Input.is_key_pressed(KEY_LEFT):
		accel += Vector2(3, 0)
	elif Input.is_key_pressed(KEY_RIGHT):
		accel += Vector2(-3, 0)
	else:
		veloc.x /= 1.1
	veloc += accel
	veloc = move_and_slide(veloc, Vector2(0, 1))