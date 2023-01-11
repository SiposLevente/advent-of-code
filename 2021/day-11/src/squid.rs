use std::fmt::Display;

#[derive(Clone)]
pub struct Squid {
    pub energy: i32,
    pub flashed: bool,
}

impl Squid {
    pub fn new(energy: i32) -> Squid {
        return Squid {
            energy,
            flashed: false,
        };
    }

    pub fn increase_energy(&mut self) {
        if self.energy < 10 {
            self.energy += 1;
        }
    }
}

impl Display for Squid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = self.energy.clone().to_string();
        if self.energy > 9 {
            output = "O".to_string();
        }

        write!(f, "{}", output)
    }
}
