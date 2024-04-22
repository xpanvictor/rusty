#[macro_export]
macro_rules! xvec {
    ( $($x: expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                println!("Adding value: {:?}", $x);
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    println!("Macro file");
    let data = xvec![3, 4];
    println!("Final vec {:?}", data);
}
