fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("Not a correct value"))
    }
}

fn main() -> Result<(), String> {
    let v = do_something_that_might_fail(42)?;

    // sample_result.rsに書いた、パターンマッチを簡潔に記述
    println!("{} found", v);
    Ok(())
} 
