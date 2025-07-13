# ConfKit CLI 部署检查清单

本文档提供了 ConfKit CLI 发布和部署的完整检查清单，确保每个步骤都能正确执行。

## 📋 发布前准备

### 1. 开发环境检查
- [ ] 确认 Rust 版本 >= 1.70
- [ ] 运行 `cargo fmt` 格式化代码
- [ ] 运行 `cargo clippy` 检查代码质量
- [ ] 运行 `cargo test` 确保所有测试通过
- [ ] 运行 `cargo build --release` 确保可以正常构建

### 2. 文档更新
- [ ] 更新 `README.md` 和 `README.zh.md`
- [ ] 更新 `CHANGELOG.md` 添加新版本记录
- [ ] 检查所有示例配置文件是否正确
- [ ] 确认 API 文档是否需要更新

### 3. 版本管理
- [ ] 在 `Cargo.toml` 中更新版本号
- [ ] 确认遵循语义化版本控制
- [ ] 检查所有依赖版本是否合适

## 🔧 CI/CD 配置

### 1. GitHub Actions 设置
- [ ] 确认 `.github/workflows/release.yml` 文件存在
- [ ] 配置 GitHub Secrets：
  - [ ] `CRATES_TOKEN` - crates.io 发布令牌
  - [ ] `DOCKER_USERNAME` - Docker Hub 用户名
  - [ ] `DOCKER_PASSWORD` - Docker Hub 密码

### 2. 构建测试
- [ ] 在本地测试多平台构建：
  ```bash
  # Linux
  cargo build --release --target x86_64-unknown-linux-gnu
  
  # macOS
  cargo build --release --target x86_64-apple-darwin
  cargo build --release --target aarch64-apple-darwin
  
  # Windows (如果在 Windows 上)
  cargo build --release --target x86_64-pc-windows-msvc
  ```

### 3. Docker 测试
- [ ] 本地构建 Docker 镜像：
  ```bash
  docker build -t confkit/cli:test .
  docker run --rm confkit/cli:test --version
  ```

## 🚀 发布流程

### 1. 代码准备
- [ ] 创建发布分支：`git checkout -b release/v1.x.x`
- [ ] 最终代码检查和测试
- [ ] 合并到主分支：`git checkout main && git merge release/v1.x.x`

### 2. 创建发布标签
- [ ] 创建带注释的标签：
  ```bash
  git tag -a v1.x.x -m "Release v1.x.x
  
  新功能:
  - 功能描述
  
  改进:
  - 改进描述
  
  修复:
  - 修复描述
  "
  ```
- [ ] 推送标签：`git push origin v1.x.x`

### 3. 自动化发布验证
- [ ] 检查 GitHub Actions 是否成功运行
- [ ] 验证 GitHub Release 是否创建成功
- [ ] 确认所有平台的二进制文件都已上传
- [ ] 检查 crates.io 是否发布成功
- [ ] 验证 Docker Hub 镜像是否推送成功

## 📦 安装脚本部署

### 1. 安装脚本准备
- [ ] 确认 `install.sh` 脚本测试通过
- [ ] 确认 `install.ps1` 脚本测试通过
- [ ] 更新脚本中的版本检测逻辑

### 2. 域名和托管设置
- [ ] 注册 `install.confkit.io` 域名（推荐）
- [ ] 配置 CDN 加速（可选）
- [ ] 部署安装脚本到 Web 服务器：
  ```bash
  # 上传到服务器
  scp install.sh user@server:/var/www/install.confkit.io/
  scp install.ps1 user@server:/var/www/install.confkit.io/
  ```

### 3. 安装脚本测试
- [ ] 测试 Linux 安装：
  ```bash
  curl -sSL https://install.confkit.io | sh
  ```
- [ ] 测试 macOS 安装
- [ ] 测试 Windows 安装：
  ```powershell
  irm https://install.confkit.io/install.ps1 | iex
  ```

## 🍺 包管理器发布

### 1. Homebrew Formula
- [ ] 创建 Homebrew Tap 仓库：`confkit/homebrew-tap`
- [ ] 创建 Formula 文件：
  ```ruby
  # Formula/confkit.rb
  class Confkit < Formula
    desc "Configuration-driven build and deployment tool"
    homepage "https://github.com/confkit/engine"
    url "https://github.com/confkit/engine/archive/v1.x.x.tar.gz"
    sha256 "..."
    
    depends_on "rust" => :build
    
    def install
      system "cargo", "install", "--root", prefix, "--path", "."
    end
  end
  ```

### 2. Scoop Manifest
- [ ] 创建 Scoop Bucket 仓库：`confkit/scoop-bucket`
- [ ] 创建 Manifest 文件：
  ```json
  {
    "version": "1.x.x",
    "description": "Configuration-driven build and deployment tool",
    "homepage": "https://github.com/confkit/engine",
    "url": "https://github.com/confkit/engine/releases/download/v1.x.x/confkit-x86_64-pc-windows-msvc.zip",
    "hash": "...",
    "bin": "confkit.exe"
  }
  ```

### 3. APT 包（可选）
- [ ] 设置 APT 仓库
- [ ] 创建 deb 包
- [ ] 配置 GPG 签名

## 🔍 发布后验证

### 1. 安装测试
- [ ] 测试所有平台的安装方式
- [ ] 验证命令行功能正常
- [ ] 检查版本信息正确

### 2. 文档验证
- [ ] 检查 GitHub Release 页面
- [ ] 验证 README 中的安装说明
- [ ] 确认所有链接都能正常访问

### 3. 社区通知
- [ ] 发布 GitHub Release Notes
- [ ] 更新项目文档网站（如果有）
- [ ] 社交媒体发布公告（可选）

## 📊 监控和反馈

### 1. 下载监控
- [ ] 监控 GitHub Releases 下载量
- [ ] 检查 crates.io 统计信息
- [ ] 观察 Docker Hub 拉取次数

### 2. 问题跟踪
- [ ] 关注 GitHub Issues
- [ ] 处理用户反馈
- [ ] 修复发现的问题

### 3. 后续维护
- [ ] 定期更新依赖
- [ ] 准备下一个版本
- [ ] 维护文档更新

## 🆘 故障排除

### 常见问题解决方案

#### GitHub Actions 构建失败
- 检查 Cargo.toml 依赖是否正确
- 确认所有平台的构建环境
- 检查 Secrets 配置

#### 安装脚本失败
- 验证下载链接是否正确
- 检查文件权限设置
- 确认平台检测逻辑

#### 包管理器问题
- 检查 hash 值是否正确
- 验证下载 URL 可访问
- 确认包描述信息准确

## 📝 发布记录模板

每次发布后，在团队内部记录以下信息：

```markdown
## 发布记录 v1.x.x

**发布日期**: 2025-01-13
**发布人**: 姓名
**发布渠道**: GitHub Releases, crates.io, Docker Hub

### 成功指标
- [ ] 所有平台构建成功
- [ ] 安装脚本测试通过
- [ ] 文档更新完成

### 遇到的问题
- 问题描述和解决方案

### 下次改进
- 流程优化建议
```

---

**注意**: 此检查清单应根据实际情况调整，并在每次发布后进行更新和完善。 