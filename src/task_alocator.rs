use std::i32;

use anyhow::{Result, anyhow};
use rand::Rng;

#[derive(Debug)]
pub struct Allocator {
    machines: Vec<Machine>,
    makespan: i32,
}

#[derive(Debug)]
pub struct Machine {
    makespan: i32,
    tasks: Vec<i32>,
}

impl std::fmt::Display for Allocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, m) in self.machines.iter().enumerate() {
            write!(f, "M{i:02} ({:4}): ", m.get_makespan())?;
            for t in m.tasks.iter().cloned() {
                write!(f, "{t:4}")?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "\nMakespan total: {}", self.get_makespan())
    }
}

impl Allocator {
    pub fn new(m: i32, r: f64) -> Result<Self> {
        // verifica se as entradas são válidas
        if m < 1 {
            return Err(anyhow!("Deve haver ao menos uma máquina"));
        }
        if r < 1.0 {
            return Err(anyhow!("r deve ser maior do que 1"));
        }

        let mut f_machines = Vec::<Machine>::new();

        // preenche primeira máquina com m^n tarefas
        let n = (m as f64).powf(r).floor() as i32;
        let mut first_machine = Machine::new();
        for _ in 0..n {
            first_machine.add_random_task();
        }
        let f_makespan = first_machine.get_makespan();
        f_machines.push(first_machine);

        // adiciona as m-1 máquinas restantes
        for _ in 1..m {
            f_machines.push(Machine::new());
        }

        Ok(Self {
            machines: f_machines,
            makespan: f_makespan,
        })
    }

    pub fn get_makespan(&self) -> i32 {
        self.makespan
    }

    fn calculate_makespan(&self) -> i32 {
        let mut max = i32::MIN;
        for m in &self.machines {
            if m.makespan > max {
                max = m.makespan;
            }
        }
        max
    }

    fn print_diff(&self, task: i32, srcm: usize, dstm: usize) {
        for (i, m) in self.machines.iter().enumerate() {
            print!("M{i:02} ({:4}): ", m.get_makespan());
            for t in m.tasks.iter().cloned() {
                print!("{t:4}");
            }
            if i == srcm {
                print!("\x1b[38;2;0;0;255;48;2;255;0;0m{task:4}  \x1b[39;49m →");
            } else if i == dstm {
                print!("\x1b[38;2;255;0;0;48;2;0;0;255m{task:4}  \x1b[39;49m ←");
            }
            println!("");
        }
        println!("\nMakespan total: {}", self.get_makespan());
    }

    pub fn search_by_first_improve(&mut self) -> Result<i32> {
        let mut n_melhorias = 0;
        let n_machines = self.machines.len();

        println!("{self}");

        'existe_melhora: loop {
            for i in 0..n_machines {
                let task_option = self.machines[i].pop_tasks();

                match task_option {
                    None => continue,
                    Some(task) => {
                        for j in (i + 1)..n_machines {
                            self.machines[j].push_task(task);

                            let new_makespan = self.calculate_makespan();
                            if new_makespan < self.makespan {
                                self.makespan = new_makespan;
                                n_melhorias += 1;

                                // Código de diff
                                // A função de print se baseia no estado atual das máquinas, então
                                // é necessário remover a tarefa da maquina j temporariamente.
                                self.machines[j].pop_tasks();
                                self.machines[j].makespan += task;
                                self.print_diff(task, i, j);
                                self.machines[j].makespan -= task;
                                self.machines[j].push_task(task);

                                println!("\n\npress enter key");

                                let mut _s = String::default();
                                std::io::stdin().read_line(&mut _s)?;

                                continue 'existe_melhora;
                            } else {
                                self.machines[j].pop_tasks();
                            }
                        }
                        self.machines[i].push_task(task);
                    }
                }
            }
            break 'existe_melhora Ok(n_melhorias);
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

    fn add_to_makespan(&mut self, i: i32) {
        self.makespan += i;
    }

    fn add_random_task(&mut self) {
        let time = rand::rng().random_range(1..100);
        self.tasks.push(time);
        self.makespan += time;
    }

    fn push_task(&mut self, task: i32) {
        self.tasks.push(task);
        self.add_to_makespan(task);
    }

    fn pop_tasks(&mut self) -> Option<i32> {
        self.tasks.pop().inspect(|i| self.makespan -= *i)
    }
}
