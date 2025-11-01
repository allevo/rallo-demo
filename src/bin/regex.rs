use rallo::RalloAllocator;

// This is the maximum length of a frame
const MAX_FRAME_LENGTH: usize = 128;
// Maximum number of allocations to keep
const MAX_LOG_COUNT: usize = 1_024 * 10;
#[global_allocator]
static ALLOCATOR: RalloAllocator<MAX_FRAME_LENGTH, MAX_LOG_COUNT> = RalloAllocator::new();

fn run_regex() {
    let re = regex::Regex::new(r"R(\w)").unwrap();
    let text = "Hello World\nThis is a test\nRallo Allocator";

    let _ = re.captures(text);
}

fn main() {
    // Safety: the program is single-threaded
    unsafe { ALLOCATOR.start_track() };

    run_regex();

    ALLOCATOR.stop_track();

    // Safety: it is called after `stop_track`
    let stats = unsafe { ALLOCATOR.calculate_stats() };
    let tree = stats.into_tree().unwrap();

    let this_file = file!();
    let file_name = std::path::Path::new(this_file)
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.replace(".rs", ".html"))
        .unwrap();
    let path = std::env::current_dir().unwrap().join(file_name);
    tree.print_flamegraph(&path);

    println!("Flamegraph saved to {}", path.display());
}
