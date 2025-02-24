use std::str::FromStr;

use by_macros::*;
use dioxus::{prelude::*, CapturedError};
use dioxus_translate::Language;
use dto::{AssetPresignedUris, Content, ContentCreateRequest};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

use crate::{config, route::Route};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub contents: Signal<Vec<ContentCreateRequest>>,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            nav: use_navigator(),
            contents: use_signal(|| vec![ContentCreateRequest::default()]),
        };

        Ok(ctrl)
    }

    pub fn add_content(&mut self) {
        self.contents.push(ContentCreateRequest::default());
    }

    pub fn set_content(&mut self, idx: usize, content: ContentCreateRequest) {
        self.contents.with_mut(move |contents| {
            contents[idx] = content;
        });
    }

    pub fn handle_cancel(&self) {
        if self.nav.can_go_back() {
            self.nav.go_back();
        } else {
            self.nav.push(Route::ContentsPage { lang: self.lang });
        }
    }

    pub async fn handle_submit(&self) -> std::result::Result<(), CapturedError> {
        let endpoint = config::get().new_api_endpoint;
        Content::get_client(endpoint)
            .create_bulk(self.contents())
            .await?;

        Ok(())
    }
}

pub async fn handle_upload(
    file_bytes: Vec<u8>,
    ext: String,
) -> std::result::Result<String, CapturedError> {
    let cli = AssetPresignedUris::get_client(config::get().new_api_endpoint);
    let res = match cli
        .get_presigned_uris(1, dto::FileType::from_str(&ext).unwrap_or_default())
        .await
    {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("Failed to get presigned uris: {:?}", e);
            return Err(e.into());
        }
    };

    let presigned_uri = res.presigned_uris.first().context("No presigned uri")?;
    let uri = res.uris.first().context("No uri")?;

    use infer;

    let kind = infer::get(&file_bytes).context("Failed to infer file type")?;
    tracing::debug!("Inferred file type: {:?}", kind);

    let content_type = kind.mime_type().to_string();
    tracing::debug!("Content type: {:?}", content_type);
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str(&content_type).context("could not convert content type to header")?,
    );

    reqwest::Client::new()
        .put(presigned_uri)
        .body(file_bytes)
        .headers(headers)
        .send()
        .await
        .context("Failed to upload file")?;

    Ok(uri.to_string())
}
