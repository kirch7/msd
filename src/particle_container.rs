use std::vec::Vec;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use particle::Particle;
use parameters::Parameters;

const BYTES_IN_A_FLOAT : usize = 4;

#[derive(Clone, Debug)]
pub struct ParticleContainer {
    pub particles      : Vec<Particle>,
    parameters         : Arc<Mutex<Parameters>>,
}

impl ParticleContainer {
    pub fn new(p : &Arc<Mutex<Parameters>>) -> ParticleContainer {
        let u_p = &p.lock().unwrap();
        ParticleContainer {
            particles : vec![Particle::new()
                             .allocate(u_p.exit_steps(),
                                       u_p.dimensions)
                             .finalize(); u_p.particles as usize],
            parameters : p.clone(),
        }
    }

    pub fn ids(&mut self) -> &mut ParticleContainer {
        for id in 0..(self.particles.len()) {
            self.particles[id].id = id as u64;
        }
        self
    }
    
    pub fn fill(&mut self, file_buffer : &mut BufReader<&File>) -> &mut ParticleContainer {
        use std::mem;
        use std::io::prelude::*;

        {
            let u_p = self.parameters.lock().unwrap();
            for step in 0..u_p.exit_steps() {
                for particle in 0..u_p.particles {
                    for dimension in 0..u_p.dimensions {
                        let mut slice = [0u8; BYTES_IN_A_FLOAT];
                        match file_buffer
                            .read_exact(&mut slice) {
                                Err(err) => { panic!("{}", err.to_string()); }
                                Ok(())   => { }
                            }
                        let banana : f64 = unsafe {
                            mem::transmute::<[u8; BYTES_IN_A_FLOAT], f32>(slice) as f64
                        };
                        //// println!("{}\t{}\t{}\t{}", step, particle, dimension, banana);
                        self.particles[particle as usize]
                            .positions[step as usize][dimension as usize] = banana.clone();
                    }
                    {
                        let mut slice = [0u8; 8];
                        match file_buffer
                            .read_exact(&mut slice) {
                                Err(err) => { panic!("{}", err.to_string()); }
                                Ok(())   => { }
                            }
                    }
                }
            }
        }
        self
    }

    pub fn finalize(&self) -> ParticleContainer {
        self.clone()
    }
    
    pub fn msd(&mut self) {
        for particle in &mut self.particles {
            particle.msd();
        }
    }
}
