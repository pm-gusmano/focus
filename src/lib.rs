pub mod os_backend;

pub mod blocking {
    pub mod config {
        pub mod config;
    }

    pub mod websites;
}

pub mod ui {
    pub mod blocking_message;
    pub mod spinners;
}

pub mod config {}

pub mod cli {
    pub mod cli_runner;
    pub mod cli_utils;
    pub mod commands {
        pub mod reset;
        pub mod setup;
        pub mod start;
    }
}
