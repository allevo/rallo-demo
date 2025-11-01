use rallo::RalloAllocator;
use sqlx::postgres::PgPoolOptions;

// This is the maximum length of a frame
const MAX_FRAME_LENGTH: usize = 128;
// Maximum number of allocations to keep
const MAX_LOG_COUNT: usize = 1_024 * 10;
#[global_allocator]
static ALLOCATOR: RalloAllocator<MAX_FRAME_LENGTH, MAX_LOG_COUNT> = RalloAllocator::new();

/// Run before:
/// ```text
/// docker run --name some-postgres \
///     -e POSTGRES_PASSWORD=mysecretpassword \
///     -p5432:5432 -d \
///     postgres:17`
async fn run_sqlx() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:mysecretpassword@localhost/")
        .await
        .unwrap();

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.0, 150);
}

#[tokio::main]
async fn main() {
    // Safety: the program is single-threaded
    unsafe { ALLOCATOR.start_track() };

    run_sqlx().await;

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
