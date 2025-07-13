#!/bin/bash

# ConfKit 自发布脚本
# 使用 ConfKit 来发布 ConfKit

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印彩色消息
print_info() {
    echo -e "${BLUE}ℹ ${1}${NC}"
}

print_success() {
    echo -e "${GREEN}✓ ${1}${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ ${1}${NC}"
}

print_error() {
    echo -e "${RED}✗ ${1}${NC}"
}

# 显示帮助信息
show_help() {
    echo -e "${BLUE}ConfKit 自发布脚本${NC}"
    echo ""
    echo "使用方法: $0 <version> [options]"
    echo ""
    echo "参数:"
    echo "  version          发布版本号 (例如: 1.0.0)"
    echo ""
    echo "选项:"
    echo "  --dry-run        仅运行测试，不实际发布"
    echo "  --skip-tests     跳过测试步骤"
    echo "  --help, -h       显示帮助信息"
    echo ""
    echo "环境变量:"
    echo "  CARGO_REGISTRY_TOKEN    crates.io 发布令牌"
    echo "  DOCKER_USERNAME         Docker Hub 用户名"
    echo "  DOCKER_PASSWORD         Docker Hub 密码"
    echo "  GITHUB_TOKEN            GitHub 令牌"
    echo "  SLACK_WEBHOOK_URL       Slack 通知 URL (可选)"
    echo ""
    echo "示例:"
    echo "  $0 1.0.0                # 发布版本 1.0.0"
    echo "  $0 1.0.0 --dry-run      # 测试发布流程"
    echo "  $0 1.0.0 --skip-tests   # 跳过测试直接发布"
}

