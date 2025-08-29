use std::time::Duration;

struct Block {
    block_list: SoftwareList,
    condition_list: BlockConditions,
    restriction_list: Restrictions,
    enabled: bool,
    active: bool,
}

struct SoftwareList {
    website_list: String,
    application_list: String,
}

struct BlockConditions {
    duration: Duration,
    schedule: String,  // Implement with Jiff later
    locations: String, // Implement with geo or some other location crate
    // etc. (?)
}

enum Restrictions {
    None,
    Password(String),
    Location(String) // Implement with geo later
    // etc.
}
