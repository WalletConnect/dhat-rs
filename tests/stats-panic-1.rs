#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
#[should_panic(expected = "dhat: getting stats before profiling has begun")]
fn stats_panic_1() {
    let _stats = dhat::get_heap_stats(); // panic
}
