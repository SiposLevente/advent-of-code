use squid::Squid;
use std::io::Error;
use std::{env::args, fs};
mod squid;

fn main() {
    let mut squids = read_input("squids.txt").expect("Invalid file!");
    let step: isize = args().nth(1).unwrap().parse().unwrap();
    let mut flash_counter = 0;

    for _ in 0..step {
        increase_energy(&mut squids);

        while squids
            .iter()
            .flatten()
            .filter(|x| x.energy > 9 && !x.flashed)
            .count()
            > 0
        {
            let mut x = 0;
            let mut y = 0;

            while x < squids.len() {
                if y == squids[0].len() {
                    y = 0;
                    x += 1
                }

                if squids[x][y].energy > 9 && !squids[x][y].flashed {
                    break;
                }

                y += 1;
            }

            squids[x][y].flashed = true;
            flash_counter += 1;
            increase_neighbours(&mut squids, x, y);
        }

        for squid_vec in &mut squids {
            for squid in squid_vec {
                if squid.energy > 9 {
                    squid.flashed = false;
                    squid.energy = 0;
                }
            }
        }
    }

    for lines in squids {
        for squid in lines {
            print!("{}", squid);
        }
        println!()
    }

    println!("Total flashes: {}", flash_counter);
}

fn increase_energy(squids: &mut [Vec<Squid>]) {
    for squid_vec in &mut squids.iter_mut() {
        for squid_element in squid_vec {
            squid_element.increase_energy();
        }
    }
}

fn increase_neighbours(squids: &mut Vec<Vec<Squid>>, x: usize, y: usize) {
    for x_diff in -1_isize..2 {
        let x_delta = x_diff + x as isize;
        for y_diff in -1_isize..2 {
            let y_delta = y_diff + y as isize;

            if !(y_diff == 0 && x_diff == 0)
                && x_delta >= 0
                && x_delta < (squids.len()) as isize
                && y_delta >= 0
                && y_delta < (squids[x].len()) as isize
            {
                squids[x_delta as usize][y_delta as usize].increase_energy();
            }
        }
    }
}

fn read_input(file: &str) -> Result<Vec<Vec<Squid>>, Error> {
    let mut squids: Vec<Vec<Squid>> = vec![];
    let content = fs::read_to_string(file)?.replace('\r', "");
    let data = content.split('\n');
    for (counter, line) in data.enumerate() {
        squids.push(vec![]);
        for character in line.chars() {
            squids[counter].push(Squid::new(character as i32 - '0' as i32))
        }
    }

    Ok(squids)
}
