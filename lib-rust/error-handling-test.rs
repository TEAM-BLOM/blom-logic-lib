
fn add_usize_int32(a: usize, b: i32) -> Result<usize, usize> {
    if b >= 0 {
        Ok(a + b as usize)
    } else {
        let b_abs = b.abs() as usize;
        if a < b_abs {
            return Err(0);
        } else {
            Ok(a - b_abs)
        }
    }
}

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    match add_usize_int32(0, 1) {
        Ok(x) => {
            println!("{:?}", x);
            print_type(&x);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
    
}