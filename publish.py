# pip install ruamel.yaml
import subprocess
import re
from ruamel.yaml import YAML

# 1. 自动 bump 版本号（patch，可改为 minor/major/prerelease）
# subprocess.run([
#     "cargo", "release", "patch", "--execute", "--no-confirm"
# ], check=True)

# 2. 读取新版本号（从 Cargo.toml 读取）
with open("Cargo.toml", "r", encoding="utf-8") as f:
    for line in f:
        m = re.match(r'version\s*=\s*"(.*?)"', line)
        if m:
            new_version = m.group(1)
            break
    else:
        raise RuntimeError("未找到 Cargo.toml 的 version 字段")

print(f"新版本号：{new_version}")

# 3. 生成 changelog
# subprocess.run(["git", "cliff", "-o", "CHANGELOG.md"], check=True)

# 4. 更新 engine.yml 的 environment 字段
yml_path = "example/.confkit/spaces/confkit/engine.yml"
yaml = YAML()
yaml.preserve_quotes = True

with open(yml_path, "r", encoding="utf-8") as f:
    data = yaml.load(f)

if "environment" not in data or data["environment"] is None:
    data["environment"] = {}
data["environment"]["VERSION"] = new_version

with open(yml_path, "w", encoding="utf-8") as f:
    yaml.dump(data, f)

print(f"已将 {yml_path} 的 environment.VERSION 更新为 {new_version}")

# 5. 执行发包
# subprocess.run(["cargo", "publish", "--no-confirm"], check=True)

print("发布流程完成！")