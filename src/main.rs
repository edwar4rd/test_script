use std::io;

fn main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer == "q\n" {
            break;
        }
        println!("v0.0.2");
    }
    Ok(())
}
