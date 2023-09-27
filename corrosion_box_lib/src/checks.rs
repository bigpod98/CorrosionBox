use std::process::Command;

pub fn selinux_check() -> bool {
    
    let mut command = Command::new("my_exe");

    let output = command.output();
    if output.is_ok() {

        let output_string = String::from_utf8_lossy(&output.unwrap().stdout).to_string(); 

        if output_string == "Disabled"{
            return false;        

        }else{
            return true;
        }

    } else {
        return false;
    }

}