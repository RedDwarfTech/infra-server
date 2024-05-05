


```bash
curl -g -X POST http://localhost:8081/infra/alipay/notification/v1/alipaySeverNotification?voucher_detail_list=[{"amount":"0.20","merchantContribute":"0.00","name":"5","otherContribute":"0.20","type":"ALIPAY_DISCOUNT_VOUCHER","voucherId":"2016101200073002586200003BQ4"}]
# legacy api
curl -g -X POST https://tex.poemhub.top/post/alipay/notification/v1/alipaySeverNotification?notify_time=2024-05-05+00%3A05%3A29&notify_type=trade_status_sync&notify_id=2024050501222000529023281447805222&charset=UTF-8&version=1.0&sign_type=RSA2&sign=re%2F%2BdOtXB%2BQLZ1ng4xXMAy7OXp1vIv6ibR8bH6WaRtTFyG9Y%2BN8GmYytn6rj70cibMgckDx7VosivOST7FMtGwysZWle5eZwmer%2B3MAm6vxaY7O5zy2q6B4xmxkJh54HiBETs8OwFakg%2Fu%2FGStWTWtMK%2Fkmf73hKpNDUFXduv58HC7jiVGwgn94LnSrVgvv7yGHiHVeVsxJXwIFlvqSifhRfCr1klNcA5H78GLwqx5z35MciBZQwXUxG7Cy2DP02Y8Vfz%2FVo%2B%2BmX7bDUbPZXfL27yM4BPk5fJuRWhAgfhgbWcV3hDMonOASL6Bj0BXal85a1xCqFwOu5MURy7W8wGQ%3D%3D&auth_app_id=2021004119642155&trade_no=2024050522001423281421545941&app_id=2021004119642155&out_trade_no=629348220820131840&buyer_id=2088402558023287&seller_id=2088541899098320&trade_status=TRADE_SUCCESS&total_amount=0.01&receipt_amount=0.01&invoice_amount=0.01&buyer_pay_amount=0.01&point_amount=0.0&subject=TeXHub%E4%BC%9A%E5%91%981%E5%A4%A9&gmt_create=2024-05-05+00%3A05%3A23&gmt_payment=2024-05-05+00%3A05%3A28&fund_bill_list=%5B%7B%22amount%22%3A%220.01%22%2C%22fundChannel%22%3A%22ALIPAYACCOUNT%22%7D%5D
curl -g -X POST https://tex.poemhub.top/post/alipay/notification/v1/alipaySeverNotification?voucher_detail_list=[{"amount":"0.20","merchantContribute":"0.00","name":"5","otherContribute":"0.20","type":"ALIPAY_DISCOUNT_VOUCHER","voucherId":"2016101200073002586200003BQ4"}]
```


```bash
curl -g -X POST https://localhost:8081/infra/alipay/notification/v1/alipaySeverNotification?voucher_detail_list=[{"amount":"0.20","merchantContribute":"0.00","name":"5折券","otherContribute":"0.20","type":"ALIPAY_DISCOUNT_VOUCHER","voucherId":"2016101200073002586200003BQ4"}]\&fund_bill_list=[{"amount":"0.80","fundChannel":"ALIPAYACCOUNT"},{"amount":"0.20","fundChannel":"MDISCOUNT"}]\&subject=PC网站支付交易\&trade_no=2016101221001004580200203978\&gmt_create=2016-10-12 21:36:12\&notify_type=trade_status_sync&total_amount=1.00&out_trade_no=mobile_rdm862016-10-12213600&invoice_amount=0.80&seller_id=2088201909970555&notify_time=2016-10-12 21:41:23&trade_status=TRADE_SUCCESS&gmt_payment=2016-10-12 21:37:19&receipt_amount=0.80&passback_params=passback_params123&buyer_id=2088102114562585&app_id=2016092101248425&notify_id=7676a2e1e4e737cff30015c4b7b55e3kh6& sign_type=RSA2&buyer_pay_amount=0.80&sign=&point_amount=0.00
```








