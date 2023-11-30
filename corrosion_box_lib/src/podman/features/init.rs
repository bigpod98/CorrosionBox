pub fn check_feature(flag_init_system: String) -> String
{
    if flag_init_system == "true"
    {
        return "--init".to_string();
    }
    return "".to_string();
}