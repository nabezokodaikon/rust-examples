fn judg(i: i32) -> Result<(), String> {
    match i % 2 == 0 {
        true => Ok(()),
        false => Err("偶数ではありません".to_string()),
    }
}

fn temp(i: i32) -> Result<String, String> {
    judg(i)?;
    Ok("偶数です".to_string())
}

fn main() {
    let r = temp(3);
    println!("{:?}", r);
}
