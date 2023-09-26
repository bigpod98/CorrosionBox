use std::{process::{Command, Stdio}, io::Read};
use corrosion_box_lib;

fn main() {
	corrosion_box_lib::hello_world();
	//corrosion_box_lib::features::home::home_world();
	let mut cmd = Command::new("echo");
	cmd.arg("hello World from commandline");
	cmd.stdout(Stdio::piped());
	

	let mut child = cmd.spawn().expect("failed to execute child");

	let mut stdout = String::new();
	child.stdout.as_mut().unwrap().read_to_string(&mut stdout).expect("failed to read stdout");

	corrosion_box_lib::output(stdout);
}