use reqwest::Client;
use scraper::{Html, Selector};
use chrono::{Local, NaiveDate};
use serde_json::json;

pub async fn fetch_and_send(telegram_token: &str, chat_id: &str) -> Result<(), Box<dyn std::error::Error>> {
  let client = Client::builder().build()?;
  let url = "https://sw.ssu.ac.kr/bbs/board.php?bo_table=notice";

  let response = client.get(url).send().await?;
  let body = response.text().await?;

  let notices = parse_and_extract(&body);
  send_to_telegram(&client, telegram_token, chat_id, notices).await?;

  println!(
      "시간 : {}\n텔레그램 전송 성공",
      Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
  );

  Ok(())
}

fn parse_and_extract(html: &str) -> Vec<(String, String, String)> {
  let document = Html::parse_document(html);
    let row_selector = Selector::parse("tr.bo_notice").unwrap();
    let title_selector = Selector::parse("td.td_subject a").unwrap();
    let date_selector = Selector::parse("td.td_datetime").unwrap();

    let mut notices: Vec<(String, String, NaiveDate)> = Vec::new();

    for row in document.select(&row_selector) {
        let title_element = row.select(&title_selector).next();
        let title = title_element
            .map(|e| e.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .unwrap_or("제목 없음".to_string());

        let link = title_element
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("링크 없음")
            .to_string();

        let date_element = row.select(&date_selector).next();
        if let Some(date_text) = date_element.map(|e| e.text().collect::<Vec<_>>().join(" ").trim().to_string()) {
            if let Ok(date) = NaiveDate::parse_from_str(&date_text, "%Y-%m-%d") {
                notices.push((title, link, date));
            }
        }
    }

    notices.sort_by(|a, b| b.2.cmp(&a.2));
    notices
        .into_iter()
        .take(7)
        .map(|(title, link, date)| (title, link, date.to_string()))
        .collect()
}

async fn send_to_telegram(
  client: &Client,
  token: &str,
  chat_id: &str,
  data: Vec<(String, String, String)>,
) -> Result<(), Box<dyn std::error::Error>> {
  let telegram_url = format!("https://api.telegram.org/bot{}/sendMessage", token);
  let now = Local::now();
  let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

  let mut message = format!("📢 *[숭실대 SW 공지사항]*\n_업데이트: {}_\n\n", timestamp);

  for (i, (title, link, _)) in data.iter().enumerate() {
      message.push_str(&format!(
          "*{}. {}*\n📌 링크 : {}\n\n",
          i + 1,
          title,
          link,
      ));
  }

  message.push_str("🔗 [더 많은 공지 보러가기](https://sw.ssu.ac.kr/bbs/board.php?bo_table=notice)");

  let payload = json!({
      "chat_id": chat_id,
      "text": message,
      "parse_mode": "Markdown",
  });

  client.post(&telegram_url).json(&payload).send().await?;
  Ok(())
}
