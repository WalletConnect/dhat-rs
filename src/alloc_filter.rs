use tikv_jemalloc_sys::malloc_usable_size;

/// Allocation filter which allows recording data only for specific
/// allocations, lowering CPU load and reducing profile noise.
pub trait AllocFilter {
    /// Given a pointer to the allocated memory, returns `true` if
    /// the allocations needs to be recorded by the profiler.
    fn should_record(&self, ptr: *const u8) -> bool;
}

/// Filter based on Jemalloc allocation bin size.
#[derive(Debug)]
pub struct JemallocMultiBinFilter<const NUM_BINS: usize> {
    bin_sizes: [libc::size_t; NUM_BINS],
}

impl<const NUM_BINS: usize> JemallocMultiBinFilter<NUM_BINS> {
    /// Constructs the filter.
    pub const fn new(bin_sizes: [libc::size_t; NUM_BINS]) -> Self {
        Self { bin_sizes }
    }
}

impl<const NUM_BINS: usize> AllocFilter for JemallocMultiBinFilter<NUM_BINS> {
    #[inline]
    fn should_record(&self, ptr: *const u8) -> bool {
        let usable_size = unsafe { malloc_usable_size(ptr as *const _) };

        self.bin_sizes.contains(&usable_size)
    }
}

/// Filter based on Jemalloc allocation bin size.
#[derive(Debug)]
pub struct JemallocSingleBinFilter {
    bin_size: libc::size_t,
}

impl JemallocSingleBinFilter {
    /// Constructs the filter.
    pub const fn new(bin_size: libc::size_t) -> Self {
        Self { bin_size }
    }
}

impl AllocFilter for JemallocSingleBinFilter {
    #[inline]
    fn should_record(&self, ptr: *const u8) -> bool {
        let usable_size = unsafe { malloc_usable_size(ptr as *const _) };

        usable_size == self.bin_size
    }
}

/// Filter that records all allocations.
#[derive(Debug, Default)]
pub struct NoopFilter;

impl AllocFilter for NoopFilter {
    #[inline]
    fn should_record(&self, _: *const u8) -> bool {
        true
    }
}
