# DevTools 打包设计文档

## 概述

将 DevTools 打包为 macOS `.dmg` 和 Ubuntu `.deb` 安装包，使用 `chip.png` 作为应用图标。

## 架构

### 工具链

- **cargo-bundle** — macOS `.app` bundle 生成
- **cargo-deb** — Ubuntu `.deb` 包生成
- **sips + iconutil** — macOS 原生工具，PNG → ICNS 转换
- **hdiutil** — 基于临时安装目录生成包含 `DevTools.app` 与 `Applications` 快捷方式的 `.dmg`

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
│   ├── make-icons.sh      # 图标转换脚本
│   └── make-dmg.sh        # 生成带 Applications 快捷方式的 DMG
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
./scripts/make-dmg.sh
```

1. 缺失 `icons/icon.icns` 时自动调用 `make-icons.sh`
2. 调用 `cargo bundle --release` 生成 `DevTools.app`
3. 创建临时 DMG staging 目录，放入 `DevTools.app` 与指向 `/Applications` 的快捷方式
4. 使用 `hdiutil create -format UDZO` 输出 `target/release/bundle/osx/DevTools.dmg`

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
- `make-dmg.sh` 检查 macOS、`cargo`、`hdiutil` 和 `.app` 输出，缺失时退出并提示
- `cargo-deb` 仅在 Linux 上可用

## 测试验证

1. macOS: 挂载 `target/release/bundle/osx/DevTools.dmg`，确认同时包含 `DevTools.app` 和 `Applications` 快捷方式；再拖拽安装并验证图标和启动
2. Ubuntu: `sudo dpkg -i target/debian/devtools_0.1.0_amd64.deb`，验证 `devtools` 命令可用

## 注意事项

- GPUI 是原生 GUI 框架，打包后仍需目标系统有相应的图形环境
- 本项目 DMG 由 `make-dmg.sh` 统一生成；不要再直接以 `.app` 为 `hdiutil -srcfolder`，否则镜像内不会出现 `Applications` 快捷方式
- `.deb` 的 `$auto` 依赖会自动检测二进制需要的共享库
