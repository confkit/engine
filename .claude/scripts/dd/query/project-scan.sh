#!/bin/bash

# 项目快速扫描脚本
# 支持 --confirm 参数启用用户确认模式

PROJECT_ROOT=$(pwd)
CONFIRM_MODE=false

# 解析命令行参数
if [ "$1" = "--confirm" ]; then
    CONFIRM_MODE=true
fi

echo "# 项目快速扫描报告"
echo "项目: $(basename "$PROJECT_ROOT")"
echo ""

# 1. 技术栈识别
echo "## 技术栈"
if [ -f "package.json" ]; then
    echo "✅ **Node.js**"
fi
if [ -f "requirements.txt" ] || [ -f "pyproject.toml" ] || [ -f "setup.py" ]; then
    echo "✅ **Python**"
fi
if [ -f "go.mod" ]; then
    echo "✅ **Go**"
fi
if [ -f "Cargo.toml" ]; then
    echo "✅ **Rust**"
fi
if [ -f "pom.xml" ] || [ -f "build.gradle" ]; then
    echo "✅ **Java**"
fi
echo ""

# 2. 项目结构 (排除构建产物和.claude目录)
echo "## 项目结构"
echo '```'
if command -v tree >/dev/null 2>&1; then
    # 显示文件和目录, 限制深度避免输出过多
    tree -a -L 4 -I 'node_modules|.git|target|dist|build|__pycache__|.venv|venv|*.pyc|coverage|.claude'
else
    # 备选方案, 显示文件和目录
    find . -not -path '*/node_modules*' -not -path '*/.git*' -not -path '*/target*' -not -path '*/dist*' -not -path '*/build*' -not -path '*/__pycache__*' -not -path '*/.venv*' -not -path '*/venv*' -not -path '*/.claude*' -maxdepth 4 | sort
fi
echo '```'
echo ""

# 3. 源码统计
echo "## 源码文件"
total_lines=0
for ext in js ts jsx tsx py go rs java cpp c h php rb; do
    files=$(find . -name "*.$ext" -not -path '*/node_modules/*' -not -path '*/.git/*' -not -path '*/target/*' -not -path '*/dist/*' -not -path '*/build/*' -not -path '*/__pycache__/*' -not -path '*/.venv/*' -not -path '*/venv/*' -type f | wc -l | tr -d ' ')
    if [ "$files" -gt 0 ]; then
        lines=$(find . -name "*.$ext" -not -path '*/node_modules/*' -not -path '*/.git/*' -not -path '*/target/*' -not -path '*/dist/*' -not -path '*/build/*' -not -path '*/__pycache__/*' -not -path '*/.venv/*' -not -path '*/venv/*' -type f -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
        echo "- .$ext: $files 个文件, $lines 行代码"
        total_lines=$((total_lines + lines))
    fi
done
echo "- **总计**: $total_lines 行代码"
echo ""

# 4. 重要文件
echo "## 配置文件"
important_files="package.json requirements.txt pyproject.toml go.mod Cargo.toml pom.xml build.gradle Dockerfile docker-compose.yml .env .gitignore README.md"
for file in $important_files; do
    if [ -f "$file" ]; then
        echo "- $file"
    fi
done
echo ""

# 5. Git 状态
echo "## Git 状态"
if [ -d ".git" ]; then
    if command -v git >/dev/null 2>&1; then
        branch=$(git branch --show-current 2>/dev/null || echo '未知')
        uncommitted=$(git status --porcelain 2>/dev/null | wc -l | tr -d ' ')
        echo "- 当前分支: $branch"
        echo "- 未提交文件: $uncommitted 个"
        last_commit=$(git log -1 --format='%h %s' 2>/dev/null | head -c 50)
        if [ -n "$last_commit" ]; then
            echo "- 最近提交: $last_commit"
        fi
    fi
else
    echo "- 未初始化 Git 仓库"
fi
echo ""

# 6. 项目规模
echo "## 项目规模"
all_files=$(find . -type f -not -path '*/.git/*' | wc -l | tr -d ' ')
source_files=$(find . -name "*.js" -o -name "*.ts" -o -name "*.jsx" -o -name "*.tsx" -o -name "*.py" -o -name "*.go" -o -name "*.rs" -o -name "*.java" -o -name "*.cpp" -o -name "*.c" -o -name "*.h" | grep -v node_modules | grep -v target | grep -v build | grep -v dist | wc -l | tr -d ' ')
echo "- 总文件数: $all_files"
echo "- 源码文件数: $source_files"

# 项目规模评估
if [ "$source_files" -lt 50 ]; then
    echo "- 规模评估: 小型项目"
elif [ "$source_files" -lt 200 ]; then
    echo "- 规模评估: 中型项目"  
else
    echo "- 规模评估: 大型项目"
fi
echo ""

# 7. 潜在问题
echo "## 检查结果"
issues_found=false

# 依赖和锁文件检查
if [ -f "package.json" ] && [ ! -f "package-lock.json" ] && [ ! -f "yarn.lock" ] && [ ! -f "pnpm-lock.yaml" ]; then
    echo "⚠️ Node.js: 缺少依赖锁文件"
    issues_found=true
fi

# Python 缓存检查
if [ -d "__pycache__" ] && [ ! -f ".gitignore" ]; then
    echo "⚠️ Python: 发现缓存目录, 建议添加到 .gitignore"
    issues_found=true
fi

# Rust target 检查
if [ -f "Cargo.toml" ] && [ -d "target" ] && [ ! -f ".gitignore" ]; then
    echo "⚠️ Rust: target 目录应该被 .gitignore 排除"
    issues_found=true
fi

# 环境文件检查
if find . -maxdepth 2 -name ".env*" -type f | grep -q .; then
    echo "⚠️ 发现环境配置文件, 注意保护敏感信息"
    issues_found=true
fi

if [ "$issues_found" = false ]; then
    echo "✅ 项目结构良好"
fi