# ConfKit CLI 发布流程（简版）

## 1. 版本管理
- 更新 `Cargo.toml` 版本号
- 更新 `CHANGELOG`

### 1.1 更新版本号
```sh
cargo release patch --execute --no-confirm
cargo release minor --execute --no-confirm
cargo release major --execute --no-confirm
```

### 1.2 生成 changelog
```sh
git cliff -o CHANGELOG.md
```

## 2. 构建产物
- MacOS: `cargo build --release --target x86_64-apple-darwin`
- Linux: `cargo build --release --target x86_64-unknown-linux-gnu`
- 产物路径：
  - MacOS: `target/x86_64-apple-darwin/release/confkit`
  - Linux: `target/x86_64-unknown-linux-gnu/release/confkit`

## 3. 产物分发
- 上传二进制文件至 GitHub Releases 或其他分发平台
- 附加校验和（SHA256）

## 4. 安装方式

### 4.1 直接下载
```bash
wget https://github.com/confkit-io/confkit/releases/latest/download/confkit-linux -O confkit
chmod +x confkit
./confkit --help
```

### 4.2 curl/wget 一键安装
```bash
curl -L https://github.com/confkit-io/confkit/releases/latest/download/confkit-linux -o /usr/local/bin/confkit
chmod +x /usr/local/bin/confkit
```

### 4.3 Homebrew（MacOS 推荐）
```bash
brew tap confkit-io/tap
brew install confkit
```

### 4.4 一键安装脚本（推荐）
- 在仓库根目录提供 `install.sh`，自动识别平台并安装，无需用户关心版本号。
- 用户一键安装命令：
```bash
curl -fsSL https://github.com/confkit-io/confkit/releases/latest/download/install.sh | sh
```

## 5. 发布公告
- 发布 Release Notes，说明主要变更与升级指引
