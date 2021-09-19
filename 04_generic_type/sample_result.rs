fn do_something_that_might_fail(i:i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("Not a correct value."))
    }
}

fn main() -> Result<(), String> {
    let result = do_something_that_might_fail(12);

    match result {
        Ok(v) => println!("{} is found", v),
        Err(_e) =>  {
            // println!("Error: {}", e),
            return Err(String::from("Something happen in main"));
        }
    }
    // Error発生なしの場合、Ok内部のUnit値を返す
    Ok(())
}