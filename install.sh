#!/bin/bash

# ConfKit CLI 一键安装脚本
# 支持 Linux (x86_64, ARM64) 和 macOS (Intel, Apple Silicon)

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置
REPO="confkit/engine"
BINARY_NAME="confkit"
INSTALL_DIR="/usr/local/bin"
TEMP_DIR="/tmp/confkit-install"

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

# 检测操作系统和架构
detect_platform() {
    local os=$(uname -s)
    local arch=$(uname -m)
    
    case $os in
        Linux)
            OS="linux"
            ;;
        Darwin)
            OS="darwin"
            ;;
        *)
            print_error "不支持的操作系统: $os"
            exit 1
            ;;
    esac
    
    case $arch in
        x86_64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            print_error "不支持的架构: $arch"
            exit 1
            ;;
    esac
    
    # 构建目标平台标识
    case $OS in
        linux)
            TARGET="${ARCH}-unknown-linux-gnu"
            ;;
        darwin)
            if [ "$ARCH" = "aarch64" ]; then
                TARGET="aarch64-apple-darwin"
            else
                TARGET="x86_64-apple-darwin"
            fi
            ;;
    esac
    
    print_info "检测到平台: $OS $ARCH ($TARGET)"
}

# 检查必要工具
check_dependencies() {
    local missing_tools=()
    
    if ! command -v curl >/dev/null 2>&1; then
        missing_tools+=("curl")
    fi
    
    if ! command -v tar >/dev/null 2>&1; then
        missing_tools+=("tar")
    fi
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        print_error "缺少必要工具: ${missing_tools[*]}"
        print_info "请安装这些工具后重试"
        exit 1
    fi
}

# 获取最新版本
get_latest_version() {
    print_info "获取最新版本信息..."
    
    local api_url="https://api.github.com/repos/${REPO}/releases/latest"
    VERSION=$(curl -s "$api_url" | grep '"tag_name"' | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')
    
    if [ -z "$VERSION" ]; then
        print_error "无法获取最新版本信息"
        exit 1
    fi
    
    print_success "最新版本: $VERSION"
}

# 下载二进制文件
download_binary() {
    local filename="confkit-${TARGET}.tar.gz"
    local download_url="https://github.com/${REPO}/releases/download/${VERSION}/${filename}"
    
    print_info "创建临时目录..."
    mkdir -p "$TEMP_DIR"
    cd "$TEMP_DIR"
    
    print_info "下载 $filename..."
    print_info "下载地址: $download_url"
    
    if ! curl -fsSL "$download_url" -o "$filename"; then
        print_error "下载失败"
        exit 1
    fi
    
    print_success "下载完成"
}

# 解压并安装
install_binary() {
    print_info "解压文件..."
    tar -xzf "confkit-${TARGET}.tar.gz"
    
    # 检查二进制文件是否存在
    if [ ! -f "$BINARY_NAME" ]; then
        print_error "二进制文件不存在"
        exit 1
    fi
    
    print_info "安装到 $INSTALL_DIR..."
    
    # 检查是否需要 sudo
    if [ -w "$INSTALL_DIR" ]; then
        mv "$BINARY_NAME" "$INSTALL_DIR/"
    else
        print_warning "需要管理员权限安装到 $INSTALL_DIR"
        sudo mv "$BINARY_NAME" "$INSTALL_DIR/"
    fi
    
    # 设置执行权限
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    
    print_success "安装完成"
}

# 清理临时文件
cleanup() {
    if [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
        print_info "清理临时文件"
    fi
}

# 验证安装
verify_installation() {
    print_info "验证安装..."
    
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        local installed_version=$("$BINARY_NAME" --version 2>/dev/null | head -n1 || echo "unknown")
        print_success "安装成功! 版本: $installed_version"
        
        # 显示使用提示
        echo ""
        print_info "使用说明:"
        echo "  $BINARY_NAME --help          # 查看帮助"
        echo "  $BINARY_NAME interactive     # 交互式模式"
        echo "  $BINARY_NAME builder list    # 列出构建器"
        echo ""
        print_info "完整文档: https://github.com/${REPO}"
    else
        print_error "安装失败: 找不到 $BINARY_NAME 命令"
        print_info "请检查 $INSTALL_DIR 是否在 PATH 中"
        exit 1
    fi
}

# 主函数
main() {
    echo -e "${BLUE}"
    echo "================================================="
    echo "    ConfKit CLI 一键安装脚本"
    echo "    Configuration-driven Build Tool"
    echo "================================================="
    echo -e "${NC}"
    
    # 检查参数
    if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        echo "使用方法: $0 [选项]"
        echo ""
        echo "选项:"
        echo "  --help, -h     显示帮助信息"
        echo "  --version, -v  指定版本 (默认: 最新版本)"
        echo ""
        echo "示例:"
        echo "  $0              # 安装最新版本"
        echo "  $0 --version v1.0.0  # 安装指定版本"
        exit 0
    fi
    
    # 处理版本参数
    if [ "$1" = "--version" ] || [ "$1" = "-v" ]; then
        if [ -z "$2" ]; then
            print_error "请指定版本号"
            exit 1
        fi
        VERSION="$2"
        print_info "指定版本: $VERSION"
    fi
    
    # 执行安装流程
    detect_platform
    check_dependencies
    
    # 如果没有指定版本，获取最新版本
    if [ -z "$VERSION" ]; then
        get_latest_version
    fi
    
    download_binary
    install_binary
    cleanup
    verify_installation
    
    print_success "ConfKit CLI 安装完成!"
}

# 错误处理
trap cleanup EXIT

# 运行主函数
main "$@" 