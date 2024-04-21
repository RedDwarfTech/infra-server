

### 用户操作

主要测试用户相关操作，登陆、获取当前用户信息等。

#### 用户登陆



```bash

curl -X 'PUT' 'http://localhost:8081/infra/user/login' \
  -H 'Accept: application/json, text/plain, */*' \
  -H 'Accept-Language: en,zh-CN;q=0.9,zh;q=0.8,zh-TW;q=0.7,fr;q=0.6' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: application/json' \
  -H 'DNT: 1' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36' \
  -H 'sec-ch-ua: "Google Chrome";v="123", "Not:A-Brand";v="8", "Chromium";v="123"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "macOS"' \
  -H 'x-action: LOGIN_BY_PHONE' \
  -H 'x-request-id: 66dca516-2847-447a-b625-ddfdcdd294ce' \
  --data-raw '{"phone":"15086958137","password":"654321","deviceId":"36d70fa8fc9e46f435bcd38ec41b86e9","deviceName":"36d70fa8fc9e46f435bcd38ec41b86e9","deviceType":4,"appId":"n29Pa29WS1","loginType":1,"cfToken":"0.hkQia4lneDHjxzDpPqHpaR1YIywjrx_nlBDzQTidD1MJ28Ok-UUIfEIzLYwRURhnvt2gCBs_3S1UnP6gUdOi43jwNqeU8YPZtg7KCF0UfV2lTa6mAfqkai9VOCYKVE2CkyNvxCEaAkI7ofTx4-BmcmLi8zZSpXekOgF24eEJBjo6g7BpvUp_BM7YPjxbIHOWFrGcfP9itxdyvoxE4EHSkN1S2Xk6SfsqE-29_eXueejSrF00bhQDGkifaQcnTXJT22WGNPC5zqx_lOzl-AOjTjmCF6Z8o6Rb7d-Ro39p4mASB8NwoGGdGH_gH5nJTknoueWGJMzR2J16GJu8R5PaYgTObmV9Vtotj83WazR6ptlakKmwVBMF-AAEm7CIn0kvAzY66MqnCLAr5EHEFeWl9CRpYMsxu2n-EWxAm4P3Yuf4vPiaTHYbReW1yyQ5zlHH.jJ8dZ78kyMgtNqUqzTCtIw.bf8bc985bdb3bd4e5fa5b61f468fabc317d9b82f999a584599773023d1b20263"}'
```

#### 获取当前用户信息


```bash
curl 
```





















