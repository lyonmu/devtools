# DevTools 打包设计文档

## 概述

将 DevTools 打包为 macOS `.dmg` 和 Ubuntu `.deb` 安装包，使用 `chip.png` 作为应用图标。

## 架构

### 工具链

- **cargo-bundle** — macOS `.app` bundle + `.dmg` 生成
- **cargo-deb** — Ubuntu `.deb` 包生成
- **sips + iconutil** — macOS 原生工具，PNG → ICNS 转换

### 配置文件

所有打包元数据集中在 `Cargo.toml` 的 `[package.metadata]` 段：

```toml
[package.metadata.bundle]
name = "DevTools"
identifier = "com.lyonmu.devtools"
icon = ["icons/icon.icns"]
category = "DeveloperTool"

[package.metadata.deb]
maintainer = "lyonmu <lyonmu@foxmail.com>"
depends = "$auto"
section = "devel"
```

### 目录结构

```
devtools/
├── icons/
│   └── icon.icns          # 由 chip.png 生成
├── scripts/
│   └── make-icons.sh      # 图标转换脚本
├── Cargo.toml             # 打包元数据
└── chip.png               # 原始图标
```

## 数据流

### 图标转换

```bash
./scripts/make-icons.sh
```

1. 检查 `chip.png` 存在
2. 创建 `icons.iconset` 目录
3. 使用 `sips` 生成多尺寸 PNG（16x16 ~ 1024x1024）
4. 使用 `iconutil` 打包为 `icons/icon.icns`

### macOS 打包

```bash
cargo bundle --release
```

输出：`target/release/bundle/osx/DevTools.dmg`

### Linux 打包

```bash
cargo deb --release
```

输出：`target/debian/devtools_0.1.0_amd64.deb`

## 依赖安装

```bash
cargo install cargo-bundle
cargo install cargo-deb
```

## 错误处理

- 脚本检查 `chip.png` 存在性，缺失时退出并提示
- `icons/` 目录不存在时自动创建
- `cargo-bundle` 仅在 macOS 上可用，Linux 上跳过
- `cargo-deb` 仅在 Linux 上可用

## 测试验证

1. macOS: `open target/release/bundle/osx/DevTools.dmg`，拖拽安装，验证图标和启动
2. Ubuntu: `sudo dpkg -i target/debian/devtools_0.1.0_amd64.deb`，验证 `devtools` 命令可用

## 注意事项

- GPUI 是原生 GUI 框架，打包后仍需目标系统有相应的图形环境
- `cargo-bundle` 的 DMG 不支持自定义背景图
- `.deb` 的 `$auto` 依赖会自动检测二进制需要的共享库
