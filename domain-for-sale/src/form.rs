use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug)]
pub enum OfferCurrency {
    USDT,
    CNY,
}

#[derive(Deserialize, Validate)]
pub struct Offer {
    #[validate(length(
        min = 2,
        max = 50,
        message = "请输入正确的联系人/Please enter your full name"
    ))]
    pub name: String,

    #[validate(length(
        min = 3,
        max = 255,
        message = "请输入正确的邮箱/Please enter your email"
    ))]
    #[validate(email(message = "请输入正确的邮箱/Please enter your email"))]
    pub email: String,

    pub currency: OfferCurrency,

    #[validate(length(min = 16, message = "请完成人机验证/Please complete Captcha"))]
    pub captcha: String,
}
