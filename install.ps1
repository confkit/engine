# ConfKit CLI Windows 一键安装脚本
# 支持 Windows x86_64 平台

param(
    [string]$Version = "",
    [switch]$Help
)

# 配置
$Repo = "confkit/engine"
$BinaryName = "confkit.exe"
$InstallDir = "$env:LOCALAPPDATA\confkit"
$TempDir = "$env:TEMP\confkit-install"
$Target = "x86_64-pc-windows-msvc"

# 颜色定义
$Colors = @{
    Red = "Red"
    Green = "Green"
    Yellow = "Yellow"
    Blue = "Blue"
    White = "White"
}

# 打印彩色消息
function Write-Info {
    param([string]$Message)
    Write-Host "ℹ $Message" -ForegroundColor $Colors.Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "✓ $Message" -ForegroundColor $Colors.Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "⚠ $Message" -ForegroundColor $Colors.Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "✗ $Message" -ForegroundColor $Colors.Red
}

# 显示帮助信息
function Show-Help {
    Write-Host @"
ConfKit CLI Windows 安装脚本

使用方法: .\install.ps1 [选项]

选项:
  -Version <版本>    指定版本 (默认: 最新版本)
  -Help              显示帮助信息

示例:
  .\install.ps1                 # 安装最新版本
  .\install.ps1 -Version v1.0.0 # 安装指定版本

注意:
  - 需要 PowerShell 5.1 或更高版本
  - 需要管理员权限来修改 PATH 环境变量
"@
}

# 检查 PowerShell 版本
function Test-PowerShellVersion {
    if ($PSVersionTable.PSVersion.Major -lt 5) {
        Write-Error "需要 PowerShell 5.1 或更高版本"
        Write-Info "当前版本: $($PSVersionTable.PSVersion)"
        exit 1
    }
}

# 检查必要工具
function Test-Dependencies {
    $missingTools = @()
    
    try {
        Invoke-WebRequest -Uri "https://httpbin.org/get" -Method Head -UseBasicParsing | Out-Null
    } catch {
        $missingTools += "Web Request capability"
    }
    
    if ($missingTools.Count -gt 0) {
        Write-Error "缺少必要功能: $($missingTools -join ', ')"
        Write-Info "请确保可以访问互联网并允许 PowerShell 执行 Web 请求"
        exit 1
    }
}

# 获取最新版本
function Get-LatestVersion {
    Write-Info "获取最新版本信息..."
    
    $apiUrl = "https://api.github.com/repos/$Repo/releases/latest"
    
    try {
        $response = Invoke-RestMethod -Uri $apiUrl -UseBasicParsing
        $script:Version = $response.tag_name
        Write-Success "最新版本: $script:Version"
    } catch {
        Write-Error "无法获取最新版本信息: $($_.Exception.Message)"
        exit 1
    }
}

# 下载二进制文件
function Get-Binary {
    $filename = "confkit-$Target.zip"
    $downloadUrl = "https://github.com/$Repo/releases/download/$Version/$filename"
    
    Write-Info "创建临时目录..."
    if (Test-Path $TempDir) {
        Remove-Item -Path $TempDir -Recurse -Force
    }
    New-Item -ItemType Directory -Path $TempDir | Out-Null
    
    $zipPath = Join-Path $TempDir $filename
    
    Write-Info "下载 $filename..."
    Write-Info "下载地址: $downloadUrl"
    
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
        Write-Success "下载完成"
    } catch {
        Write-Error "下载失败: $($_.Exception.Message)"
        exit 1
    }
    
    return $zipPath
}

