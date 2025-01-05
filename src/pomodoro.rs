use std::collections::HashMap;
use std::time::Duration;
use crossterm::{
    cursor,
    execute,
    style::{Print, SetForegroundColor, Color, ResetColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;
use tokio::time;
use rand::Rng;

fn get_random_motivation() -> &'static str {
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

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..messages.len());
    messages[index]
}

fn get_ascii_art_map() -> HashMap<char, Vec<&'static str>> {
    let mut map = HashMap::new();

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

    map
}

pub async fn run_pomodoro_timer(work_min: u64, break_min: u64) {
    let ascii_map = get_ascii_art_map();

    loop {
        println!("\n🟢 Starting 25-minute Work Session...");
        run_timer(work_min, &ascii_map, "Work").await;

        println!("\n🟡 Starting 5-minute Break...");
        run_timer(break_min, &ascii_map, "Break").await;
    }
}

async fn run_timer(minutes: u64, ascii_map: &HashMap<char, Vec<&str>>, session_type: &str) {
    let mut total_seconds = minutes * 60;

    let mut current_motivation = get_random_motivation();
    let mut seconds_motivation = 0;

    while total_seconds > 0 {
        let mins = total_seconds / 60;
        let secs = total_seconds % 60;
        let time_str = format!("{:02}:{:02}", mins, secs);

        let mut stdout = stdout();

        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All)).unwrap();

        execute!(
            stdout,
            SetForegroundColor(if session_type == "Work" {
                Color::Green
            } else {
                Color::Yellow
            }),
            Print(format!("🔔 {} Timer\n\n", session_type)),
            ResetColor
        )
        .unwrap();

        if seconds_motivation >= 10 {
          current_motivation = get_random_motivation();
          seconds_motivation = 0;
        }

        println!("{}", current_motivation);
        println!("");

        for row in 0..7 {
            let mut line = String::new();
            for ch in time_str.chars() {
                let color = match ch {
                    '0'..='9' => Color::Blue,
                    ':' => Color::Red,
                    _ => Color::White,
                };
                execute!(stdout, SetForegroundColor(color)).unwrap();
                if let Some(ascii_art) = ascii_map.get(&ch) {
                    line.push_str(ascii_art[row]);
                    line.push_str("  ");
                }
            }
            execute!(stdout, ResetColor).unwrap();
            println!("{}", line);
        }

        time::sleep(Duration::from_secs(1)).await;
        total_seconds -= 1;
        seconds_motivation += 1;
    }
}
