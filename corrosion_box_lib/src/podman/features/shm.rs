pub fn check_feature(flag_shm: String, ) -> String
{
    if  flag_shm == "true"
    {
        return "-v /dev/shm:/dev/shm:rslave".to_string();
    }
    else {
        return "".to_string();
    }  
}