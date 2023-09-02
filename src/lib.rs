use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Info {
    id: String,
    display: String,
    plugin_type: String,
    needs: Vec<Need>,
    language: HashMap<String, String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Need {
    key: String,
    display: String,
}

#[no_mangle]
pub async fn translate(
    text: &str,
    from: &str,
    to: &str,
    _needs: HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::ClientBuilder::new().build()?;

    let token=client
        .get("https://edge.microsoft.com/translate/auth")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.42")
        .send().await?
        .text().await?;

    let res:Value=client.post("https://api-edge.cognitive.microsofttranslator.com/translate")
                        .header("accept", "*/*")
                        .header("accept-language", "zh-TW,zh;q=0.9,ja;q=0.8,zh-CN;q=0.7,en-US;q=0.6,en;q=0.5")
                        .header("authorization", format!("Bearer {token}"))
                        .header("cache-control", "no-cache")
                        .header("content-type", "application/json")
                        .header("pragma", "no-cache")
                        .header("sec-ch-ua", "\"Microsoft Edge\";v=\"113\", \"Chromium\";v=\"113\", \"Not-A.Brand\";v=\"24\"")
                        .header("sec-ch-ua-mobile", "?0")
                        .header("sec-ch-ua-platform", "\"Windows\"")
                        .header("sec-fetch-dest", "empty")
                        .header("sec-fetch-mode", "cors")
                        .header("sec-fetch-site", "cross-site")
                        .header("Referer", "https://pot-app.com/")
                        .header("Referrer-Policy", "strict-origin-when-cross-origin")
                        .header("User-Agent",
                                        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.42")
                        .query(&[
                                    ("from", from),
                                    ("to", to),
                                    ("api-version", "3.0"),
                                    ("includeSentenceLength", "true"),
                        ])
                        .body(format!("[{{ \"Text\": \"{}\" }}]", text))
                        .send().await?.json().await?;

    fn parse_result(res: Value) -> Option<String> {
        let result = res
            .as_array()?
            .get(0)?
            .as_object()?
            .get("translations")?
            .as_array()?
            .get(0)?
            .as_object()?
            .get("text")?
            .as_str()?
            .to_string();
        Some(result)
    }
    if let Some(result) = parse_result(res) {
        return Ok(result);
    } else {
        return Err("Response Parse Error".into());
    }
}

#[no_mangle]
pub fn info() -> Result<Info, Box<dyn Error>> {
    let file = File::open("info.json")?;
    let reader = BufReader::new(file);

    let info: Info = serde_json::from_reader(reader)?;
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn try_request() {
        let result = translate("Hello World\n\nHello Pot", "", "zh-Hans", HashMap::new())
            .await
            .unwrap();
        println!("{result}");
    }

    #[test]
    fn try_get_info() {
        let info = info().unwrap();
        println!("{info:?}");
    }
}
