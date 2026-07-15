use std::io::{self, Read, Write};
use serde_json::Value;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    // Read message length (first 4 bytes, native endian)
    let mut length_bytes = [0u8; 4];
    if handle.read_exact(&mut length_bytes).is_err() {
        return Ok(());
    }
    let length = u32::from_ne_bytes(length_bytes) as usize;
    
    // Read message
    let mut message = vec![0u8; length];
    handle.read_exact(&mut message)?;
    
    let value: Value = serde_json::from_slice(&message).unwrap();
    
    // Process message - echo back
    let response = serde_json::json!({"status": "received", "data": value});
    let response_bytes = serde_json::to_vec(&response).unwrap();
    
    let mut stdout = io::stdout();
    stdout.write_all(&(response_bytes.len() as u32).to_ne_bytes())?;
    stdout.write_all(&response_bytes)?;
    stdout.flush()?;
    
    Ok(())
}
