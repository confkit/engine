# CLI 命令参考

ConfKit 所有 CLI 命令的完整参考。

## 全局选项

```bash
confkit --help     # 查看帮助
confkit -v         # 查看版本（--version 的缩写）
```

---

## 交互式模式

```bash
confkit
```

启动交互式模式，提供引导式菜单：

- `[RUN] Run 管理` → 执行项目构建任务
- `[BUILDER] Builder 管理` → 镜像和容器管理
- `[IMAGE] Image 管理` → 管理构建镜像
- `[LOG] Log 管理` → 列出、查看和检查任务日志（支持全部项目 / 按 space / 按 space+project 查询）
- `[CLEAN] Clean 管理` → 清理日志、工作空间、产物、缓存、临时文件

---

## Builder 命令

```bash
confkit builder list                    # 列出所有构建器
confkit builder create -n <name>        # 创建构建器
confkit builder start -n <name>         # 启动构建器
confkit builder stop -n <name>          # 停止构建器
confkit builder remove -n <name>        # 删除构建器
confkit builder health                  # 健康检查（全部）
confkit builder health -n <name>        # 健康检查（指定）
```

## Image 命令

```bash
confkit image list                      # 列出镜像
confkit image create <image:tag>        # 拉取/构建镜像
confkit image remove <image:tag>        # 删除镜像
```

## Run 命令

```bash
confkit run --space <space> --project <project>                  # 运行构建
confkit run --space <space> --project <project> --dry-run        # 预览（不实际执行）
confkit run --space <space> --project <project> -e KEY=VALUE     # 注入环境变量
```

## Log 命令

```bash
confkit log list                                              # 列出所有日志
confkit log list --space <space>                              # 按空间过滤
confkit log list --space <space> --project <project>          # 按空间+项目过滤
confkit log list --space <space> --project <project> \        # 分页查询
  --page 1 --size 10
confkit log show --task <task_id>                             # 查看日志内容
confkit log info --task <task_id>                             # 查看元数据
confkit log clean --all                                       # 清理所有日志
confkit log clean --space <space>                             # 按空间清理
confkit log clean --space <space> --project <project>         # 按空间+项目清理
confkit log clean --task <task_id>                            # 清理指定任务
```

## Clean 命令

```bash
confkit clean workspace          # 清理工作空间目录
confkit clean artifacts          # 清理产物目录
confkit clean cache              # 清理缓存目录
confkit clean temp               # 清理临时文件目录
confkit clean log --all          # 清理所有日志（等同于 log clean --all）
confkit clean log --space <s>    # 按空间清理日志
confkit clean log --space <s> --project <p>   # 按空间+项目清理日志
confkit clean log --task <id>    # 清理指定任务日志
confkit clean all                # 清理全部（workspace、artifacts、cache、temp、logs）
```

## Config 命令

```bash
confkit config show              # 展示配置概览（引擎、空间、项目、镜像）
confkit config validate          # 校验配置文件（路径、必填字段等）
```
