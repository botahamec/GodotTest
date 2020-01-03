#![allow(non_snake_case)]

#[macro_use]
extern crate gdnative;

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Label)]
#[user_data(gdnative::user_data::MutexData<Fibonacci>)]
struct Fibonacci {
	number: u32,
	total: f64
}

fn fib(num: u32) -> u32 {
	match num {
		0 | 1 => num,
		_ => fib(num - 1) + fib(num - 2)
	}
}

#[gdnative::methods]
impl Fibonacci {

    fn _init(_owner: gdnative::Label) -> Self {
		Fibonacci {
			number: 0,
			total: 0.0
		}
	}

	#[export]
    unsafe fn _process(&mut self, mut owner: gdnative::Label, delta: f64) {

		if self.number > 50 {return;} // does nothing
		if self.number > 25 {self.total += delta;}
		self.number += 1;

		if self.number == 50 {
			owner.set_text(gdnative::GodotString::from(format!("Rust: {}secs", self.total)));
			self.number += 1;
		}
		else {
			owner.set_text(gdnative::GodotString::from(format!("{}", fib(self.number - 25))));
			godot_print!("{}", self.number - 25);
		}
	}
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Fibonacci>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();