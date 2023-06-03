use honggfuzz::fuzz;
use rppal::pwm::sysfs::user_to_uid;

fn main() {
    // Define the fuzzing loop
    loop {
        // Fuzzing input will be provided by Honggfuzz
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(name) = std::str::from_utf8(data) {
                // Call the user_to_uid function with the fuzzed input
                let _ = group_to_gid(name);

            }
        });
    }
}
