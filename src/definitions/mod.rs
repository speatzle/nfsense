use self::config::Config;

pub mod config;
pub mod firewall;
pub mod network;
pub mod object;
pub mod service;
pub mod system;
pub mod vpn;

pub trait Referenceable<T> {
    fn named_get(&self, name: String) -> T;
    fn named_exists(&self, name: String) -> bool;
}

#[macro_export]
macro_rules! impl_referenceable_trait {
    ($typ:ty, $ele:ty) => {
        impl Referenceable<$ele> for $typ {
            fn named_get(&self, name: String) -> $ele {
                let index = self.iter().position(|e| *e.name == name);

                match index {
                    Some(i) => self[i].clone(),
                    // This is fine since the config always has to validated before commiting
                    None => panic!("Referenced Thing: '{:?}' does not exist ", name),
                }
            }

            fn named_exists(&self, name: String) -> bool {
                let index = self.iter().position(|e| *e.name == name);
                index.is_some()
            }
        }
    };
}

pub trait References<T> {
    fn get_ref(&self, config: Config) -> T;
    fn ref_exists(&self, config: Config) -> bool;
}

#[macro_export]
macro_rules! impl_references_trait {
    ($thing:ty, $referenced:ty, $( $path:ident ).+) => {
        impl References<$referenced> for $thing {
            fn get_ref(&self, config: Config) -> $referenced {
                config.$($path).+.named_get(self.clone())
            }

            fn ref_exists(&self, config: Config) -> bool {
                config.$($path).+.named_exists(self.clone())
            }
        }
    };
}
