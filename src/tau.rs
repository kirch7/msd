#[derive(Clone, Debug)]
pub struct Tau {
    pub delta_steps       : u64,
    pub last_initial_step : u64,
    pub msd         : f64,
}

impl Tau {
    pub fn new() -> Tau {
        Tau {
            delta_steps       : 0,
            last_initial_step : 0,
            msd               : -0.0,
        }
    }
    
    pub fn msd(&mut self, positions : &Vec<Vec<f64>>) {
        let mut m = -0f64;
        for initial_step in  0..self.last_initial_step {
            let final_step = initial_step + self.delta_steps;
            m += module2(&positions[final_step as usize],
                         &positions[initial_step as usize]);
        }
        
        self.msd = m / self.last_initial_step as f64;
    }

    pub fn finalize(&self) -> Tau {
        self.clone() // Dereference.
    }

}

fn square(a : f64) -> f64 {
    (a * a) // Returned value.
}

fn module2(v1 : &Vec<f64>, v2 : &Vec<f64>) -> f64 {
    assert_eq!(v1.len(), v2.len());

    let mut sum = -0.0f64;

    for i in 0..v1.len() {
        sum += square(v1[i] - v2[i]);
    }
    
    sum
}
