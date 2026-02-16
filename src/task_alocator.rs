use std::i32;

use anyhow::{Result, anyhow};
use rand::{Rng, random};

#[derive(Debug)]
pub struct Allocator {
    machines: Vec<Machine>,
    machines_star: Vec<Machine>,
    makespan: i32,
    makespan_star: i32,
    alpha: f64,
    display: bool,
    tempura_iters: u32,
}

#[derive(Debug)]
pub struct Machine {
    makespan: i32,
    tasks: Vec<i32>,
}

impl Clone for Machine {
    fn clone(&self) -> Self {
        let mut tk = vec![];
        for v in self.tasks.iter().cloned() {
            tk.push(v);
        }
        Machine {
            makespan: self.makespan,
            tasks: tk,
        }
    }
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
    pub fn new(display: bool, m: i32, r: f64, a: f64, tempura_iters: u32) -> Result<Self> {
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
            machines: f_machines.clone(),
            machines_star: f_machines,
            makespan: f_makespan,
            makespan_star: f_makespan,
            alpha: a,
            display,
            tempura_iters,
        })
    }

    pub fn new_randonly_distributed(
        display: bool,
        m: i32,
        r: f64,
        a: f64,
        tempura_iters: u32,
    ) -> Result<Self> {
        let mut f_machines = Vec::<Machine>::new();
        let n = (m as f64).powf(r).floor() as i32;
        for _ in 0..m {
            f_machines.push(Machine::new());
        }

        // preenche máquinas aleatoriamente
        for _ in 0..n {
            f_machines[rand::rng().random_range(0..m) as usize].add_random_task();
        }
        let mut alocator = Self {
            machines: f_machines.clone(),
            machines_star: f_machines,
            makespan: 0,
            makespan_star: 0,
            alpha: a,
            display,
            tempura_iters,
        };
        alocator.makespan = alocator.calculate_makespan();
        Ok(alocator)
    }

    fn shadow(&mut self) {
        self.makespan_star = self.makespan;
        self.machines_star.clear();
        for v in self.machines.iter().cloned() {
            self.machines_star.push(v);
        }
    }

    fn unshadow(&mut self) {
        self.makespan = self.makespan_star;
        self.machines.clear();
        for v in self.machines_star.iter().cloned() {
            self.machines.push(v);
        }
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

    fn tempura_criteria(&self, delta_s: f64, t: f64) -> bool {
        if delta_s <= 0f64 {
            false
        } else {
            ((-delta_s / t).exp() * 100f64) as u64 > (random::<u64>() % 100)
        }
    }

    pub fn search_by_fi_tempura(&mut self) -> Result<i32> {
        let mut temperature = 100f64;
        let mut n_melhorias = 0;
        let mut n_tentativas = 0;
        let mut shuffle_idx = 0;
        let n_machines = self.machines.len();

        if self.display {
            println!("\x1b[2J\x1b[H");
        }

        'existe_melhora: loop {
            for i in 0..n_machines {
                if self.machines[i].makespan != self.makespan {
                    continue;
                }

                let task_option = self.machines[i].pop_tasks();

                let Some(task) = task_option else { continue };

                for j in 0..n_machines {
                    if i == j {
                        continue;
                    }

                    self.machines[j].push_task(task);

                    let old_makespan = self.makespan;
                    let new_makespan = self.calculate_makespan();

                    if new_makespan < self.makespan_star {
                        self.makespan = new_makespan;
                        self.shadow();

                        if self.display {
                            // Código de diff
                            self.machines[j].pop_tasks();
                            self.machines[j].makespan += task;
                            println!("temperature :: {}ºC", temperature);
                            self.print_diff(task, i, j);
                            self.machines[j].makespan -= task;
                            self.machines[j].push_task(task);

                            println!("\n\n\n\n\n\npress enter key\n\n");

                            let mut _s = String::default();
                            std::io::stdin().read_line(&mut _s)?;

                            println!("\x1b[2J\x1b[H");
                        }

                        n_melhorias += 1;
                        n_tentativas = 0;

                        continue 'existe_melhora;
                    } else {
                        self.makespan = old_makespan;
                        self.machines[j].pop_tasks();
                    }
                }

                self.machines[i].push_task(task);

                let old_makespan = self.makespan;

                let mut ct = self.machines[i].tasks.len();
                loop {
                    if shuffle_idx == i {
                        shuffle_idx += 1;
                        if shuffle_idx >= n_machines {
                            shuffle_idx = 0;
                        }
                        continue;
                    }

                    let task = self.machines[i]
                        .pop_tasks()
                        .ok_or(anyhow!("out of tasks"))?;

                    self.machines[shuffle_idx].push_task(task);

                    shuffle_idx += 1;
                    ct -= 1;
                    if shuffle_idx >= n_machines {
                        shuffle_idx = 0;
                    }
                    if ct == 0 {
                        break;
                    }
                }

                self.makespan = self.calculate_makespan();

                if self.tempura_criteria((self.makespan - old_makespan) as f64, temperature) {
                    if self.display {
                        println!("\x1b[2J\x1b[H");
                        println!("\x1b[0;5;30;107mSHUFFLE VIA TEMPURA\x1b[0;25;38;48m\n");
                    }
                    temperature *= self.alpha;
                    n_melhorias += 1;
                    n_tentativas = 0;
                    continue 'existe_melhora;
                } else {
                    n_tentativas += 1;
                    temperature *= self.alpha;
                    if n_tentativas == self.tempura_iters {
                        break;
                    } else {
                        if self.display {
                            println!("\x1b[2J\x1b[H");
                            println!(
                                "\x1b[0;5;30;107mNO SHUFFLE\x1b[0;25;38;48m ({}) {}ºC\n",
                                n_tentativas, temperature
                            );
                            println!("{self}");
                            let mut _s = String::default();
                            std::io::stdin().read_line(&mut _s)?;
                        }
                        continue 'existe_melhora;
                    }
                }
            }
            self.unshadow();
            break 'existe_melhora Ok(n_melhorias);
        }
    }

    fn get_lower_makespan_machine_index(&self) -> usize {
        let mut lower_makespan = i32::MAX;
        let mut lower_makespan_machine = 0;
        for i in 0..self.machines.len() {
            let current_machine_makespan = self.machines[i].get_makespan();
            if current_machine_makespan < lower_makespan {
                lower_makespan_machine = i;
                lower_makespan = current_machine_makespan;
            }
        }
        return lower_makespan_machine;
    }

    pub fn search_by_best_improve(&mut self) -> i32 {
        let mut n_melhorias = 0;

        if self.display {
            println!("{self}");
        }

        loop {
            let task = self.machines[0].pop_tasks().unwrap_or(0);
            let stand_by_machine = self.get_lower_makespan_machine_index();
            self.machines[stand_by_machine].push_task(task);
            let new_makespan = self.calculate_makespan();
            if new_makespan < self.get_makespan() {
                self.makespan = new_makespan;
                n_melhorias += 1;

                if self.display {
                    self.machines[stand_by_machine].pop_tasks();
                    self.print_diff(task, 0, stand_by_machine);
                    self.machines[stand_by_machine].push_task(task);

                    println!("\n\npress enter key");

                    let mut _s = String::default();
                    std::io::stdin().read_line(&mut _s).unwrap();
                }
                continue;
            } else {
                break n_melhorias;
            }
        }
    }

    pub fn search_by_first_improve(&mut self) -> Result<i32> {
        let mut n_melhorias = 0;
        let n_machines = self.machines.len();

        if self.display {
            println!("{self}");
        }

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

                                if self.display {
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
                                }

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
