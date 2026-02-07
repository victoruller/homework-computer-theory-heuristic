
pub mod alocador_tarefas;

use alocador_tarefas::Alocator;
fn main() {

    let m = [10, 20, 50];
    let r = [1.5, 2.0];

    let a = Alocator::create(0, 4.0).unwrap();
    print!("{a:#?}");
    let a = Alocator::create(6, 0.0).unwrap();
    print!("{a:#?}");
    let a = Alocator::create(6, 1.5).unwrap();
    print!("{a:#?}");


    println!("Hello, world!");
}
