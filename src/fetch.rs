use anyhow::Result;
use hf_hub::Cache;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, RANGE};
use url::Url;

/// Insert the Hugging Face token into the headers
fn insert_hf_token_header(token: &Option<String>, headers: &mut HeaderMap) -> Result<HeaderMap> {
    if let Some(token) = token {
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
    }

    Ok(headers.clone())
}

/// Insert the Range header into the headers
fn insert_range_bytes_header(headers: &mut HeaderMap, start: u64, end: u64) -> Result<HeaderMap> {
    let range_header_value = format!("bytes={}-{}", start, end);
    headers.insert(RANGE, HeaderValue::from_str(&range_header_value)?);

    Ok(headers.clone())
}

pub fn fetch_remote_bytes(
    url: &Url,
    token: &Option<String>,
    start: u64,
    length: u64,
) -> Result<Vec<u8>> {
    let client = Client::new();

    let mut headers = HeaderMap::new();

    // insert headers
    insert_hf_token_header(&token, &mut headers)?;
    insert_range_bytes_header(&mut headers, start, start + length - 1)?;

    let res = client.get(url.as_str()).headers(headers).send()?;

    if res.status().is_success() {
        let bytes = res.bytes()?;
        Ok(bytes.to_vec())
    } else {
        Err(anyhow::anyhow!("Failed to fetch the data"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_example_8bytes() {
        // HTTPクライアントの作成
        let client = Client::new();

        // Rangeヘッダの設定
        let range_header_value = format!("bytes={}-{}", 0, 7); // インデックスを指定する; 0から7までの8バイトを取得

        // GETリクエストの送信
        let response = client
            .get("https://example.com")
            .header(RANGE, range_header_value)
            .send()
            .unwrap();

        // ステータスコードのチェック
        if response.status().is_success() {
            // バイトデータの取得
            let bytes = response.bytes().unwrap().to_vec();
            println!("{:?}", bytes);
            assert_eq!(bytes.len(), 8);
        }
        panic!("Failed to fetch the data");
    }

    #[test]
    fn test_fetch_remote_bytes() {
        let url = Url::parse("https://example.com").unwrap();

        let bytes = fetch_remote_bytes(&url, &None, 0, 8).unwrap();
        println!("{:?}", bytes);
        assert_eq!(bytes.len(), 8);

        let bytes2 = fetch_remote_bytes(&url, &None, 8, 8).unwrap();
        println!("{:?}", bytes2);
        assert_eq!(bytes2.len(), 8);

        assert_ne!(bytes, bytes2);
    }
}
