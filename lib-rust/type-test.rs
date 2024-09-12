type Dev = i32;

fn main() {
    println!("{:?}", std::any::type_name::<Dev>());

    println!("{}", std::any::type_name::<Dev>() == "i32");
}