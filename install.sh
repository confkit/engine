#!/bin/sh
set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 获取操作系统
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

# 获取架构
ARCH=$(uname -m)

# 将架构名称标准化
case "$ARCH" in
  x86_64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) 
    printf "${RED}错误: 不支持的架构 $ARCH${NC}\n" >&2
    exit 1 
    ;;
esac

# 根据操作系统设置目标平台和安装目录
case "$OS" in
  linux)
    TARGET="${ARCH}-unknown-linux-gnu"
    # Linux: 优先使用 /usr/local/bin，如果没有权限则使用 ~/.local/bin
    if [ -w "/usr/local/bin" ] || [ "$(id -u)" = "0" ]; then
      INSTALL_DIR="/usr/local/bin"
      USE_SUDO="sudo"
    else
      INSTALL_DIR="$HOME/.local/bin"
      USE_SUDO=""
      mkdir -p "$INSTALL_DIR"
    fi
    SHELL_RC="$HOME/.bashrc"
    if [ -f "$HOME/.zshrc" ]; then
      SHELL_RC="$HOME/.zshrc"
    fi
    ;;
  darwin)
    TARGET="${ARCH}-apple-darwin"
    # macOS: 使用 /usr/local/bin（Homebrew 标准）
    INSTALL_DIR="/usr/local/bin"
    USE_SUDO="sudo"
    SHELL_RC="$HOME/.zshrc"
    if [ -f "$HOME/.bash_profile" ]; then
      SHELL_RC="$HOME/.bash_profile"
    fi
    ;;
  *)
    printf "${RED}错误: 不支持的操作系统 $OS${NC}\n" >&2
    exit 1
    ;;
esac

# 构建下载 URL
URL="https://github.com/confkit/engine/releases/latest/download/confkit-${TARGET}"

printf "${BLUE}正在下载 confkit for ${TARGET}...${NC}\n"
printf "${BLUE}下载地址: $URL${NC}\n"
printf "${BLUE}安装目录: $INSTALL_DIR${NC}\n"

# 创建临时文件
TEMP_FILE=$(mktemp)

# 下载二进制文件
if ! curl -fsSL "$URL" -o "$TEMP_FILE"; then
  printf "${RED}错误: 下载失败\n可能原因:\n1. 网络连接问题\n2. 指定的版本不存在\n3. GitHub 服务不可用${NC}\n" >&2
  rm -f "$TEMP_FILE"
  exit 1
fi

# 检查文件是否为空或是否为有效的二进制文件
if [ ! -s "$TEMP_FILE" ]; then
  printf "${RED}错误: 下载的文件为空${NC}\n" >&2
  rm -f "$TEMP_FILE"
  exit 1
fi

# 检查文件是否为 HTML（通常意味着 404 错误）
if head -c 100 "$TEMP_FILE" | grep -q "<html\|<HTML"; then
  printf "${RED}错误: 下载失败，文件不存在或版本不匹配${NC}\n" >&2
  rm -f "$TEMP_FILE"
  exit 1
fi

# 确保目标目录存在
if [ -n "$USE_SUDO" ]; then
  $USE_SUDO mkdir -p "$INSTALL_DIR"
else
  mkdir -p "$INSTALL_DIR"
fi

# 安装二进制文件
if [ -n "$USE_SUDO" ]; then
  $USE_SUDO mv "$TEMP_FILE" "$INSTALL_DIR/confkit"
  $USE_SUDO chmod +x "$INSTALL_DIR/confkit"
else
  mv "$TEMP_FILE" "$INSTALL_DIR/confkit"
  chmod +x "$INSTALL_DIR/confkit"
fi

printf "${GREEN}✓ confkit 已成功安装到 $INSTALL_DIR/confkit${NC}\n"
printf "${GREEN}检测到的平台: ${TARGET}${NC}\n"

# 检查 PATH 并添加安装目录（如果需要）
PATH_UPDATED=false
case ":$PATH:" in
  *":$INSTALL_DIR:"*)
    printf "${GREEN}✓ $INSTALL_DIR 已在 PATH 中${NC}\n"
    ;;
  *)
    # 尝试添加到 PATH
    if [ -f "$SHELL_RC" ]; then
      if ! grep -q "$INSTALL_DIR" "$SHELL_RC" 2>/dev/null; then
        printf "\n# Added by confkit installer\nexport PATH=\"$INSTALL_DIR:\$PATH\"\n" >> "$SHELL_RC"
        PATH_UPDATED=true
        printf "${GREEN}✓ 已将 $INSTALL_DIR 添加到 $SHELL_RC${NC}\n"
        printf "${YELLOW}请运行以下命令或重新启动终端生效:\n  source $SHELL_RC${NC}\n"
      else
        printf "${GREEN}✓ $INSTALL_DIR 已在 $SHELL_RC 中配置${NC}\n"
      fi
    else
      printf "${YELLOW}警告: 未找到 shell 配置文件${NC}\n"
      printf "${YELLOW}请手动将以下命令添加到您的 shell 配置文件中:\n  export PATH=\"$INSTALL_DIR:\$PATH\"${NC}\n"
    fi
    ;;
esac

# 验证安装
if command -v confkit >/dev/null 2>&1; then
  printf "\n${GREEN}正在验证安装...${NC}\n"
  confkit --help
  printf "\n${GREEN}🎉 confkit 安装成功！${NC}\n"
else
  printf "\n${YELLOW}安装完成，但 confkit 尚未在当前会话的 PATH 中。${NC}\n"
  if [ "$PATH_UPDATED" = "true" ]; then
    printf "${YELLOW}请运行以下命令使 PATH 生效:\n  source $SHELL_RC${NC}\n"
  else
    printf "${YELLOW}请手动将 $INSTALL_DIR 添加到您的 PATH 中。${NC}\n"
  fi
  printf "${YELLOW}或者您可以直接运行:\n  $INSTALL_DIR/confkit --help${NC}\n"
fi
