pub struct Lanternfish {
    spawn_timer: isize,
}

impl Lanternfish {
    pub fn new(spawn_timer: isize) -> Lanternfish {
        Lanternfish { spawn_timer }
    }

    pub fn decrease_timer(&mut self) -> bool {
        if self.spawn_timer == 0 {
            self.spawn_timer = 6;
            return true;
        }
        self.spawn_timer -= 1;
        false
    }

    pub fn create_school_of_lanternfish(spawn_timers: Vec<isize>) -> Vec<Lanternfish> {
        let mut school: Vec<Lanternfish> = vec![];
        for timer in spawn_timers {
            let new_fish = Lanternfish::new(timer);
            school.push(new_fish);
        }
        school
    }
}

impl std::fmt::Display for Lanternfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spawn_timer)
    }
}
