use std::collections::HashMap;
use std::time::Duration;
use crossterm::{
    cursor,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::stdout;
use tokio::time;
use rand::Rng;

// pub : 해당 함수는 pub로써 다른 모듈에서 홏출이 가능함을 의미
// u64는 부호 없는 64bit
pub async fn run_pomodoro_timer(work_min: u64, break_min: u64) {
  // HashMap을 생성함.
  let ascii_map = get_ascii_art_map();

  loop {
      println!("\n🟢 Starting 25-minute Work Session...");
      // 타이머가 동작하는 과정에서 HashMap의 데이터를 참조할 수 있도록 함.
      run_timer(work_min, &ascii_map, "Work").await;

      println!("\n🟡 Starting 5-minute Break...");
      run_timer(break_min, &ascii_map, "Break").await;
  }
}

// -> 은 반환 타입을 지정함.
// & : 참조 (값을 빌려온다는 뜻임, 값을 복사하거나 소유권을 옮기는 것이 아닌 원본 데이터에 대한 접근만 허용)
// 'static : 생애 주기를 나타냄. 데이터가 프로그램 전체 실행 시간동안 유효함을 의미함. (항상 메모리에 남아 있음.)
fn get_random_motivation() -> &'static str {
    // vec! 는 동적 배열을 생성하는 매크로입니다.
    // let은 기본적으로 불변이고 이를 변경하기 위해서는 let mut을 사용해야 함.
    let messages = vec![
        "💀 공부 안 하면, 네 인생은 이미 끝났다. 지금이라도 붙잡아.",
        "🔥 넌 지금 노는 게 편하지? 그 대가로 평생 고통받을 준비는 됐냐?",
        "⚰️ 네가 공부를 포기하는 순간, 미래의 넌 숨 쉴 공간조차 없을 거다.",
        "💣 지금 넌 변명하고 있지만, 변명은 네 통장 잔고를 채워주지 않는다.",
        "⚡ 놀지 마라. 넌 이미 실패의 길을 걷고 있다. 멈추고 공부해.",
        "👀 너 지금 뭐하냐? 네 경쟁자는 이미 널 밟고 지나갔다.",
        "🩸 쉬고 싶다고? 그러면 평생 고생할 각오도 같이 해라.",
        "💥 지금의 게으름은 네 미래를 박살 낸다. 정신 차려!",
        "🔥 네가 원하는 게 성공이라면, 지금 당장 공부하러 가라. 아니면 꿈도 꾸지 마라.",
        "⚡ '나중에'는 없다. 지금 안 하면 넌 영원히 끝난다.",
        "🚀 너 지금 책 안 폈지? 그럼 미래에 넌 네 자리도 없을 거다.",
        "👊 공부 안 하면, 네 부모님 얼굴에 먹칠하는 거다. 지금 해라!",
        "😡 노력 안 하겠다고? 그럼 평생 땅바닥에서 굴러라.",
        "💢 지금이 힘들다고? 네가 포기하면 평생이 더 힘들어질 거다.",
        "⚰️ 네가 놀고 있는 동안, 성공은 이미 네 손을 떠났다.",
        "💀 너 지금 멈추면, 미래엔 시작조차 못 할 거다. 끝났다.",
        "🩸 공부하기 싫지? 그래, 그럼 평생 네 꿈은 쓰레기통에 처박혀 있을 거다.",
        "🔥 지금 시작해. 아니면 영원히 후회하며 살게 될 거다.",
        "⚡ 네가 '나중에'를 외치는 순간, 실패는 이미 네 곁에 있다.",
        "💀 실패할 용기가 있냐? 아니면 지금 공부할 용기를 내라.",
    ];

    let mut rng = rand::thread_rng(); // 랜덤 생성기를 초기화 진행
    let index = rng.gen_range(0..messages.len()); // 메시지 길이 내에서 랜덤 숫자를 생성
    messages[index] // 메시지 반환 (; 없으므로 그냥 반환하는 것)
}

