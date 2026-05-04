// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::client::Client;
use crate::messages::Message;
use crate::models::Model;
use crate::requests::{ImagenParameters, ImagenPrompt, ImagenRequest};
use crate::responses::ImagenPredictResponse;
use crate::traits::CTrait;
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use derive_builder::Builder;
use reqwest::Method;
#[derive(Clone)]
pub struct Images {
    pub client: Client,
}

#[derive(Builder, Clone)]
#[builder(setter(into))]
pub struct ImageGen {
    pub model: Model,
    pub input: Message,
    #[builder(setter(into, strip_option), default)]
    pub system: Option<Vec<Message>>,
}

impl Images {
    pub async fn generate(&self, params: ImageGen) -> Result<Vec<u8>> {
        let request_body = ImagenRequest {
            instances: vec![ImagenPrompt {
                prompt: params.input.get_text(),
            }],
            parameters: ImagenParameters {
                sample_count: 1,
                aspect_ratio: Some("1:1".to_string()),
            },
        };

        let req = self
            .client
            .request(Method::POST, "predict")?
            .json(&request_body);

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await?;
            return Err(anyhow!("API Error ({}): {}", status, error_text));
        }

        let json: ImagenPredictResponse = res.json().await?;

        let prediction = json
            .predictions
            .first()
            .ok_or_else(|| anyhow!("No predictions in response"))?;

        let image_data = general_purpose::STANDARD
            .decode(&prediction.bytes_base64_encoded)
            .map_err(|e| anyhow!("Failed to decode base64 image: {}", e))?;

        Ok(image_data)
    }
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
