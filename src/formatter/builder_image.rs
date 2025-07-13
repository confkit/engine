//! Author: xiaoYown
//! Created: 2025-07-14
//! Description: Builder Image Formatter

use crate::types::config::ConfKitImageInfo;
use tabled::{builder::Builder, settings::Style};

pub struct BuilderImageFormatter;

impl BuilderImageFormatter {
    // 获取镜像列表

    // 打印镜像列表
    pub fn print_image_list(images: &[ConfKitImageInfo]) {
        let mut builder = Builder::new();

        builder.push_record(vec![
            "● Name",
            "● ID",
            "● Base Image",
            "● Tag",
            "● Created At",
            "● Size",
            "● Status",
        ]);

        for image in images {
            builder.push_record(vec![
                image.name.clone(),
                image.id.as_deref().unwrap_or("").to_string(),
                image.base_image.clone(),
                image.tag.clone(),
                image.created_at.as_deref().unwrap_or("").to_string(),
                image.size.as_deref().unwrap_or("").to_string(),
                format!("{}", image.status),
            ]);
        }

        let mut table = builder.build();
        table.with(Style::ascii_rounded());

        println!("{}", table);
    }

    // 打印单个镜像信息
    pub fn print_image_info(image: Option<&ConfKitImageInfo>) {
        if image.is_none() {
            tracing::error!("Image not found");
            return;
        }

        let image = image.unwrap();

        tracing::info!("● Name: {}", image.name);
        tracing::info!("● ID: {}", image.id.as_deref().unwrap_or(""));
        tracing::info!("● Base Image: {}", image.base_image);
        tracing::info!("● Tag: {}", image.tag);
        tracing::info!("● Created At: {}", image.created_at.as_deref().unwrap_or(""));
        tracing::info!("● Size: {}", image.size.as_deref().unwrap_or(""));
        tracing::info!("● Status: {}", image.status);
    }
}
