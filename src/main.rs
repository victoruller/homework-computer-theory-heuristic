pub mod task_alocator;

use std::time::SystemTime;

use anyhow::Result;
use task_alocator::Allocator;

const TEMPURA_ITERS: u32 = 1000;

fn main() -> anyhow::Result<()> {
    let ms: [i32; 3] = [10, 20, 50];
    let rs = [1.5, 2.0];
    let alphas = [0.8, 0.85, 0.9, 0.95, 0.99];

    println!("heuristica,n,m,replicacao,tempo,iteracoes,valor,parametro");
    for m in ms {
        for r in rs {
            for a in alphas {
                for ct in 1..=10 {
                    // let iters = replicate_nth_search_by_fi(10, m, r, a)?;
                    let mut all = Allocator::new(false, m, r, a, TEMPURA_ITERS)?;

                    let time = SystemTime::now();
                    let iters = all.search_by_first_improve()?;
                    let elap = time.elapsed()?;

                    println!(
                        "monotonaprimeiramelhora,{},{m},{ct},{},{iters},{},{a}",
                        m.pow(r as u32),
                        elap.as_micros() as f64 * 0.000001f64,
                        all.get_makespan()
                    );
                }
            }
        }
    }

    for m in ms {
        for r in rs {
            for a in alphas {
                for ct in 1..=10 {
                    // let iters = replicate_nth_search_by_fi(10, m, r, a)?;
                    let mut all = Allocator::new(false, m, r, a, TEMPURA_ITERS)?;

                    let time = SystemTime::now();
                    let iters = all.search_by_fi_tempura()?;
                    let elap = time.elapsed()?;

                    println!(
                        "tempurasimulada,{},{m},{ct},{},{iters},{},{a}",
                        m.pow(r as u32),
                        elap.as_micros() as f64 * 0.000001f64,
                        all.get_makespan()
                    );
                }
            }
        }
    }

    for m in ms {
        for r in rs {
            for a in alphas {
                for ct in 1..=10 {
                    // let iters = replicate_nth_search_by_fi(10, m, r, a)?;
                    let mut all = Allocator::new(false, m, r, a, TEMPURA_ITERS)?;

                    let time = SystemTime::now();
                    let iters = all.search_by_best_improve();
                    let elap = time.elapsed()?;

                    println!(
                        "monotonamelhormelhora,{},{m},{ct},{},{iters},{},{a}",
                        m.pow(r as u32),
                        elap.as_micros() as f64 * 0.000001f64,
                        all.get_makespan()
                    );
                }
            }
        }
    }

    Ok(())
}
