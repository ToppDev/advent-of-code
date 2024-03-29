use day_04::part2::process;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{}", result);
}
