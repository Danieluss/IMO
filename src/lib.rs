macro_rules! def_str_consts {
    ($($name:ident),*) => (
        $(const $name: &str = stringify!($name);)*
    )
}

pub mod traits;

pub mod tsp {
    pub mod def;
    pub mod solver;
    pub mod random_solver;
    pub mod partial_path;
    pub mod picker;
    pub mod solvers_factory;
    pub mod pickers {
        pub mod nearest_picker;
        pub mod cycle_picker;
        pub mod cycle_simultaneous_picker;
        pub mod regret_picker;
    }
}

pub mod utils;

