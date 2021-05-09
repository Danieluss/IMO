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
    pub mod solvers_factory;
    pub mod partial_path;
    pub mod picker;
    pub mod pickers {
        pub mod nearest_picker;
        pub mod cycle_picker;
        pub mod cycle_simultaneous_picker;
        pub mod regret_picker;
    }
    pub mod neighborhoods {
        pub mod neighborhood;
        pub mod transition;
        pub mod inter_cycle_transition;
        pub mod edges_transition;
        pub mod vertex_transition;
    }
    pub mod local_solvers;
    pub mod candidate_solver;
    pub mod memory_solver;
    pub mod multistart_solver;
    pub mod iterated_solver;
}
pub mod primes {
    pub mod primes;
}

pub mod utils;


