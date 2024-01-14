/*
# TODO
thing_referencing is a vec without uniqe names. Should this be a thing? Firwall rules via index? -> Return Both index and name, backreferences are only shown to help users (update ref name might be a backend use but that can just always us the index)
validation function (ref exists, no duplicate names, custom functions), register globally/ on config?
add function to change name of referenced and update all references

# Missing link types
link_opt -> link from option
link_enum_opt -> link from option in an enum
*/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReferencedBy {
    pub name: String,
    pub path: String,
}

// Main Macro
#[macro_export]
macro_rules! macro_db {
    (
        // all links
        $(
            {
                // link sources
                $(
                    [ $link_type:tt :
                    $field_name:ident,
                    $thing_referencing:ty,
                    $( $path_referenced:ident ).+;
                    $( $path_referencing:ident ).+
                    ( $($arg:tt)*  )
                    ],
                )+ ->
                // link destination
                $thing_referenced:ty
            },
        )+
    ) => {
        use crate::definitions::macro_db::ReferencedBy;
        use crate::macro_db_link;
        // Loop here to impl Relation
        $(
            $(
                macro_db_link!($link_type, $field_name, $thing_referencing, $thing_referenced, $($path_referenced).+ ( $($arg)* ));
            )+
        )+

        // Validation here
        impl Config {
            #[allow(dead_code)]
            // TODO make proper error
            fn validate_relations() -> Result<(), String> {
                // Loop for each

                Ok(())
            }
        }

        // Loop here for back reference, grouped by destination
        $(
            impl $thing_referenced {
                #[allow(dead_code)]
                pub fn referenced_by(&self, config: Config) -> Vec<ReferencedBy> {
                    let mut by = Vec::<ReferencedBy>::new();
                    $(
                        macro_rules! macro_db_back_link {
                            (S ()) => {
                                config.$($path_referencing).+.iter().filter(|e| *e.$field_name == self.name).for_each(|e| by.push(ReferencedBy{
                                    name: e.name.clone(),
                                    path: stringify!(config.$($path_referencing).+).to_string(),
                                }));
                            };
                            (M ()) => {
                                config.$($path_referencing).+.iter().filter(|e| e.$field_name.contains(&self.name)).for_each(|e| by.push(ReferencedBy{
                                    name: e.name.clone(),
                                    path: stringify!(config.$($path_referencing).+).to_string(),
                                }));
                            };
                            (E
                                ($enum_name:ident,
                                $enum_type:ident,
                                $enum_variant:ident,
                                $fn_name:ident )
                            ) => {
                                /* Temporarly Commented since this crashes rust-analyzer but compiles fine...
                                for e in config.$($path_referencing).+.clone() {
                                    if let $enum_type::$enum_variant { $field_name, .. } = e.$enum_name {
                                        if &self.name == &$field_name {
                                            by.push(ReferencedBy{
                                                name: e.name.clone(),
                                                path: stringify!(config.$($path_referencing).+).to_string(),
                                            });
                                        }
                                    }
                                }
                                */
                            };
                            (EM
                                ($enum_name:ident,
                                $enum_type:ident,
                                $enum_variant:ident,
                                $fn_name:ident )
                            ) => {
                                /* Temporarly Commented since this crashes rust-analyzer but compiles fine...
                                for e in config.$($path_referencing).+.clone() {
                                    if let $enum_type::$enum_variant { $field_name, .. } = e.$enum_name {
                                        for reference in &$field_name {
                                            if &self.name == reference {
                                                by.push(ReferencedBy{
                                                    name: e.name.clone(),
                                                    path: stringify!(config.$($path_referencing).+).to_string(),
                                                });
                                            }
                                        }
                                    }

                                }
                                */
                            };
                        }

                        macro_db_back_link!($link_type ($($arg)*));
                    )+
                    return by
                }
            }
        )+

    };
}

#[macro_export]
macro_rules! macro_db_link {
    (   S,
        $field_name:ident,
        $thing_referencing:ty,
        $thing_referenced:ty,
        $( $path_referenced:ident ).+
        ()
    ) => {
        impl $thing_referencing {
            #[allow(dead_code)]
            pub fn $field_name(&self, config: Config) -> $thing_referenced {

                let index = config.$($path_referenced).+.iter().position(|e| *e.name == self.$field_name);

                match index {
                    Some(i) => config.$($path_referenced).+[i].clone(),
                    // This is fine since the config always has to validated before commiting
                    None => panic!("Referenced Thing: '{:?}' does not exist ", self.$field_name),
                }

            }
        }
    };
    (   M,
        $field_name:ident,
        $thing_referencing:ty,
        $thing_referenced:ty,
        $( $path_referenced:ident ).+
        ()
    ) => {
        impl $thing_referencing {
            #[allow(dead_code)]
            pub fn $field_name(&self, config: Config) -> Vec<$thing_referenced> {
                let mut res = Vec::<$thing_referenced>::new();

                for reference in self.$field_name.clone() {
                    for referenced in config.$($path_referenced).+.clone() {
                        if reference == referenced.name {
                            res.push(referenced);
                        }
                    }
                }
                return res
            }
        }
    };
    (   E,
        $field_name:ident,
        $thing_referencing:ty,
        $thing_referenced:ty,
        $( $path_referenced:ident ).+
        ($enum_name:ident,
        $enum_type:ident,
        $enum_variant:ident,
        $fn_name:ident )
    ) => {
        // Unfortunetly Enum Variants are not Types, which is why we can't impl on the Variant and need seperate function names (since multiple variant could have the same field name)
        impl $enum_type {
            #[allow(dead_code)]
            pub fn $fn_name(&self, config: Config) -> $thing_referenced {
                let index = config.$($path_referenced).+.iter().position(
                    |e| {
                        if let $enum_type::$enum_variant { $field_name, .. } = self.clone() {
                            return *e.name == $field_name;
                        }
                        return false;
                    }
                );

                match index {
                    Some(i) => config.$($path_referenced).+[i].clone(),
                    // This is fine since the config always has to validated before commiting
                    None => panic!("Referenced Thing: does not exist (from Enum)"),
                }

            }
        }
    };
    (   EM,
        $field_name:ident,
        $thing_referencing:ty,
        $thing_referenced:ty,
        $( $path_referenced:ident ).+
        ($enum_name:ident,
        $enum_type:ident,
        $enum_variant:ident,
        $fn_name:ident )
    ) => {
        // Unfortunetly Enum Variants are not Types, which is why we can't impl on the Variant and need seperate function names (since multiple variant could have the same field name)
        impl $enum_type {
            #[allow(dead_code)]
            pub fn $fn_name(&self, config: Config) -> Vec<$thing_referenced> {
                let mut res = Vec::<$thing_referenced>::new();

                if let $enum_type::$enum_variant { $field_name, .. } = self {
                    for reference in $field_name.clone() {
                        for referenced in config.$($path_referenced).+.clone() {
                            if reference == referenced.name {
                                res.push(referenced);
                            }
                        }
                    }
                }
                return res
            }
        }
    };
}
