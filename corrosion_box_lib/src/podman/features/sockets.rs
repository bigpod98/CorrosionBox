use libc;

fn getuid() -> String {

    let uid = unsafe { libc::getuid() };
    let uidstr = uid.to_string();
    println!("Current user ID: {}", uidstr);

    return format!("{}", uidstr).to_string();
}

pub fn podman_user_socket(flag_pass_podman: String) -> String
{
    let uid = getuid();

    if flag_pass_podman == "true"
    {
        return format!("-v /run/user/{}/podman/podman.sock:/run/user/{}/podman/podman.sock", uid, uid).to_string();
    }
    return "".to_string();
}

pub fn podman_system_socket(flag_pass_system_podman: String) -> String
{
    if flag_pass_system_podman == "true"
    {
        return format!("-v /run/podman/podman.sock:/run/podman/podman.sock").to_string();
    }
    return "".to_string();
}

pub fn wayland_socket(flag_pass_wayland: String) -> String
{
    let uid = getuid();
    if flag_pass_wayland == "true"
    {
        return format!("-v /run/user/{}/wayland-0:/run/user/{}/wayland-0", uid, uid).to_string();

    }
    return "".to_string();
}

pub fn pipewire_socket(flag_pass_pipewire: String) -> String
{
    let uid = getuid();
    if flag_pass_pipewire == "true"
    {
        return format!("-v /run/user/{}/pipewire-0:/run/user/{}/pipewire-0", uid, uid).to_string();

    }
    return "".to_string();
}

pub fn libvirt_system_socket(flag_pass_libvirt_system: String) -> String
{
    if flag_pass_libvirt_system == "true"
    {
        return format!("-v /run/libvirt:/run/libvirt").to_string();

    }
    return "".to_string();
}