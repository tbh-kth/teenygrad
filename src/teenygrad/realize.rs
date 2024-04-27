pub fn run_schedule(schedule: Vec<i32>, disable_logging: bool) -> std::io::Result<()> {
    // not sure about schedule type, seems to be its own class in tinygrad. The rest seems fine
    if disable_logging {
        return Err("Disable_logging is true");
    }
    return Ok();
}

pub fn create_schedule(outs: Vec<i32>, seen: Option<Vec<i32>>) -> Vec<i32> {
    // Check if the types are correct, especially the output
    return vec![];
}
