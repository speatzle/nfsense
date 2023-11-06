pub mod config;
pub mod firewall;
pub mod network;
pub mod object;
pub mod service;
pub mod system;
pub mod vpn;

#[macro_export]
macro_rules! get_thing {
    ($out:ty, $n:ident) => {
        pub fn $n(list: Vec<$out>, name: String) -> Option<$out> {
            for e in list {
                if e.name == name {
                    return Some(e);
                }
            }
            None
        }
    };
}
