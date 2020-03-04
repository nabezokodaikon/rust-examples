fn judg(i: i32) -> Result<(), String> {
    match i % 2 == 0 {
        true => Ok(()),
        false => Err("偶数ではありません".to_string()),
    }
}

fn temp(i: i32) -> Result<(), String> {
    judg(i)?;
    println!("偶数です");
    Ok(())
}

fn main() {
    temp(2);
}
