use tau::Tau;

#[derive(Clone, Debug)]
pub struct Particle {
    pub id           : u64,
    pub positions    : Vec<Vec<f64>>,
    pub angle_cm     : f64,
    pub old_angle_cm : f64,
    pub taus         : Vec<Tau>,
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            id           : 0,
            positions    : Vec::new(),
            angle_cm     : -0.0,
            old_angle_cm : -0.0,
            taus         : Vec::new(),
        }
    }

    pub fn allocate (&mut self, steps : u64, dimensions : u64) -> &mut Particle {
        self.positions    = vec![vec![-10241024.1024f64; dimensions as usize] ; steps as usize];
        let taus_no : u64 = steps - 1;
        self.taus         = vec![Tau::new()
                                 .finalize(); taus_no as usize];
        for tau in 0..taus_no {
            self.taus[tau as usize].delta_steps       = tau + 1;
            self.taus[tau as usize].last_initial_step = steps - tau - 1;
        }
        
        self
    }
    
    pub fn finalize(&self) -> Particle {
        self.clone()
    }

    pub fn msd(&mut self) {
        for tau in &mut self.taus {
            tau.msd(&self.positions);
        }
    }

    
}
