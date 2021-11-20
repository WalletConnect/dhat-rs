#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
#[should_panic(expected = "dhat: getting stats after profiling has finished")]
fn main() {
    {
        let _dhat = dhat::start_heap_profiling();

        let _v = vec![1u32, 2, 3, 4];
    }

    let _stats = dhat::get_heap_stats(); // panic
}
