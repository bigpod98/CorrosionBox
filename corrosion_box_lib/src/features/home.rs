pub fn check_feature(flag_home: String, flag_custom_home: String, flag_custom_home_path: String) -> String
{
    if  flag_custom_home == "true"
    {
        if flag_custom_home_path .len() > 0
        {
            return "Custom_home".to_string();
        }
        else {
            if flag_home == "true" {
                return "User_home".to_string();
            }
            else {
                return "no_home".to_string();
            }
        }
    }
    else {
        if flag_home == "true" {
            return "User_home".to_string();
        }
        else {
            return "no_home".to_string();
        }
    }  
}

pub fn home_mount() -> String
{
    let userhomedir = "";

    let mut selinux = "";
    let selinux_check=true;
    if selinux_check == true
    {
        selinux = ":Z";
    }

    return format!("-v {}:{}{}", userhomedir, userhomedir, selinux).to_string();
}

pub fn custom_home_mount(path: String, username: String) -> String
{
    let mut selinux = "";
    let selinux_check=true;
    if selinux_check == true
    {
        selinux = ":Z";
    }

    return format!("-v {}:/home/{}{}", path, username, selinux).to_string();
}