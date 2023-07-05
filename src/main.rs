mod comp;
mod phy;

fn main() {
    let mut world = comp::sample_world();
    println!("{}", world);
    for i in 0..20{
        phy::update(1.0,&mut world);
        println!("After update {}\n{}",i, world);
    }
}
 
