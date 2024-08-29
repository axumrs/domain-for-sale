use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    Json,
};
use tokio::time;
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
    let captcha_verify = match captcha::verify(&state.cfg.hcaptcha, &frm.captcha)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))
    {
        Ok(v) => v,
        Err(_) => {
            return Err(Error::from(anyhow::anyhow!(
                "人机验证超时/Timeout for verify captcha"
            )));
        }
    };

    if !captcha_verify {
        return Err(anyhow::anyhow!("请完成人机验证/Please complete captcha").into())
            .map_err(log_error(handler_name));
    }

    let body = format!(
        "联系人：{}\n邮箱：{}\n货币：{:?}\n",
        &frm.name, frm.email, frm.currency
    );

    let send_mail_timeout = state.cfg.mail.timeout;

    let subject = format!("来自{}的意向", &frm.name);
    // 发送邮件
    let mut task_send = tokio::spawn(async move {
        mail::send(&state.cfg.mail, &subject, body).map_err(log_error(handler_name))
    });

    let sleep = time::sleep(time::Duration::from_secs(send_mail_timeout as u64));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            _=&mut sleep => {
                tracing::warn!("发送邮件超时");
                task_send.abort();
                return Err(Error::from(anyhow::anyhow!("发送邮件超时/Timeout for sending mail")));
            }
            r=(&mut task_send) => {
                if let Ok(Err(_)) = r {
                     return Err(Error::from(anyhow::anyhow!("发送邮件失败/Failed to send email")));
                }
                break;
            }
        }
    }

    Ok(Json(resp::Resp::ok(frm.email.clone())))
}

fn log_error(handler_name: &str) -> Box<dyn Fn(Error) -> Error> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("👉 [{}] - {}", handler_name, err.msg());
        err
    })
}
