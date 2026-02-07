use rand::Rng;
use anyhow::{Result, anyhow};


#[derive(Debug)]
pub struct Alocator {
  machines: Vec::<Machine>,
  makespan: i32,
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
    let f_makespan = first_machine.get_makespan();
    f_machines.push(first_machine);


    // adiciona as m-1 máquinas restantes
    for _ in 1..m {
      f_machines.push(Machine::new());
    }


    Ok(Self {machines: f_machines, makespan: f_makespan})
  }

  pub fn get_makespan(&self) -> i32 {
    self.makespan
  }

  fn calculate_makespan(&self) -> i32 {
    let mut max = -2147483648;
    for m in &self.machines {
      if m.makespan > max {
        max = m.makespan;
      }
    }
    max
  }


  pub fn search_by_first_improve(&mut self) -> i32 {
    let mut n_melhorias = 0;
    let n_machines = self.machines.len();

    'existe_melhora: loop {
      for i in 0..n_machines {
        let task_option = self.machines[i].pop_tasks();

        match task_option {
          None => continue,
          Some(task) => {
            for j in (i+1)..n_machines {
              self.machines[j].add_task(task);
              
              let new_makespan = self.calculate_makespan();
              if new_makespan < self.makespan {
                self.makespan = new_makespan;
                n_melhorias += 1;
                continue 'existe_melhora;
              } else {
                self.machines[j].pop_tasks();
              }
            }
            self.machines[i].add_task(task);
          }
        }
      }
      break 'existe_melhora n_melhorias;
    }
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

  fn edit_makespan(&mut self, i: i32) {
    self.makespan += i;
  }

  fn add_randon_task(&mut self) {
    let time = rand::rng().random_range(1..100);
    self.tasks.push(time);
    self.makespan += time;
  }

  fn add_task(&mut self, task: i32) {
    self.tasks.push(task);
    self.edit_makespan(task);
  }

  fn pop_tasks(&mut self) -> Option<i32> {
    match self.tasks.pop() {
      None => None,
      Some(i) => {
        self.makespan -= i;
        Some(i)
      }
    }
  }
}