// HashMap(key-value)을 생성하는 함수
// char:는 '0', '1', ..., ':' 과 같은 문자
// 어차피 사용해야되는 데이터이므로 &'static 을 사용하여 메모리로 부터 데이터를 참조한다는 것임. 
fn get_ascii_art_map() -> HashMap<char, Vec<&'static str>> {
    let mut map = HashMap::new(); // HashMap을 초기화

    // HashMap에 key에 대응하는 value를 삽입
    map.insert('0', vec![
        "  00000  ",
        " 0     0 ",
        " 0     0 ",
        " 0     0 ",
        " 0     0 ",
        " 0     0 ",
        "  00000  ",
    ]);
    map.insert('1', vec![
        "    11   ",
        "   111   ",
        "    11   ",
        "    11   ",
        "    11   ",
        "    11   ",
        "   11111 ",
    ]);
    map.insert('2', vec![
        "  22222  ",
        " 2     2 ",
        "       2 ",
        "    222  ",
        "   2     ",
        "  2      ",
        " 2222222 ",
    ]);
    map.insert('3', vec![
        "  33333  ",
        " 3     3 ",
        "       3 ",
        "   3333  ",
        "       3 ",
        " 3     3 ",
        "  33333  ",
    ]);
    map.insert('4', vec![
        "     44  ",
        "    444  ",
        "   4 44  ",
        "  4  44  ",
        " 4444444 ",
        "     44  ",
        "     44  ",
    ]);
    map.insert('5', vec![
        "  555555 ",
        "  5      ",
        "  55555  ",
        "       5 ",
        "       5 ",
        " 5     5 ",
        "  55555  ",
    ]);
    map.insert('6', vec![
        "   6666  ",
        "  6      ",
        "  6      ",
        "  66666  ",
        "  6    6 ",
        "  6    6 ",
        "   6666  ",
    ]);
    map.insert('7', vec![
        " 7777777 ",
        "      77 ",
        "     77  ",
        "    77   ",
        "   77    ",
        "   77    ",
        "   77    ",
    ]);
    map.insert('8', vec![
        "  88888  ",
        " 8     8 ",
        " 8     8 ",
        "  88888  ",
        " 8     8 ",
        " 8     8 ",
        "  88888  ",
    ]);
    map.insert('9', vec![
        "  99999  ",
        " 9     9 ",
        " 9     9 ",
        "  999999 ",
        "       9 ",
        "      9  ",
        "  9999   ",
    ]);
    map.insert(':', vec![
        "         ",
        "    .    ",
        "         ",
        "         ",
        "         ",
        "    .    ",
        "         ",
    ]);

    map // 데이터 삽입 완료 후에 HashMap을 반환
}

// sesstion_type은 문자열을 참조하고 있음. 만약 참조를 하지 않고 String을 해버리면 소유권이 이전됨.
async fn run_timer(minutes: u64, ascii_map: &HashMap<char, Vec<&str>>, session_type: &str) {
    // let mut은 변경 가능한 변수
    let mut total_seconds = minutes * 60;

    let mut current_motivation = get_random_motivation();
    let mut seconds_motivation = 0;

    while total_seconds > 0 {
        let mins = total_seconds / 60;
        let secs = total_seconds % 60;
        let time_str = format!("{:02}:{:02}", mins, secs); // 2자리로 숫자를 출력하도록 포맷팅.

        let mut stdout = stdout(); // 터미널 출력 스트림

        // 터미널 작업 실행을 위한 매크로 (터미널에 명령을 전달하여 특정 작업을 실행하는 역할)
        // target은 표준 출력 장치
        // 다음은 명령어를 작성 (커서 위치를 0,0 좌측 상단으로 이동시킴, 전체화면을 지움)
        // unwrap() 은 항상 성공할 것이라는 전제 하에 사용. 실패하면 프로그램 패닉 -> 프로그램 강제 종료 (따라서 이거보다 다른 방법을 사용하는 것이 좋음)
        // execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All)).unwrap(); -> 기존 코드를 아래와 같이 사용하는게 안전
        if let Err(error) = execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All)){
          eprintln!("Error Clearing the Terminal: {}", error);
        }

        // 색상은 삭제 하였음.
        if let Err(error) = execute!(
            stdout,
            Print(format!("🔔 {} Timer\n\n", session_type)),
        ) {
          eprintln!("Error Start Timer : {}", error);
        }

        if seconds_motivation >= 10 {
          current_motivation = get_random_motivation();
          seconds_motivation = 0;
        }

        println!("{}", current_motivation);
        println!("");

        // 총 7줄 (행)을 순회함.
        for row in 0..7 {
            let mut line = String::new();

            //시간 문자열을 순회함.
            for ch in time_str.chars() {
                // ascli_map에서 현재 문자(key)에 대한 데이터(value)를 가져옴.
                if let Some(ascii_art) = ascii_map.get(&ch) {
                    line.push_str(ascii_art[row]);
                    line.push_str("  ");
                }
            }
            // 누적된 데이터를 출력함.
            println!("{}", line);
        }

        // 1초 기다림
        time::sleep(Duration::from_secs(1)).await;
        total_seconds -= 1;
        seconds_motivation += 1;
    }
}
