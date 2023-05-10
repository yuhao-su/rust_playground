use tikv_jemalloc_ctl::{epoch, stats};
// use tikv_jemallocator::Jemalloc;

//  a#[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

fn print_stats() {
    let jemalloc_epoch_mib = epoch::mib().unwrap();
    jemalloc_epoch_mib.advance().unwrap();
    println!(
        "allocated: {}",
        stats::allocated::mib().unwrap().read().unwrap()
    );
    println!(
        "metadata: {}",
        stats::metadata::mib().unwrap().read().unwrap()
    );
    println!("active: {}", stats::active::mib().unwrap().read().unwrap());
    println!(
        "resident: {}",
        stats::resident::mib().unwrap().read().unwrap()
    );
}

fn main() {
    let _d1 = vec![0u64; 1 << 20];
    unsafe {
    tikv_jemalloc_sys::malloc_stats_print(None, std::ptr::null_mut(), std::ptr::null_mut());
    }
    print_stats();
    println!("{}", _d1.len());
}