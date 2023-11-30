pub mod podman;
pub mod incus;
pub mod checks;
pub mod selinux;

pub fn hello_world() {
    println!("Hello, world!");
}

pub fn output(test: String) {
    print!("{}", test);
}

