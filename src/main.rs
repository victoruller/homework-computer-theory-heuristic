
pub mod task_alocator;

use task_alocator::Alocator;
fn main() {

    let m = [10, 20, 50];
    let r = [1.5, 2.0];

    
    let mut a = Alocator::create(m[0], r[0]).unwrap();
    println!("{a:#?}");
    let n_operations = a.search_by_first_improve();
    let makespan = a.get_makespan();
    println!("número de operações: {n_operations}\nmakespan: {makespan}\n{a:#?}");

}
