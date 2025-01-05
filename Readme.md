# SSU CLI

SSU CLI는 숭실대학교 소프트웨어학부 학생들을 위한 명령줄 인터페이스 도구입니다. 공지사항 알림과 뽀모도로 타이머 기능을 제공합니다.

## 주요 기능

### 1. 공지사항 알림 (`fetch-notice`)

- 숭실대학교 소프트웨어학부 공지사항을 자동으로 스크래핑
- 최신 공지사항 7개를 Telegram으로 전송
- 공지사항 제목, 링크 및 타임스탬프 포함
- Markdown 형식으로 포맷팅된 메시지 전송

### 2. 뽀모도로 타이머 (`timer`)

- 25분 작업 / 5분 휴식 주기의 뽀모도로 타이머
- ASCII 아트를 활용한 시각적 타이머 표시
- 작업/휴식 세션 구분을 위한 컬러 코딩
- 동기 부여를 위한 랜덤 메시지 표시 (한국어)

## 설치 요구사항

- Rust 환경
- 필수 크레이트:
  - tokio (비동기 런타임)
  - reqwest (HTTP 클라이언트)
  - scraper (웹 스크래핑)
  - crossterm (터미널 UI)
  - rustyline (명령줄 입력)
  - dotenv (환경 변수 관리)
  - chrono (날짜/시간 처리)
  - serde_json (JSON 처리)

## 환경 설정

1. `.env` 파일 생성 및 다음 변수 설정:

```
TELEGRAM_TOKEN=your_telegram_bot_token
TELEGRAM_CHAT_ID=your_telegram_chat_id
```

2. Telegram 봇 설정:
   - Telegram 봇 생성 및 토큰 획득
   - 채팅방 ID 설정

## 사용 방법

1. 프로젝트 실행:

```bash
cargo run
```

2. 사용 가능한 명령어:

```
help         : 사용 가능한 명령어 목록 표시
fetch-notice : 공지사항 확인 및 Telegram 전송
timer        : 뽀모도로 타이머 실행
exit         : 프로그램 종료
```

## 프로젝트 구조

```
src/
├── main.rs          # 메인 CLI 인터페이스
├── fetch_notice.rs  # 공지사항 스크래핑 및 Telegram 전송
└── pomodoro.rs      # 뽀모도로 타이머 구현
```

## 기술 스택

- **비동기 처리**: tokio
- **웹 스크래핑**: reqwest + scraper
- **터미널 UI**: crossterm
- **입력 처리**: rustyline
- **설정 관리**: dotenv
- **시간 처리**: chrono
- **데이터 처리**: serde_json

## 라이선스

MIT License

## 기여

버그 리포트, 기능 제안 및 풀 리퀘스트를 환영합니다.
