#![allow(non_snake_case)]

#[macro_use]
extern crate gdnative;

use gdnative::Vector2;

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::KinematicBody2D)]
#[user_data(gdnative::user_data::MutexData<Character>)]
struct Character {
    acceleration : Vector2,
    velocity : Vector2
}

#[gdnative::methods]
impl Character {

    fn _init(_owner: gdnative::KinematicBody2D) -> Self {
		Character {
			acceleration: Vector2::new(0.0, 0.0),
			velocity: Vector2::new(0.0, 0.0)
		}
	}

	#[export]
    unsafe fn _ready(&mut self, mut owner: gdnative::KinematicBody2D) {
		owner.set_physics_process(true);
	}

	#[export]
	unsafe fn _physics_process(&mut self, mut owner: gdnative::KinematicBody2D, delta: f64) {

		if delta == 0.0 {return;} // prevents division by zero

		// calculate gravity
		let gravity: f32 = (9.80665 / (32.0 * delta)) as f32;
		self.acceleration = Vector2::new(0.0, gravity);

		// takes input and moves the character
		let key_left = gdnative::GodotString::from_str("ui_left");
		let key_right = gdnative::GodotString::from_str("ui_right");
		if gdnative::Input::godot_singleton().is_action_pressed(key_left) {
			self.acceleration += Vector2::new(-3.0, 0.0);
		}
		else if gdnative::Input::godot_singleton().is_action_pressed(key_right) {
			self.acceleration += Vector2::new(3.0, 0.0);
		}

		else {self.velocity.x /= 1.1;} // adds friction

		// moves the character
		self.velocity += self.acceleration;
		self.velocity = owner.move_and_slide(self.velocity, Vector2::new(0.0, 1.0), false, 4, 0.785398, true);
	}
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Character>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();