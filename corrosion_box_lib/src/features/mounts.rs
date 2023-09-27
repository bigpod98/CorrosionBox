
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
        return "-v /tmp:/tmp:rslave -v /sys:/sys:rslave -v /dev:/dev:rslave".to_string();
    }
    return "".to_string();
}