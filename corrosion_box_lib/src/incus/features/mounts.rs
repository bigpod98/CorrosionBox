use std::fs;

pub fn mount_root(flag_root_mount: String) -> String
{
    if flag_root_mount == "true"
    {
        // let mut selinux = "";
        // let selinux_check=crate::checks::selinux_check();
        // if selinux_check == false
        // {
        //     selinux = ":Z";
        // }

        return format!("-v /:/host_rootfs").to_string();
    }
    else {
        return "".to_string();
    }
}

pub fn systemmounts(flag_disable_system_mounts: String) -> String
{
        // let mut selinux = "";
        // let selinux_check=crate::checks::selinux_check();
        // if selinux_check == false
        // {
        //     selinux = ":Z";
        // }

    if flag_disable_system_mounts == "false" {
        
        let paths = fs::read_dir("/dev").unwrap();

        let mut lines = Vec::new();

        for path in paths {
            let pat = path.unwrap().path().display().to_string();
            println!("Name: {}", pat);
            lines.push(format!("{}", pat));
        }


        let mut out=String::new();
        for line in lines {
            if line != "shm" {
                out = format!("{} -v {}:{}",out , line, line)
            }
        }

        return format!("-v /tmp:/tmp:rslave -v /sys:/sys:rslave {}", out).to_string();
    }
    return "".to_string();
}