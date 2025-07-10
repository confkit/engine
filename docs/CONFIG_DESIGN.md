 # confkit CLI 配置文档

本文档详细介绍了confkit CLI的配置文件格式、参数设置和最佳实践。

## 📋 目录

- [项目配置文件](#项目配置文件)
- [环境变量](#环境变量)
- [参数优先级](#参数优先级)
- [配置示例](#配置示例)
- [最佳实践](#最佳实践)

## 📄 配置文件

### 基本结构

```
builder.yml
docker-compose.yml
.confkit/
├──builders/
│   ├── golang
│   │   ├── Dockerfile.1.22
│   │   ├── Dockerfile.1.24
│   │   └── ...
│   ├── rust
│   │   ├── Dockerfile.1.28.1
│   │   └── ...
│   └── node
│       ├── Dockerfile.22
│       └── ...
└──spaces 
    ├── space_01
    │   ├── config.yml
    │   ├── projects
    │   │   ├── project_01.yml
    │   │   ├── project_02.yml
    │   │   └── ...
    └── space_02
        ├── project_01.yml
        ├── project_02.yml
        └── ...
```

### 基本结构

```yaml
# .confkit/build.yml 或 projects/my-project.yml
project:
  name: "项目名称"
  type: "项目类型"
  description: "项目描述"

source:
  git_repo: "Git仓库地址"
  git_branch: "分支名称"
  git_tag: "标签名称（可选）"

environment:
  # 环境变量定义
  
steps:
  # 构建步骤定义
  # container 是可选项，默认在宿主机执行
  # commands 是命令数组，按顺序执行

step_options:
  # 步骤默认选项

notifications:
  # 通知配置（可选）
```

### 完整示例 - Golang微服务

```yaml
# projects/microservice-api.yml
project:
  name: "microservice-api"
  type: "golang"
  description: "微服务API构建和部署"
  version: "1.0.0"

source:
  git_repo: "https://github.com/company/microservice-api.git"
  git_branch: "main"
  clone_depth: 1  # 浅克隆深度

environment:
  # Go 编译环境
  CGO_ENABLED: "0"
  GOOS: "linux"
  GOARCH: "amd64"
  
  # 应用配置
  API_VERSION: "${GIT_TAG:-v1.0.0}"
  BUILD_TIME: "$(date -Iseconds)"
  
  # Docker配置
  DOCKER_REGISTRY: "registry.company.com"
  IMAGE_NAME: "${PROJECT_NAME}"

steps:
  # 代码准备阶段
  - name: "代码拉取"
    # container 省略，默认在宿主机执行
    working_dir: "./volumes/workspace"
    commands:
      - "rm -rf ${PROJECT_NAME} || true"
      - "git clone --depth=${CLONE_DEPTH:-1} ${GIT_REPO} ${PROJECT_NAME}"
      - "cd ${PROJECT_NAME}"
      - "git checkout ${GIT_BRANCH}"
      - "echo \"代码拉取完成，commit: $(git rev-parse HEAD)\""
    retry: 3
    timeout: "5m"

  - name: "依赖检查"
    container: "golang-builder-1.24"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"验证 Go 模块...\""
      - "go mod verify"
      - "echo \"下载依赖...\""
      - "go mod download"
      - "echo \"依赖检查完成\""
    depends_on: ["代码拉取"]
    
  # 质量检查阶段
  - name: "代码检查"
    container: "golang-builder-1.24"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"运行 golangci-lint...\""
      - "golangci-lint run --timeout=10m --out-format=json > lint-report.json || true"
      - "cat lint-report.json"
    depends_on: ["依赖检查"]
    continue_on_error: true
    
  - name: "单元测试"
    container: "golang-builder-1.24"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"运行单元测试...\""
      - "go test -v -race -coverprofile=coverage.out ./..."
      - "go tool cover -html=coverage.out -o coverage.html"
      - "echo \"测试覆盖率：$(go tool cover -func=coverage.out | tail -1)\""
    depends_on: ["依赖检查"]
    parallel_group: "testing"
    
  - name: "基准测试"
    container: "golang-builder-1.24"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"运行基准测试...\""
      - "go test -bench=. -benchmem ./... > benchmark.txt"
      - "cat benchmark.txt"
    depends_on: ["依赖检查"]
    parallel_group: "testing"
    continue_on_error: true
    
  # 构建阶段
  - name: "二进制构建"
    container: "golang-builder-1.24"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"构建二进制文件...\""
      - "go build -ldflags \"-X main.version=${API_VERSION} -X main.buildTime=${BUILD_TIME}\" -o ${PROJECT_NAME} ./cmd/server"
      - "echo \"构建完成，文件大小: $(du -h ${PROJECT_NAME})\""
      - "./${PROJECT_NAME} --version"
    depends_on: ["代码检查", "单元测试"]
    
  - name: "Docker镜像构建"
    # container 省略，在宿主机执行
    working_dir: "./volumes/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"构建Docker镜像...\""
      - "docker build -t ${DOCKER_REGISTRY}/${IMAGE_NAME}:${API_VERSION} -t ${DOCKER_REGISTRY}/${IMAGE_NAME}:${TASK_ID} -t ${DOCKER_REGISTRY}/${IMAGE_NAME}:latest --build-arg VERSION=${API_VERSION} --build-arg BUILD_TIME=\"${BUILD_TIME}\" ."
      - "echo \"镜像构建完成\""
      - "docker images ${DOCKER_REGISTRY}/${IMAGE_NAME}"
    depends_on: ["二进制构建"]
    
  # 产物管理阶段
  - name: "产物收集"
    container: "golang-builder-1.24"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"收集构建产物...\""
      - "mkdir -p /artifacts/${TASK_ID}"
      - "cp ${PROJECT_NAME} /artifacts/${TASK_ID}/"
      - "cp coverage.out /artifacts/${TASK_ID}/ || true"
      - "cp coverage.html /artifacts/${TASK_ID}/ || true"
      - "cp lint-report.json /artifacts/${TASK_ID}/ || true"
      - "cp benchmark.txt /artifacts/${TASK_ID}/ || true"
      - "cp Dockerfile /artifacts/${TASK_ID}/"
      - "echo \"产物收集完成\""
      - "ls -la /artifacts/${TASK_ID}/"
    depends_on: ["二进制构建"]
    
  - name: "构建信息记录"
    # container 省略，在宿主机执行
    working_dir: "./artifacts/${TASK_ID}"
    commands:
      - "echo \"记录构建信息...\""
      - |
        cat > build-info.json << 'EOF'
        {
          "task_id": "${TASK_ID}",
          "project": "${PROJECT_NAME}",
          "version": "${API_VERSION}",
          "git_repo": "${GIT_REPO}",
          "git_branch": "${GIT_BRANCH}",
          "git_commit": "${GIT_COMMIT_HASH}",
          "git_commit_short": "${GIT_COMMIT_SHORT}",
          "build_time": "${BUILD_TIME}",
          "builder": "golang-builder-1.24",
          "artifacts": [
            "${PROJECT_NAME}",
            "coverage.out",
            "coverage.html", 
            "lint-report.json",
            "benchmark.txt",
            "Dockerfile"
          ],
          "docker_images": [
            "${DOCKER_REGISTRY}/${IMAGE_NAME}:${API_VERSION}",
            "${DOCKER_REGISTRY}/${IMAGE_NAME}:${TASK_ID}",
            "${DOCKER_REGISTRY}/${IMAGE_NAME}:latest"
          ]
        }
        EOF
      - "echo \"构建信息记录完成\""
      - "cat build-info.json"
    depends_on: ["产物收集"]

# 步骤执行选项
step_options:
  retry: 1                    # 默认重试次数
  timeout: "10m"              # 默认超时时间
  continue_on_error: false    # 默认失败时停止
  parallel: false             # 默认串行执行
  shell: "/bin/bash"          # 默认Shell

# 通知配置（可选）
notifications:
  on_success:
    - type: "webhook"
      url: "https://api.company.com/build-notify"
      method: "POST"
      headers:
        Content-Type: "application/json"
        Authorization: "Bearer ${WEBHOOK_TOKEN}"
      payload: |
        {
          "status": "success",
          "project": "${PROJECT_NAME}",
          "task_id": "${TASK_ID}",
          "version": "${API_VERSION}",
          "commit": "${GIT_COMMIT_HASH}",
          "branch": "${GIT_BRANCH}",
          "artifacts_url": "https://artifacts.company.com/${TASK_ID}",
          "build_time": "${BUILD_TIME}"
        }
        
  on_failure:
    - type: "email"
      to: ["dev@company.com", "ops@company.com"]
      subject: "构建失败: ${PROJECT_NAME} - ${TASK_ID}"
      body: |
        项目 ${PROJECT_NAME} 的构建失败了。
        
        详细信息:
        - 任务ID: ${TASK_ID}
        - 分支: ${GIT_BRANCH}
        - 提交: ${GIT_COMMIT_HASH}
        - 构建时间: ${BUILD_TIME}
        
        请查看日志: confkit logs ${PROJECT_NAME} --task-id ${TASK_ID}
        
    - type: "slack"
      webhook_url: "${SLACK_WEBHOOK_URL}"
      channel: "#ci-cd"
      message: |
        :x: 构建失败: ${PROJECT_NAME}
        分支: ${GIT_BRANCH}
        任务: ${TASK_ID}
```

### Node.js项目示例

```yaml
# projects/frontend-app.yml
project:
  name: "frontend-app"
  type: "node"
  description: "Vue.js前端应用"

source:
  git_repo: "https://github.com/company/frontend-app.git"
  git_branch: "main"

environment:
  NODE_ENV: "production"
  API_URL: "https://api.company.com"
  CDN_URL: "https://cdn.company.com"

steps:
  - name: "代码拉取"
    # container 省略，在宿主机执行
    working_dir: "./volumes/workspace"
    commands:
      - "git clone ${GIT_REPO} ${PROJECT_NAME}"
      - "cd ${PROJECT_NAME}"
      - "git checkout ${GIT_BRANCH}"
    
  - name: "依赖安装"
    container: "node-builder-22"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"安装依赖...\""
      - "pnpm install --frozen-lockfile"
      - "echo \"依赖安装完成\""
    depends_on: ["代码拉取"]
    
  - name: "代码检查"
    container: "node-builder-22"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"运行ESLint...\""
      - "pnpm lint"
      - "echo \"运行类型检查...\""
      - "pnpm type-check"
    depends_on: ["依赖安装"]
    
  - name: "单元测试"
    container: "node-builder-22"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"运行单元测试...\""
      - "pnpm test:unit --coverage"
    depends_on: ["依赖安装"]
    
  - name: "构建应用"
    container: "node-builder-22"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "echo \"构建生产版本...\""
      - "pnpm build"
      - "echo \"构建完成，文件大小统计:\""
      - "du -sh dist/*"
    depends_on: ["代码检查", "单元测试"]
    
  - name: "产物收集"
    container: "node-builder-22"
    working_dir: "/workspace/${PROJECT_NAME}"
    commands:
      - "mkdir -p /artifacts/${TASK_ID}"
      - "cp -r dist/* /artifacts/${TASK_ID}/"
      - "cp package.json /artifacts/${TASK_ID}/"
    depends_on: ["构建应用"]

step_options:
  retry: 2
  timeout: "15m"
```

## 🔧 环境变量

### 自动注入的环境变量

confkit CLI 会自动注入以下环境变量到构建环境中：

| 变量名 | 说明 | 示例值 |
|--------|------|--------|
| `TASK_ID` | 任务唯一标识 | `api-20240115-143022-a1b2c3` |
| `PROJECT_NAME` | 项目名称 | `microservice-api` |
| `GIT_REPO` | Git仓库地址 | `https://github.com/company/api.git` |
| `GIT_BRANCH` | Git分支名 | `main` |
| `GIT_TAG` | Git标签（如果有） | `v1.2.0` |
| `GIT_COMMIT_HASH` | 完整commit hash | `2373442e2de493b9f97ad6aa5e0f2845811a5e3e` |
| `GIT_COMMIT_SHORT` | 短commit hash | `2373442e` |
| `BUILD_TIME` | 构建时间 | `2024-01-15T14:30:22Z` |
| `BUILD_NUMBER` | 构建编号（自增） | `42` |
| `WORKSPACE_DIR` | 工作空间目录 | `/workspace` |
| `ARTIFACTS_DIR` | 产物目录 | `/artifacts` |

### 环境变量前缀

confkit CLI 使用以下前缀的环境变量：

- `confkit_*`: confkit CLI 系统配置
- `BUILDER_*`: 构建器相关配置
- `GIT_*`: Git 相关配置
- `CI_*`: CI/CD 环境标识

### 示例：使用环境变量

```bash
# 通过环境变量配置
export confkit_LOG_LEVEL=debug
export confkit_MAX_CONCURRENT=3
export BUILDER_GOLANG_IMAGE=golang:1.24
export GIT_TOKEN=ghp_xxxxxxxxxxxxx

# 运行构建
confkit run projects/api.yml
```

## 📊 参数优先级

confkit CLI 使用以下优先级顺序来确定配置值（从高到低）：

1. **命令行参数** (最高优先级)
   ```bash
   confkit run --git-branch develop --parallel
   ```

2. **环境变量**
   ```bash
   export confkit_GIT_BRANCH=develop
   export confkit_PARALLEL=true
   ```

3. **项目配置文件**
   ```yaml
   source:
     git_branch: "main"
   step_options:
     parallel: false
   ```

4. **默认值** (最低优先级)

### 优先级示例

```yaml
# 项目配置文件
step_options:
  timeout: "10m"

# 环境变量
export confkit_TIMEOUT=5m

# 命令行参数
confkit run --timeout 2m projects/api.yml
```

最终使用的超时时间为 `2m`（命令行参数优先级最高）。

## 📚 配置示例

### 多环境部署配置

```yaml
# projects/api-prod.yml
project:
  name: "api"
  type: "golang"
  environment: "production"

environment:
  DEPLOY_ENV: "production"
  API_URL: "https://api.company.com"
  DB_HOST: "prod-db.company.com"

steps:
  - name: "构建"
    container: "golang-builder-1.24"
    commands:
      - "go build -tags=prod -o api ."
    
  - name: "部署到生产环境"
    # container 省略，在宿主机执行
    commands:
      - "docker tag api:${TASK_ID} registry.company.com/api:prod"
      - "docker push registry.company.com/api:prod"
      - "kubectl set image deployment/api api=registry.company.com/api:prod"
```

### 微服务批量构建

```yaml
# projects/microservices.yml
project:
  name: "microservices"
  type: "batch"

environment:
  SERVICES: "user-service,order-service,payment-service,notification-service"

steps:
  - name: "批量构建微服务"
    # container 省略，在宿主机执行
    commands:
      - |
        for service in $(echo $SERVICES | tr ',' ' '); do
          echo "构建 $service..."
          confkit run projects/${service}.yml --parallel &
        done
      - "wait"
      - "echo \"所有微服务构建完成\""
```

### 测试环境清理

```yaml
# projects/cleanup.yml
project:
  name: "test-cleanup"
  type: "maintenance"

steps:
  - name: "清理测试数据"
    container: "postgres-client"
    commands:
      - "psql $TEST_DB_URL -c \"TRUNCATE TABLE test_users, test_orders CASCADE;\""
      
  - name: "重置缓存"
    container: "redis-client"
    commands:
      - "redis-cli -h $REDIS_HOST FLUSHDB"
      
  - name: "清理文件存储"
    # container 省略，在宿主机执行
    commands:
      - "rm -rf ./test-uploads/*"
      - "mkdir -p ./test-uploads"
```

## 🎯 最佳实践

### 1. 配置文件组织

```
project-repo/
├── builder.yml/           # builders 镜像构建配置文件
├── docker-compose.yml/    # builders compose 配置文件
├── .confkit/              # confkit 文件管理目录
├── volumes/               # 临时存储目录
│   ├── artifacts/         # 产物缓存
│   ├── cache/             # 容器数据缓存
│   ├── workspace/         # 宿主机拉取代码/容器构建 共用工作空间目录
└── ...
```

### 2. 环境变量管理

```yaml
# 敏感信息使用环境变量
environment:
  DB_PASSWORD: "${DB_PASSWORD}"          # 从环境变量获取
  API_KEY: "${API_KEY}"
  
  # 非敏感信息可以直接写在配置中
  APP_NAME: "my-app"
  LOG_LEVEL: "info"
```

### 3. 步骤设计原则

```yaml
steps:
  # ✅ 好的做法：步骤职责单一
  - name: "代码检查"
    commands:
      - "golangci-lint run"
  
  - name: "单元测试"  
    commands:
      - "go test ./..."
  
  - name: "构建"
    commands:
      - "go build -o app ."
  
  # ❌ 避免：在一个步骤中做太多事情
  - name: "检查测试构建"
    commands:
      - "golangci-lint run"
      - "go test ./..."
      - "go build -o app ."
```

### 4. 错误处理

```yaml
steps:
  - name: "可选的性能测试"
    commands:
      - "go test -bench=."
    continue_on_error: true    # 失败不影响整个流水线
  
  - name: "关键的安全扫描"
    commands:
      - "gosec ./..."
    retry: 3                   # 失败重试3次
    timeout: "5m"              # 设置超时
```

### 5. 并行优化

```yaml
steps:
  - name: "单元测试"
    commands:
      - "go test ./..."
    parallel_group: "testing"
  
  - name: "集成测试"
    commands:
      - "go test -tags=integration ./..."
    parallel_group: "testing"
  
  - name: "基准测试"
    commands:
      - "go test -bench=."
    parallel_group: "testing"
    continue_on_error: true
```

### 6. 版本管理

```yaml
# 使用Git标签作为版本号
environment:
  VERSION: "${GIT_TAG:-${GIT_COMMIT_SHORT}}"
  
# 构建时注入版本信息
steps:
  - name: "构建"
    commands:
      - "go build -ldflags \"-X main.version=${VERSION}\" -o app ."
```

这些配置示例和最佳实践可以帮助您快速上手confkit CLI，并构建出高效、可维护的CI/CD流水线。