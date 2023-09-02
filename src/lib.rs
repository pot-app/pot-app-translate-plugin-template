use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str,
    from: &str,
    to: &str,
    _needs: HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let token=client
        .get("https://edge.microsoft.com/translate/auth")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36 Edg/113.0.1774.42")
        .send()?
        .text()?;

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
                        .send()?.json()?;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let result = translate("Hello World\n\nHello Pot", "", "zh-Hans", HashMap::new()).unwrap();
        println!("{result}");
    }
}
