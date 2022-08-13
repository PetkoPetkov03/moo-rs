use text_io::read;

fn main() {
    println!("The cow will say");
    let i: String = read!();
    println!("_____________\n
  < {} > \n_____________ \n
            \\
            \\
            ((...))
            ( o o ) 
             |   |
              ^_^  

", i);
}
