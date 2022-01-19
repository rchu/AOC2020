macro_rules! mod_use { ($name:ident) => { 
    mod $name;
    pub use self::$name::$name;
}}

mod_use!(day01);