use std::fs;
use std::env;

pub fn load_env() -> Result<(), Box<dyn std::error::Error>> {
  // .env로 부터 값을 불러온다음
  let content = fs::read_to_string(".env")?;
  for line in content.lines() {
    if let Some((key, value)) = line.split_once("=") {
      // 앞뒤 공백을 삭제한 이후에 환경 변수를 설정함.
      env::set_var(key.trim(), value.trim());
    }
  }
  Ok(())
}