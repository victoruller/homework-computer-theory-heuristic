pub mod task_alocator;

use task_alocator::Allocator;
fn main() -> anyhow::Result<()> {
    let m = [10, 20, 50];
    let r = [1.5, 2.0];
    let a = [0.8, 0.85, 0.9, 0.95, 0.99];

    let mut a = Allocator::new(false, m[2], r[1], a[4], 1000).unwrap();

    let n_operations = a.search_by_fi_tempura()?;
    // let n_operations = a.search_by_first_improve()?;

    let makespan = a.get_makespan();

    println!("número de operações: {n_operations}\nmakespan: {makespan}\n");

    println!("\x1b[2J\x1b[H");
    println!("\x1b[0;30;107mSOLUTION\x1b[0;38;48m\n\n{a}");

    Ok(())
}
