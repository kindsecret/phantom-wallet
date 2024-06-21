use bs58;
use std::env;
use console::style;
use std::io::Write;
use std::fs::File;


fn write_file(phantom: Vec<u8>) -> std::io::Result<()> {
    let serialized: String = serde_json::to_string(&phantom).unwrap();
    let mut file = File::create("phantom.json")?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let secret_key = &args[1];
    let decoded = bs58::decode(
        secret_key,
    )
    .into_vec().unwrap();
    let result = write_file(decoded);
    match result {
        Ok(..) => { 
            println!(
                "✅{}",
                style("Generated File!: ./phantom.json").white().bold()
            );
         }
        Err(err) => { println!("Failed to Write: {}", err) }
    }
}
