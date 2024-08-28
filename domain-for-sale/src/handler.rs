use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    Json,
};
use validator::Validate;

use crate::{captcha, form, mail, resp, AppState, Assets, Error, Result};
const INDEX_HTML: &str = "index.html";

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches("/");
    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains(".") {
                return not_found().await;
            }
            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

pub async fn offer(
    State(state): State<Arc<AppState>>,
    Json(frm): Json<form::Offer>,
) -> Result<Json<resp::Resp<String>>> {
    let handler_name = "offer";

    frm.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    // 人机验证
    let captcha_verify = captcha::verify(&state.cfg.hcaptcha, &frm.captcha)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if !captcha_verify {
        return Err(anyhow::anyhow!("请完成人机验证/Please complete captcha").into())
            .map_err(log_error(handler_name));
    }

    let body = format!(
        "联系人：{}\n邮箱：{}\n货币：{:?}\n",
        &frm.name, frm.email, frm.currency
    );
    let subject = format!("来自{}的意向", &frm.name);
    // 发送邮件
    tokio::spawn(async move {
        mail::send(&state.cfg.mail, &subject, body).map_err(log_error(handler_name))
    });
    Ok(Json(resp::Resp::ok(frm.email.clone())))
}

fn log_error(handler_name: &str) -> Box<dyn Fn(Error) -> Error> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("👉 [{}] - {}", handler_name, err.msg());
        err
    })
}
