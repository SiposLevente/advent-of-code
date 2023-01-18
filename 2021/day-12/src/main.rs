use map::CaveMap;

mod map;

fn main() {
    let cave = CaveMap::new("input.txt");
    println!("{}", cave);
    cave.start_search();
}
