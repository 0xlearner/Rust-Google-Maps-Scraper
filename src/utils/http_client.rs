use reqwest::header;

pub fn create_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.7".parse().unwrap());
    headers.insert("priority", "u=1, i".parse().unwrap());
    headers.insert("referer", "https://www.google.com/".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Brave\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-arch", "\"x86\"".parse().unwrap());
    headers.insert("sec-ch-ua-bitness", "\"64\"".parse().unwrap());
    headers.insert(
        "sec-ch-ua-full-version-list",
        "\"Brave\";v=\"131.0.0.0\", \"Chromium\";v=\"131.0.0.0\", \"Not_A Brand\";v=\"24.0.0.0\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-model", "\"\"".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());
    headers.insert("sec-ch-ua-platform-version", "\"6.8.0\"".parse().unwrap());
    headers.insert("sec-ch-ua-wow64", "?0".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".parse().unwrap());
    headers
}