# 解压并安装
function Install-Binary {
    param([string]$ZipPath)
    
    Write-Info "解压文件..."
    try {
        Expand-Archive -Path $ZipPath -DestinationPath $TempDir -Force
    } catch {
        Write-Error "解压失败: $($_.Exception.Message)"
        exit 1
    }
    
    $binaryPath = Join-Path $TempDir $BinaryName
    if (-not (Test-Path $binaryPath)) {
        Write-Error "二进制文件不存在: $binaryPath"
        exit 1
    }
    
    Write-Info "安装到 $InstallDir..."
    
    # 创建安装目录
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir | Out-Null
    }
    
    # 复制二进制文件
    $targetPath = Join-Path $InstallDir $BinaryName
    Copy-Item -Path $binaryPath -Destination $targetPath -Force
    
    Write-Success "安装完成"
    return $targetPath
}

# 更新 PATH 环境变量
function Update-Path {
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    
    if ($currentPath -notlike "*$InstallDir*") {
        Write-Info "更新 PATH 环境变量..."
        
        try {
            $newPath = "$InstallDir;$currentPath"
            [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
            
            # 更新当前会话的 PATH
            $env:PATH = "$InstallDir;$env:PATH"
            
            Write-Success "PATH 更新完成"
        } catch {
            Write-Warning "更新 PATH 失败: $($_.Exception.Message)"
            Write-Info "请手动将 $InstallDir 添加到 PATH 环境变量"
        }
    } else {
        Write-Info "PATH 已包含安装目录"
    }
}

# 清理临时文件
function Remove-TempFiles {
    if (Test-Path $TempDir) {
        Remove-Item -Path $TempDir -Recurse -Force
        Write-Info "清理临时文件"
    }
}

# 验证安装
function Test-Installation {
    Write-Info "验证安装..."
    
    try {
        $output = & $BinaryName --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Success "安装成功! 版本: $output"
            
            # 显示使用提示
            Write-Host ""
            Write-Info "使用说明:"
            Write-Host "  $BinaryName --help          # 查看帮助"
            Write-Host "  $BinaryName interactive     # 交互式模式"
            Write-Host "  $BinaryName builder list    # 列出构建器"
            Write-Host ""
            Write-Info "完整文档: https://github.com/$Repo"
            Write-Host ""
            Write-Warning "重要提示: 请重新启动 PowerShell 或命令提示符以使 PATH 更改生效"
        } else {
            Write-Error "命令执行失败: $output"
            Test-PathAccess
        }
    } catch {
        Write-Error "找不到 $BinaryName 命令"
        Test-PathAccess
    }
}

# 检查 PATH 访问
function Test-PathAccess {
    Write-Info "检查 PATH 配置..."
    
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    $systemPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
    
    Write-Info "用户 PATH: $userPath"
    Write-Info "系统 PATH: $systemPath"
    Write-Info "当前 PATH: $env:PATH"
    
    if ($env:PATH -notlike "*$InstallDir*") {
        Write-Warning "安装目录不在 PATH 中"
        Write-Info "请手动运行: $InstallDir\$BinaryName"
    }
}

# 主函数
function Main {
    # 显示标题
    Write-Host @"
=================================================
    ConfKit CLI Windows 一键安装脚本
    Configuration-driven Build Tool
=================================================
"@ -ForegroundColor $Colors.Blue

    # 检查参数
    if ($Help) {
        Show-Help
        exit 0
    }
    
    # 执行安装流程
    try {
        Test-PowerShellVersion
        Test-Dependencies
        
        # 如果没有指定版本，获取最新版本
        if (-not $Version) {
            Get-LatestVersion
        } else {
            Write-Info "指定版本: $Version"
        }
        
        $zipPath = Get-Binary
        Install-Binary -ZipPath $zipPath
        Update-Path
        Remove-TempFiles
        Test-Installation
        
        Write-Success "ConfKit CLI 安装完成!"
    } catch {
        Write-Error "安装过程中发生错误: $($_.Exception.Message)"
        Remove-TempFiles
        exit 1
    }
}

# 错误处理
trap {
    Write-Error "发生未预期的错误: $($_.Exception.Message)"
    Remove-TempFiles
    exit 1
}

# 运行主函数
Main 