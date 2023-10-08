pub fn check_feature(flag_home: String, flag_custom_home: String, flag_custom_home_path: String, ) -> String
{
    if  flag_custom_home == "true"
    {
        if flag_custom_home_path .len() > 0
        {
            return custom_home_mount(flag_custom_home_path);
        }
        else {
            if flag_home == "true" {
                return home_mount();
            }
            else {
                return "".to_string();
            }
        }
    }
    else {
        if flag_home == "true" {
            return home_mount();
        }
        else {
            return "".to_string();
        }
    }  
}

pub fn home_mount() -> String
{
    let userhomedir = std::env::var("$HOME").unwrap().to_string();

    // let mut selinux = "";
    // let selinux_check=crate::checks::selinux_check();
    // if selinux_check == true
    // {
    //     selinux = ":Z";
    // }

    return format!("-v {}:{} -e HOME=\"{}\"", userhomedir, userhomedir, userhomedir).to_string();
}

pub fn custom_home_mount(path: String) -> String
{
    let username = std::env::var("$HOME").unwrap().to_string();

    // let mut selinux = "";
    // let selinux_check= crate::checks::selinux_check();
    // if selinux_check == true
    // {
    //     selinux = ":Z";
    // }

    return format!("-v {}:/home/{} -e HOME=\"/home/{}\"", path, username, username).to_string();
}