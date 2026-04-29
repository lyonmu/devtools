# DevTools 打包实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 DevTools 打包为 macOS .dmg 和 Ubuntu .deb 安装包，使用 chip.png 作为应用图标。

**Architecture:** 通过 cargo-bundle 和 cargo-deb 工具，配合 Cargo.toml 元数据配置，实现一键打包。图标通过脚本从 PNG 转换为 ICNS 格式。

**Tech Stack:** cargo-bundle, cargo-deb, sips/iconutil (macOS), bash scripting

---

## 文件结构

| 文件 | 操作 | 说明 |
|------|------|------|
| `scripts/make-icons.sh` | 创建 | 图标转换脚本 |
| `icons/` | 创建目录 | 存放生成的 .icns 文件 |
| `Cargo.toml` | 修改 | 添加 [package.metadata.bundle] 和 [package.metadata.deb] 配置 |
| `README.md` | 修改 | 添加打包使用说明 |

---

### Task 1: 创建图标转换脚本

**Files:**
- Create: `scripts/make-icons.sh`

- [ ] **Step 1: 编写图标转换脚本**

```bash
#!/bin/bash
set -e

echo "Generating macOS .icns icon from chip.png..."

# Check if chip.png exists
if [ ! -f "chip.png" ]; then
    echo "Error: chip.png not found in project root!"
    exit 1
fi

# Create icons directory
mkdir -p icons

# Create iconset directory
ICONSET_DIR="icons/icon.iconset"
rm -rf "$ICONSET_DIR"
mkdir -p "$ICONSET_DIR"

# Generate different sizes using sips
SIZES=(16 32 64 128 256 512 1024)
for SIZE in "${SIZES[@]}"; do
    sips -z "$SIZE" "$SIZE" chip.png --out "$ICONSET_DIR/icon_${SIZE}x${SIZE}.png" 2>/dev/null || true
    # Also create @2x versions for retina
    if [ "$SIZE" -le 512 ]; then
        RETINA_SIZE=$((SIZE * 2))
        sips -z "$RETINA_SIZE" "$RETINA_SIZE" chip.png --out "$ICONSET_DIR/icon_${SIZE}x${SIZE}@2x.png" 2>/dev/null || true
    fi
done

# Convert to .icns
iconutil -c icns "$ICONSET_DIR" -o "icons/icon.icns"

# Cleanup
rm -rf "$ICONSET_DIR"

echo "Success! Generated icons/icon.icns"
```

- [ ] **Step 2: 设置脚本可执行权限**

```bash
chmod +x scripts/make-icons.sh
```

- [ ] **Step 3: 运行脚本验证**

```bash
./scripts/make-icons.sh
```
Expected: `Success! Generated icons/icon.icns`

- [ ] **Step 4: 验证生成的图标**

```bash
ls -la icons/icon.icns
file icons/icon.icns
```
Expected: 显示 ICNS 文件信息

- [ ] **Step 5: Commit**

```bash
git add scripts/make-icons.sh icons/icon.icns
git commit -m "feat: add icon conversion script and generate icns"
```

---

### Task 2: 配置 Cargo.toml 打包元数据

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: 添加 cargo-bundle 和 cargo-deb 元数据配置**

在 `Cargo.toml` 末尾添加：

```toml
[package.metadata.bundle]
name = "DevTools"
identifier = "com.lyonmu.devtools"
version = "0.1.0"
icon = ["icons/icon.icns"]
category = "DeveloperTool"
short_description = "密码学桌面工具箱"
long_description = """
DevTools 是一个基于 GPUI 框架构建的跨平台密码学桌面工具，
支持证书解析、算法分析与密码学运算。
"""

[package.metadata.deb]
maintainer = "lyonmu <lyonmu@foxmail.com>"
copyright = "2026, lyonmu <lyonmu@foxmail.com>"
license-file = ["LICENSE", "4"]
extended-description = """\
DevTools 是一个基于 GPUI 框架构建的跨平台密码学桌面工具，\
支持证书解析、算法分析与密码学运算。\
"""
depends = "$auto"
section = "devel"
priority = "optional"
assets = [
    ["target/release/devtools", "usr/bin/", "755"],
    ["README.md", "usr/share/doc/devtools/README", "644"],
]
```

- [ ] **Step 2: 验证 Cargo.toml 格式**

```bash
cargo check
```
Expected: 无错误

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "feat: add cargo-bundle and cargo-deb metadata to Cargo.toml"
```

---

### Task 3: 更新 README 添加打包说明

**Files:**
- Modify: `README.md`

- [ ] **Step 1: 在 README.md 的 "快速开始" 章节后添加 "打包" 章节**

在 `---` 分隔符之后、"项目结构" 之前添加：

```markdown
## 打包 | Packaging

### 前置要求 | Prerequisites

```bash
# 安装打包工具
cargo install cargo-bundle  # macOS
cargo install cargo-deb     # Linux
```

### macOS | macOS

```bash
# 1. 生成图标
./scripts/make-icons.sh

# 2. 构建并打包
cargo bundle --release
```

输出：`target/release/bundle/osx/DevTools.dmg`

### Ubuntu/Debian | Ubuntu/Debian

```bash
# 构建并打包
cargo deb --release
```

输出：`target/debian/devtools_0.1.0_amd64.deb`

安装：
```bash
sudo dpkg -i target/debian/devtools_0.1.0_amd64.deb
```

---
```

- [ ] **Step 2: 验证 README 格式**

```bash
cat README.md | head -80
```

- [ ] **Step 3: Commit**

```bash
git add README.md
git commit -m "docs: add packaging instructions to README"
```

---

### Task 4: 端到端验证（macOS）

**Files:**
- 验证已有文件

- [ ] **Step 1: 确保 release 构建完成**

```bash
cargo build --release
```

- [ ] **Step 2: 运行完整打包流程**

```bash
./scripts/make-icons.sh && cargo bundle --release
```

- [ ] **Step 3: 验证 DMG 输出**

```bash
ls -la target/release/bundle/osx/DevTools.dmg
```
Expected: 显示 .dmg 文件

- [ ] **Step 4: 手动测试 DMG**

```bash
open target/release/bundle/osx/DevTools.dmg
```
Expected: Finder 窗口打开，显示 DevTools.app

- [ ] **Step 5: Commit (如有修改)**

```bash
git add -A
git commit -m "chore: verify packaging workflow"
```

---

### Task 5: 端到端验证（Linux/Debian）

**Files:**
- 验证已有文件

- [ ] **Step 1: 确保 release 构建完成**

```bash
cargo build --release
```

- [ ] **Step 2: 运行完整打包流程**

```bash
cargo deb --release
```

- [ ] **Step 3: 验证 DEB 输出**

```bash
ls -la target/debian/devtools_*.deb
```
Expected: 显示 .deb 文件

- [ ] **Step 4: 验证 DEB 内容**

```bash
dpkg-deb --info target/debian/devtools_*.deb
dpkg-deb --contents target/debian/devtools_*.deb
```
Expected: 显示包信息和文件列表

- [ ] **Step 5: Commit (如有修改)**

```bash
git add -A
git commit -m "chore: verify debian packaging workflow"
```
