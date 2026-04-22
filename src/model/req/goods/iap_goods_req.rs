use validator::Validate;

#[derive(serde::Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct IapGoodsReq {
    #[serde(default = "default_lang")]
    pub lang: String,
}

fn default_lang() -> String {
    "zh-CN".to_string()
}