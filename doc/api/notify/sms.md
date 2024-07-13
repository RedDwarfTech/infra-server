
#### 发送短信

```bash
curl -X PUT 'http://127.0.0.1:8081/infra/user/pwd/send-reset-verify-code' \
-H 'Content-Type: application/json' \
-d '{
  "phone": "15683761628",
  "app_id": "n29Pa29WS1"
}'
```


