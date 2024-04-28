### 刷新Access Token

```
curl -i 'http://localhost:8081/infra/auth/access-token/refresh' \
  -H 'Accept: */*' \
  -H 'Accept-Language: en,zh-CN;q=0.9,zh;q=0.8,zh-TW;q=0.7,fr;q=0.6' \
  -H 'Connection: keep-alive' \
  -H 'Content-type: application/json' \
  -H 'DNT: 1' \
  -H 'Origin: https://tex.poemhub.top' \
  -H 'Referer: https://tex.poemhub.top/goods' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36' \
  -H 'sec-ch-ua: "Google Chrome";v="123", "Not:A-Brand";v="8", "Chromium";v="123"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "macOS"' \
  --data-raw '{"grant_type":"refresh_token","refresh_token":"2d7c51d0-66ac-4bd3-8fd4-b305561df525"}'
```
