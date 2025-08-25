pub mod os_backend;

pub mod blocking {
    pub mod methods {
        pub mod block_duration;
    }
    pub mod ui {
        pub mod spinners;
    }
    pub mod config {
        pub mod config;
    }
    pub mod websites {
        pub mod hosts_specific_implementation {
            pub mod hosts;
        }
        pub mod block_websites;
    }
}

pub mod config {}

pub mod cli {
    pub mod cli_utils;
    pub mod cli_runner;
    pub mod set_block_for_time_and_task;
    pub mod commands {
        pub mod reset;
        pub mod start;
        pub mod setup;
    }
}