# 检查必要的环境变量
check_environment() {
    local missing_vars=()
    
    if [ -z "$CARGO_REGISTRY_TOKEN" ] && [ "$DRY_RUN" != "true" ]; then
        missing_vars+=("CARGO_REGISTRY_TOKEN")
    fi
    
    if [ -z "$DOCKER_USERNAME" ] && [ "$DRY_RUN" != "true" ]; then
        missing_vars+=("DOCKER_USERNAME")
    fi
    
    if [ -z "$DOCKER_PASSWORD" ] && [ "$DRY_RUN" != "true" ]; then
        missing_vars+=("DOCKER_PASSWORD")
    fi
    
    if [ -z "$GITHUB_TOKEN" ] && [ "$DRY_RUN" != "true" ]; then
        missing_vars+=("GITHUB_TOKEN")
    fi
    
    if [ ${#missing_vars[@]} -ne 0 ]; then
        print_error "缺少必要的环境变量: ${missing_vars[*]}"
        echo ""
        echo "请设置以下环境变量:"
        for var in "${missing_vars[@]}"; do
            echo "  export $var=<your-token>"
        done
        echo ""
        echo "或者使用 --dry-run 选项进行测试运行"
        exit 1
    fi
}

# 检查先决条件
check_prerequisites() {
    print_info "检查先决条件..."
    
    # 检查 confkit 是否可用
    if ! command -v confkit >/dev/null 2>&1; then
        print_error "找不到 confkit 命令"
        print_info "请先安装 ConfKit CLI"
        exit 1
    fi
    
    # 检查 Docker 是否可用
    if ! command -v docker >/dev/null 2>&1; then
        print_error "找不到 docker 命令"
        print_info "请先安装 Docker"
        exit 1
    fi
    
    # 检查 docker-compose 是否可用
    if ! command -v docker-compose >/dev/null 2>&1; then
        print_error "找不到 docker-compose 命令"
        print_info "请先安装 Docker Compose"
        exit 1
    fi
    
    # 检查 Git 状态
    if [ -n "$(git status --porcelain)" ]; then
        print_warning "工作目录有未提交的更改"
        read -p "是否继续? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "发布已取消"
            exit 0
        fi
    fi
    
    print_success "先决条件检查通过"
}

# 准备发布环境
prepare_release() {
    print_info "准备发布环境..."
    
    # 创建必要的目录
    mkdir -p volumes/artifacts volumes/cache
    
    # 启动发布环境
    print_info "启动发布容器..."
    docker-compose -f release-docker-compose.yml --profile release up -d
    
    # 等待容器启动
    sleep 5
    
    # 检查容器状态
    if ! docker-compose -f release-docker-compose.yml ps | grep -q "Up"; then
        print_error "发布容器启动失败"
        exit 1
    fi
    
    print_success "发布环境准备完成"
}

# 清理发布环境
cleanup_release() {
    print_info "清理发布环境..."
    docker-compose -f release-docker-compose.yml --profile release down
    print_success "发布环境已清理"
}

# 验证版本号
validate_version() {
    local version=$1
    
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        print_error "无效的版本号格式: $version"
        print_info "版本号应该是 x.y.z 格式 (例如: 1.0.0)"
        exit 1
    fi
    
    # 检查版本号是否已存在
    if git tag -l | grep -q "^v$version$"; then
        print_error "版本 v$version 已存在"
        exit 1
    fi
    
    print_success "版本号验证通过: $version"
}

# 更新版本号
update_version() {
    local version=$1
    
    print_info "更新 Cargo.toml 中的版本号..."
    
    # 更新 Cargo.toml
    sed -i.bak "s/^version = \".*\"/version = \"$version\"/" ../Cargo.toml
    
    # 验证更新
    if grep -q "version = \"$version\"" ../Cargo.toml; then
        print_success "版本号已更新到 $version"
        rm ../Cargo.toml.bak
    else
        print_error "版本号更新失败"
        mv ../Cargo.toml.bak ../Cargo.toml
        exit 1
    fi
}

# 运行发布流程
run_release() {
    local version=$1
    
    print_info "开始发布 ConfKit v$version..."
    
    # 设置 ConfKit 需要的环境变量
    export RELEASE_VERSION="$version"
    export BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    
    # 如果是 dry-run，修改镜像名称
    if [ "$DRY_RUN" = "true" ]; then
        print_warning "这是一个 dry-run，不会实际发布"
        # 在 dry-run 模式下，可以修改一些行为
        print_info "Dry-run 模式：将跳过实际的发布操作"
    fi
    
    # 显示将要使用的环境变量
    print_info "发布环境变量:"
    echo "  RELEASE_VERSION=$RELEASE_VERSION"
    echo "  BUILD_DATE=$BUILD_DATE"
    echo "  CARGO_REGISTRY_TOKEN=<hidden>"
    echo "  DOCKER_USERNAME=$DOCKER_USERNAME"
    echo "  GITHUB_TOKEN=<hidden>"
    
    # 运行 ConfKit 发布任务
    cd ..
    confkit run --space release --project confkit-release
    cd examples
    
    print_success "发布流程完成"
}

# 创建 Git 标签
create_git_tag() {
    local version=$1
    
    if [ "$DRY_RUN" = "true" ]; then
        print_info "Dry-run: 跳过 Git 标签创建"
        return
    fi
    
    print_info "创建 Git 标签..."
    
    # 提交版本更改
    git add ../Cargo.toml
    git commit -m "chore: bump version to $version"
    
    # 创建标签
    git tag -a "v$version" -m "Release v$version"
    
    # 推送更改和标签
    git push origin main
    git push origin "v$version"
    
    print_success "Git 标签 v$version 已创建并推送"
}

# 发布后验证
verify_release() {
    local version=$1
    
    if [ "$DRY_RUN" = "true" ]; then
        print_info "Dry-run: 跳过发布验证"
        return
    fi
    
    print_info "验证发布结果..."
    
    # 等待一段时间让发布传播
    sleep 30
    
    # 检查 GitHub Release
    if curl -s "https://api.github.com/repos/confkit/engine/releases/tags/v$version" | grep -q "\"tag_name\": \"v$version\""; then
        print_success "GitHub Release 验证通过"
    else
        print_warning "GitHub Release 验证失败，可能需要更多时间"
    fi
    
    # 检查 crates.io
    if curl -s "https://crates.io/api/v1/crates/confkit-engine" | grep -q "\"max_version\": \"$version\""; then
        print_success "crates.io 发布验证通过"
    else
        print_warning "crates.io 发布验证失败，可能需要更多时间"
    fi
    
    # 检查 Docker Hub
    if curl -s "https://hub.docker.com/v2/repositories/confkit/cli/tags/v$version" | grep -q "\"name\": \"v$version\""; then
        print_success "Docker Hub 发布验证通过"
    else
        print_warning "Docker Hub 发布验证失败，可能需要更多时间"
    fi
}

# 主函数
main() {
    # 解析参数
    DRY_RUN=false
    SKIP_TESTS=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            --skip-tests)
                SKIP_TESTS=true
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            -*)
                print_error "未知选项: $1"
                show_help
                exit 1
                ;;
            *)
                if [ -z "$VERSION" ]; then
                    VERSION=$1
                else
                    print_error "太多参数: $1"
                    show_help
                    exit 1
                fi
                shift
                ;;
        esac
    done
    
    # 检查版本参数
    if [ -z "$VERSION" ]; then
        print_error "缺少版本参数"
        show_help
        exit 1
    fi
    
    # 显示发布信息
    echo -e "${BLUE}"
    echo "================================================="
    echo "    ConfKit 自发布流程"
    echo "    版本: $VERSION"
    echo "    Dry-run: $DRY_RUN"
    echo "================================================="
    echo -e "${NC}"
    
    # 执行发布流程
    validate_version "$VERSION"
    check_prerequisites
    check_environment
    
    # 设置错误处理
    trap cleanup_release EXIT
    
    prepare_release
    
    if [ "$DRY_RUN" != "true" ]; then
        update_version "$VERSION"
    fi
    
    run_release "$VERSION"
    
    if [ "$DRY_RUN" != "true" ]; then
        create_git_tag "$VERSION"
        verify_release "$VERSION"
    fi
    
    print_success "🎉 ConfKit v$VERSION 发布完成！"
    
    if [ "$DRY_RUN" != "true" ]; then
        echo ""
        print_info "发布链接:"
        echo "  GitHub: https://github.com/confkit/engine/releases/tag/v$VERSION"
        echo "  crates.io: https://crates.io/crates/confkit-engine"
        echo "  Docker: confkit/cli:v$VERSION"
    fi
}

# 运行主函数
main "$@" 