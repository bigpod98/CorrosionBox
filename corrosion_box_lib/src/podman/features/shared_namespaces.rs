pub fn ipc_namespace(flag: String) -> String 
{
    if flag=="false"
    {
        return "--ipc host".to_string();
    }
    else{
        return "".to_string();
    }

}

pub fn pid_namespace(flag: String) -> String 
{
    if flag=="false"
    {
        return "--pid host".to_string();
    }
    else{
        return "".to_string();
    }

}

pub fn net_namespace(flag: String) -> String 
{
    if flag=="false"
    {
        return "--network host".to_string();
    }
    else{
        return "".to_string();
    }

}