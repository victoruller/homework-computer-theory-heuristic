pub mod task_alocator;

use task_alocator::Allocator;
fn main() -> anyhow::Result<()> {
    let m = [10, 20, 50];
    let r = [1.5, 2.0];

    let mut a = Allocator::new(m[0], r[0]).unwrap();
    let n_operations = a.search_by_first_improve()?;
    let makespan = a.get_makespan();
    println!("número de operações: {n_operations}\nmakespan: {makespan}\n{a}");
    Ok(())
}
