# 多阶段构建，减少镜像体积
FROM rust:1.75-slim as builder

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟 main.rs 进行依赖预构建
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 复制实际源码
COPY src ./src

# 构建应用
RUN cargo build --release

# 运行阶段，使用轻量级基础镜像
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -m -s /bin/bash confkit

# 复制构建的二进制文件
COPY --from=builder /app/target/release/confkit /usr/local/bin/confkit

# 设置权限
RUN chmod +x /usr/local/bin/confkit

# 创建工作目录
WORKDIR /workspace
RUN chown confkit:confkit /workspace

# 切换到非 root 用户
USER confkit

# 设置环境变量
ENV PATH="/usr/local/bin:$PATH"

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD confkit --version || exit 1

# 默认命令
ENTRYPOINT ["confkit"]
CMD ["--help"]

# 添加标签
LABEL maintainer="confkit team" \
      version="1.0.0" \
      description="ConfKit CLI - Configuration-driven build and deployment tool" \
      org.opencontainers.image.title="ConfKit CLI" \
      org.opencontainers.image.description="Configuration-driven build and deployment tool" \
      org.opencontainers.image.source="https://github.com/confkit/engine" \
      org.opencontainers.image.url="https://github.com/confkit/engine" \
      org.opencontainers.image.documentation="https://github.com/confkit/engine/blob/main/README.md" 