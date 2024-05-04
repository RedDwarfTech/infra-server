


```bash
curl -g -X POST http://localhost:8081/infra/alipay/notification/v1/alipaySeverNotification?voucher_detail_list=[{"amount":"0.20","merchantContribute":"0.00","name":"5","otherContribute":"0.20","type":"ALIPAY_DISCOUNT_VOUCHER","voucherId":"2016101200073002586200003BQ4"}]
# legacy api
curl -g -X POST https://tex.poemhub.top/post/alipay/notification/v1/alipaySeverNotification?sign=e0g4oyhKJPf2M99C59iSds3LlyhIKvrdJxkb9aE8dSBDtmQvzQzdUNLOTsKdMp/u9JUqp4vw0C1foOsu/wDO2+otWRo1A/6HSGQsHuN31L6Wlm4UXh4aubS7AqeeXfIt4D1iqri2TMnef6KhkXrLfrwjSrm8aMi5j/Dj0taC2i/YwpXSXeNpcUfKj75il3+O2tGUSFVezHlN4dv2N3Bmzm1oDxSjrfcIWOEQWz+s7JZsgNAyk1eNMb/xb76DNc7jYYIhmQXeH0X3vDYzC9vuxk1lV8BVZe44iLFd7Jg7qchXVTgZB2WgrWHyJvZuNbkXckheApAU4d05jyeKv8NvIQ==&sign_type=rsa2&app_id=2021004119642155&auth_app_id=2021004119642155&buyer_id=2088402558023287&buyer_pay_amount=0.01&charset=UTF-8&fund_bill_list=[{"amount":"0.01","fundChannel":"ALIPAYACCOUNT"}]&gmt_create=2024-05-04+16:41:13&gmt_payment=2024-05-04+16:41:17&invoice_amount=0.01&notify_id=2024050401222164117023281450148821&notify_time=2024-05-04+20:08:06&notify_type=trade_status_sync&out_trade_no=629236395147399168&point_amount=0.0&receipt_amount=0.01&seller_id=2088541899098320&subject=TeXHub会员1天&total_amount=0.01&trade_no=2024050422001423281421845841&trade_status=TRADE_SUCCESS&version=1.0
curl -g -X POST https://tex.poemhub.top/post/alipay/notification/v1/alipaySeverNotification?voucher_detail_list=[{"amount":"0.20","merchantContribute":"0.00","name":"5","otherContribute":"0.20","type":"ALIPAY_DISCOUNT_VOUCHER","voucherId":"2016101200073002586200003BQ4"}]
```


```bash
curl -g -X POST https://localhost:8081/infra/alipay/notification/v1/alipaySeverNotification?voucher_detail_list=[{"amount":"0.20","merchantContribute":"0.00","name":"5折券","otherContribute":"0.20","type":"ALIPAY_DISCOUNT_VOUCHER","voucherId":"2016101200073002586200003BQ4"}]\&fund_bill_list=[{"amount":"0.80","fundChannel":"ALIPAYACCOUNT"},{"amount":"0.20","fundChannel":"MDISCOUNT"}]\&subject=PC网站支付交易\&trade_no=2016101221001004580200203978\&gmt_create=2016-10-12 21:36:12\&notify_type=trade_status_sync&total_amount=1.00&out_trade_no=mobile_rdm862016-10-12213600&invoice_amount=0.80&seller_id=2088201909970555&notify_time=2016-10-12 21:41:23&trade_status=TRADE_SUCCESS&gmt_payment=2016-10-12 21:37:19&receipt_amount=0.80&passback_params=passback_params123&buyer_id=2088102114562585&app_id=2016092101248425&notify_id=7676a2e1e4e737cff30015c4b7b55e3kh6& sign_type=RSA2&buyer_pay_amount=0.80&sign=&point_amount=0.00
```








