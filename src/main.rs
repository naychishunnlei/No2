// fn main() {
//     println!("Hello, world!");
// }

fn main(){
    let args:Vec<String> = std::env::args().collect();

    let temp1_arg = if args.len() < 2 {""} else {&args[1]};
    let temp2_arg = if args.len() < 3  {""} else {&args[2]};
    let step_arg = if args.len() < 4 {""} else {&args[3]};

    let temp1: i32 = temp1_arg.parse().unwrap_or(0);
    let temp2: i32 = temp2_arg.parse().unwrap_or(0);
    let step: usize = step_arg.parse().unwrap_or(0);

    println!("Fahr Celcius");
    if temp1 <= temp2 {
        for fah in (temp1..=temp2).step_by(step){
            let cel: f32 = temp::fah2cel(fah);
            println!("{:>4} {:>7.1}", fah, cel);        
            }
        }
    
    else {
        for fah in (temp2..=temp1).rev().step_by(step){
            let cel: f32 = temp::fah2cel(fah);
            println!("{:>4} {:>7.1}", fah, cel);
        }
    }
}