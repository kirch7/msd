use std::io::BufReader;
use std::io::BufRead; ////
use std::io::Lines;
use std::fs::File;

#[derive(Clone, Copy, Debug)]
pub struct Parameters {
    pub particles     : u64,
    pub steps         : u64,
    pub exit_interval : u64,
    pub dimensions    : u64,
    pub dt            : f64,
    pub range         : f64,
}

impl Parameters {
    pub fn exit_steps(&self) -> u64 {
        (self.steps / self.exit_interval) + 1
    }

    pub fn read(file_buffer : &mut BufReader<&File>) -> Parameters {
        fn unwrap_several_times(my_lines : &mut Lines<&mut BufReader<&File>>) -> String {
            my_lines
                .nth(0)
                .unwrap()
                .unwrap()
                .to_string()
        }
        
        let mut lines = file_buffer.lines();
        let particles : u64 = unwrap_several_times(&mut lines)
            .parse::<u64>()
            .unwrap();
        let steps : u64 = unwrap_several_times(&mut lines)
            .parse::<u64>()
            .unwrap();
        let exit_interval : u64 = unwrap_several_times(&mut lines)
            .parse::<u64>()
            .unwrap();
        let dimensions : u64 = unwrap_several_times(&mut lines)
            .parse::<u64>()
            .unwrap();
        let dt : f64 = unwrap_several_times(&mut lines)
            .parse::<f64>()
            .unwrap();
        let range : f64 = unwrap_several_times(&mut lines)
            .parse::<f64>()
            .unwrap();
        
        Parameters {
            particles     : particles,
            steps         : steps,
            exit_interval : exit_interval,
            dimensions    : dimensions,
            dt            : dt,
            range         : range,
        }
    }
}

