macro_rules! def_str_consts {
    ($($name:ident),*) => (
        $(const $name: &str = stringify!($name);)*
    )
}

pub mod traits;

pub mod tsp {
    pub mod def;
    pub mod solver;
}

mod utils;

