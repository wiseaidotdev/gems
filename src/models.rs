// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::responses::ModelInfo;
use crate::responses::ModelsResponse;
use crate::traits::CTrait;
use crate::Client;
use anyhow::{anyhow, Error};
use derive_builder::Builder;
use reqwest::Method;
use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Model {
    Pro31Preview,
    #[default]
    Flash3Preview,
    Flash31LitePreview,
    Embedding001,
    Imagen4,
    Veo31Preview,
    Tts31Preview,
    Flash31LivePreview,
    Flash31ImagePreview,
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Model::Pro31Preview => "gemini-3.1-pro-preview",
            Model::Flash3Preview => "gemini-3-flash-preview",
            Model::Flash31LitePreview => "gemini-3.1-flash-lite-preview",
            Model::Embedding001 => "gemini-embedding-001",
            Model::Imagen4 => "imagen-4.0-generate-001",
            Model::Veo31Preview => "veo-3.1-generate-preview",
            Model::Tts31Preview => "gemini-3.1-flash-tts-preview",
            Model::Flash31LivePreview => "gemini-3.1-flash-live-preview",
            Model::Flash31ImagePreview => "gemini-3.1-flash-image-preview",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Model {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gemini-3.1-pro-preview" => Ok(Model::Pro31Preview),
            "gemini-3-flash-preview" => Ok(Model::Flash3Preview),
            "gemini-3.1-flash-lite-preview" => Ok(Model::Flash31LitePreview),
            "gemini-embedding-001" => Ok(Model::Embedding001),
            "imagen-4.0-generate-001" => Ok(Model::Imagen4),
            "veo-3.1-generate-preview" => Ok(Model::Veo31Preview),
            "gemini-3.1-flash-tts-preview" => Ok(Model::Tts31Preview),
            "gemini-3.1-flash-live-preview" => Ok(Model::Flash31LivePreview),
            "gemini-3.1-flash-image-preview" => Ok(Model::Flash31ImagePreview),
            _ => Err(anyhow!("Unknown model: {}", s)),
        }
    }
}
#[derive(Clone)]
pub struct Models {
    pub client: Client,
}

#[derive(Builder, Default, Clone)]
#[builder(setter(into), default)]
pub struct Mod {
    pub model: Model,
}

impl Models {
    pub async fn list(&self) -> anyhow::Result<ModelsResponse> {
        let req = self.client.request(Method::GET, "models")?;

        let res = req.send().await?;
        Ok(res.json().await?)
    }

    pub async fn get(&self, _model: Mod) -> anyhow::Result<ModelInfo> {
        let req = self.client.request(Method::GET, "")?;

        let res = req.send().await?;
        Ok(res.json().await?)
    }
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
