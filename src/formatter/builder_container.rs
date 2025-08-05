//! Author: xiaoYown
//! Created: 2025-07-14
//! Description: Builder Container Formatter

use crate::types::config::EngineContainerInfo;
use tabled::{builder::Builder, settings::Style};

pub struct BuilderContainerFormatter;

impl BuilderContainerFormatter {
    // 获取镜像列表

    // 打印镜像列表
    pub fn print_container_list(containers: &[EngineContainerInfo]) {
        let mut builder = Builder::new();

        builder.push_record(vec![
            "● Name",
            "● ID",
            "● Image",
            "● Created At",
            "● Size",
            "● Status",
        ]);

        for container in containers {
            builder.push_record(vec![
                container.name.clone(),
                container.id.clone(),
                container.image.clone(),
                container.created_at.clone(),
                container.size.clone(),
                container.status.clone().to_string(),
            ]);
        }

        let mut table = builder.build();
        table.with(Style::ascii_rounded());

        println!("{table}");
    }

    // 打印单个镜像信息
    pub fn print_container_info(container: Option<&EngineContainerInfo>) {
        if container.is_none() {
            tracing::error!("Container not found");
            return;
        }

        let container = container.unwrap();

        tracing::info!("● ID: {}", container.id);
        tracing::info!("● Name: {}", container.name);
        tracing::info!("● Image: {}", container.image);
        tracing::info!("● Status: {}", container.status);
    }
}
