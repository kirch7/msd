extern crate msd;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use msd::parameters::Parameters;
use msd::particle_container::ParticleContainer;

fn main() {
    if let Some(input_filename) = std::env::args().nth(1) { // If there is some argv[1] like.
        let file = File::open(input_filename).unwrap(); // File variable.
        let mut file_buffer = BufReader::new(&file);    // Buffer with a pointer associated with the previous file.
        let parameters = Arc::new(Mutex::new(Parameters::read(&mut file_buffer))); // Pointer to mutex with Parameters.
        let mut particle_container = ParticleContainer::new(&parameters) // Allocate.
            .ids()                                                       // Set IDs.
            .fill(&mut file_buffer)                                      // Fill with information from file (buffer).
            .finalize();                                                 // Dereference.
        particle_container.msd();                                        // Call MSD calculation.
        
        if false { ////
            for particle in &particle_container.particles {
                for tau in &particle.taus {
                    println!("{}\t{}", tau.delta_steps, tau.last_initial_step);
                }
            }
        }
        
        if false {
            //// let u_p = parameters.lock().unwrap();
            for tau in &particle_container.particles[0].taus {
                println!("{}\t{}", tau.delta_steps as f64, tau.msd);
            }
        }
        
        if true {
            let u_p = parameters.lock().unwrap();
            let mut mean_msd = vec![-0.0f64; u_p.exit_steps() as usize];
            for particle in &particle_container.particles {
                for tau in &particle.taus {
                    mean_msd[(tau.delta_steps - 1) as usize] += tau.msd / u_p.particles as f64;
                    //// println!("{}\t{}",
                    ////         tau.delta_steps as f64 * u_p.exit_interval as f64 * u_p.dt,
                    ////         tau.msd);
                }
                //// print!("\n\n");
            }

            for i in 0..(u_p.exit_steps() - 1) as usize {
                //// println!("{}\t{}", (i + 1) as f64 * u_p.exit_interval as f64 * u_p.dt, mean_msd[i]);
                println!("{}\t{}", (i + 1) as f64 * u_p.exit_interval as f64 * u_p.dt, particle_container.particles[0].taus[i].msd);
            }
            
        }
        
    } else {
        panic!("Please, enter with a input file name.");
    }
}
