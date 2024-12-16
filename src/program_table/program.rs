


#[derive( serde::Serialize, serde::Deserialize)]
pub struct Program{
    name: String,
    localPort: i32,
    hostPort: i32,
    hostIp: String
}
