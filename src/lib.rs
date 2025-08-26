pub mod os_backend;

pub mod blocking {
    pub mod methods {
        pub mod duration;
    }
    pub mod websites {
        pub mod block_websites;
    }
}

pub mod config {}

pub mod cli {
    pub mod cli_runner;
    pub mod set_block_for_time_and_task;
    pub mod commands {
        pub mod reset;
        pub mod setup;
    }
}
