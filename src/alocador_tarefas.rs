use rand::Rng;
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct Alocator {
  machines: Vec::<Machine>,
}

#[derive(Debug)]
pub struct Machine {
  makespan: i32,
  tasks: Vec::<i32>,
}

impl Alocator {

  pub fn create(m: i32, r: f64) -> Result<Self> {

    // verifica se as entradas são válidas
    // if m < 1 { return Err(anyhow!("Deve haver ao menos uma máquina")) }
    // if r < 1.0 { return Err(anyhow!("r deve ser maior do que 1"))}

    let mut f_machines = Vec::<Machine>::new();


    // preenche primeira máquina com m^n tarefas
    let n = (m as f64).powf(r).floor() as i32;
    let mut first_machine = Machine::new();
    for _ in 0..n {
      first_machine.add_randon_task();
    }
    f_machines.push(first_machine);


    // adiciona as m-1 máquinas restantes
    for _ in 1..m {
      f_machines.push(Machine::new());
    }


    Ok(Self {machines: f_machines})
  }

}

impl Machine {

  fn new() -> Self {
    Self {
      makespan: 0,  
      tasks: Vec::<i32>::new(),
    }
  }


  fn get_makespan(&self) -> i32 {
    return self.makespan;
  }

  fn add_randon_task(&mut self) {
    let time = rand::rng().random_range(1..100);
    self.tasks.push(time);
    self.makespan += time;
  }

  fn pop_tasks(&mut self) -> Option<i32> {
    self.tasks.pop()
  }
}