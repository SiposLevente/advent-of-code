use map::CaveMap;

mod map;

fn main() {
    let mut cave = CaveMap::new("input.txt");
    cave.start_search();
    for route in &cave.routes {
        println!("{:?}",route);
    }
    println!("Number of routes: {}", cave.routes.len());
}
