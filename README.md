# DevTools — 密码学桌面工具箱

DevTools 是一个基于 [GPUI](https://www.gpui.rs/) 构建的 Rust 原生桌面应用，面向证书解析、算法识别与常用密码学运算场景。项目当前提供 X.509 证书查看、国密/后量子 OID 识别，以及哈希、对称加解密、RSA/ECDSA、ML-KEM、ML-DSA 等交互式工具。

---

## 功能概览 | Features

### 证书解析 | Certificate Parsing

- **多格式导入** — 支持 `.pem`, `.der`, `.crt`, `.cer`, `.p12`, `.pfx`。
- **PEM 多证书文件** — 可从一个 PEM 文件解析多张证书，并将首张证书作为主证书、其余证书作为链信息展示。
- **基础字段展示** — 文件路径、版本、主题、颁发者、序列号、有效期、签名算法、公钥算法、公钥 OID、密钥长度、密钥类别。
- **证书链展示** — 按解析顺序展示叶证书/中间证书/根证书标签；当前不做证书路径构建或信任验证。
- **扩展项展示** — 展示扩展项 OID、名称、critical 标记和值；对 Basic Constraints、Key Usage、Extended Key Usage、SAN、SKI、AKI 提供更友好的显示。
- **OID 解析** — 覆盖常见 RSA/ECDSA/SHA/AES OID、国密 SM2/SM3/SM4 OID，以及 ML-DSA、SLH-DSA、FN-DSA、ML-KEM、HQC 等后量子相关 OID。
- **PKCS#12 说明** — 当前自动尝试空密码解析；受密码保护的 `.p12/.pfx` 会提示需要密码，尚未提供密码输入 UI。

### 交互工具 | Interactive Tools

| 分类       | Category   | 算法 / 参数                                  | Operations                   |
| ---------- | ---------- | -------------------------------------------- | ---------------------------- |
| 对称算法   | Symmetric  | AES-128-ECB, AES-256-CBC, SM4-ECB, SM4-CBC   | 十六进制输入的加密 / 解密    |
| 非对称算法 | Asymmetric | RSA 2048/3072/4096                           | 密钥生成 / 加密 / 解密       |
|            |            | RSA padding: OAEP-SHA256, PKCS#1 v1.5 兼容模式 | 填充模式切换                 |
|            |            | ECDSA P-256                                  | 密钥生成 / 签名 / 验签       |
| 哈希算法   | Hash       | SHA-256, SHA-384, SHA-512, SM3               | 文本或十六进制输入哈希计算   |
| 密钥封装   | KEM        | ML-KEM-512/768/1024                          | 密钥生成 / 封装 / 解封装     |
| 数字签名   | Signature  | ML-DSA-44/65/87                              | 密钥生成 / 签名 / 验签       |

### OID 识别 | OID Recognition

以下能力主要用于证书解析和算法注册表，不代表均已提供交互式运算界面：

| 分类       | Category   | 算法 / OID 覆盖                              |
| ---------- | ---------- | -------------------------------------------- |
| 哈希       | Hash       | SHA-1, SHA-224, SHA-256/384/512, SHA-512/224, SHA-512/256, SHA3-256/384/512, SM3 |
| 对称       | Symmetric  | AES-128-ECB/CBC, AES-256-ECB/CBC/GCM, SM4    |
| 非对称     | Asymmetric | RSA, RSA-PSS, ECDSA, Ed25519, Ed448, X25519, X448, SM2 |
| 签名       | Signature  | SHA*-RSA, ECDSA-SHA*, SM2-SM3, ML-DSA, SLH-DSA, FN-DSA |
| KEM        | KEM        | ML-KEM-512/768/1024, legacy ML-KEM OID, HQC-128/192/256 |
| 扩展项     | Extension  | SKI, Key Usage, SAN, Basic Constraints, AIA, EKU, CRL Distribution Points, Certificate Policies 等 |

### 界面特性 | UI Features

- **双标签结构** — 顶层分为“证书解析”和“算法解析”，每个标签下有独立左侧菜单。
- **复制到剪贴板** — 证书字段和算法输出支持一键复制，并显示“已复制”状态。
- **可展开错误详情** — 关键操作错误可展开查看完整错误文本。
- **文本输入组件** — 支持单行/多行输入、IME 标记文本、焦点检测和快捷键触发。
- **右侧自定义滚动条** — 主内容区域带自定义滚动条和拖动同步。

---

## 快速开始 | Quick Start

### 环境要求 | Prerequisites

- **Rust** edition 2024 (stable)
- **macOS** 或 **Linux** 图形环境（X11/Wayland）— GPUI 需要原生窗口系统

### 编译与运行 | Build & Run

```bash
# 编译
cargo build

# 运行桌面应用
cargo run

# 运行全部测试
cargo test -p devtools
```

> 说明：`cargo run` 会启动原生 GUI，需要可用的 macOS 桌面或 Linux X11/Wayland 环境。

---

## 打包 | Packaging

### 前置要求 | Prerequisites

```bash
# macOS 打包工具
cargo install cargo-bundle

# Linux/Debian 打包工具
cargo install cargo-deb
```

### macOS | macOS

```bash
# 构建 .app 并创建带“拖入 Applications 安装”入口的 DMG
./scripts/make-dmg.sh

# 如只需要 .app bundle，也可以单独运行
cargo bundle --release
```

输出：

- `.app`：`target/release/bundle/osx/DevTools.app`
- `.dmg`：`target/release/bundle/osx/DevTools.dmg`

`make-dmg.sh` 会在缺少 `icons/icon.icns` 时自动调用 `./scripts/make-icons.sh`。生成的 DMG 挂载后包含 `DevTools.app` 和 `Applications` 快捷方式，可将应用拖入完成安装。

### Ubuntu/Debian | Ubuntu/Debian

```bash
# 构建并打包
cargo deb
```

输出：`target/debian/devtools_0.1.0_amd64.deb`

安装：

```bash
sudo dpkg -i target/debian/devtools_0.1.0_amd64.deb
```

---

## 项目结构 | Project Structure

```text
devtools/
├── src/
│   ├── main.rs              # 入口：初始化 GPUI App 并打开主窗口
│   ├── app.rs               # DevToolsApp：顶层状态、布局、事件、算法工具渲染
│   ├── tabs/
│   │   └── mod.rs           # CertTab / AlgoTab：标签页状态、菜单、输入同步
│   ├── cert/
│   │   ├── mod.rs           # 证书解析入口：PEM/DER/PKCS#12、格式探测、链组装
│   │   ├── extensions.rs    # X.509 扩展项解析与显示格式化
│   │   ├── oid_resolver.rs  # 算法 OID / 扩展 OID / 密钥类别解析
│   │   └── fixtures/        # 证书解析测试夹具
│   ├── algo/
│   │   ├── mod.rs           # 算法模块入口
│   │   ├── tool_trait.rs    # CryptoTool 通用接口
│   │   ├── symmetric.rs     # AES/SM4 对称加解密状态与实现
│   │   ├── asymmetric.rs    # RSA/ECDSA 状态与实现
│   │   ├── hash.rs          # SHA-2/SM3 哈希工具
│   │   ├── pq_kem.rs        # ML-KEM 密钥生成/封装/解封装
│   │   ├── pq_signature.rs  # ML-DSA 密钥生成/签名/验签
│   │   ├── registry.rs      # 算法注册表与 OID 查询
│   │   └── oid_defs.rs      # 国密与后量子 OID 常量
│   └── components/
│       ├── input.rs         # 活跃使用：GPUI 文本输入组件
│       ├── ui_helpers.rs    # 活跃使用：状态条、结果卡片、复制按钮等 UI helper
│       ├── left_menu.rs     # 预留组件：当前主界面未直接使用
│       └── tab_bar.rs       # 预留组件：当前主界面未直接使用
├── scripts/
│   ├── make-icons.sh        # macOS: chip.png → icons/icon.icns
│   └── make-dmg.sh          # macOS: 生成带 Applications 快捷方式的 DMG
├── icons/
│   └── icon.icns            # macOS 应用图标
├── Cargo.toml               # 依赖与 cargo-bundle/cargo-deb 元数据
├── README.md
├── AGENTS.md
└── LICENSE                  # MIT
```

---

## 测试 | Testing

测试以内联 `#[cfg(test)]` 形式放在各源码文件中，当前无独立 `tests/` 目录。

```bash
# 全量测试
cargo test -p devtools

# 单模块示例
cargo test -p devtools -- cert::oid_resolver
```

当前已验证：`cargo test -p devtools` 通过，`92 passed; 0 failed`。

---

## 依赖 | Dependencies

| 类别   | Category    | 核心依赖                                      | 用途                         |
| ------ | ----------- | --------------------------------------------- | ---------------------------- |
| GUI    | UI          | `gpui`, `rfd`                                 | 原生桌面 UI、文件选择        |
| 证书   | Certificate | `x509-parser`, `p12-keystore`, `base64`       | X.509 / PKCS#12 解析与公钥 PEM 输出 |
| OID    | OID         | `oid-registry`, `const-oid`, `once_cell`      | OID 常量、注册表、懒加载映射 |
| 对称   | Symmetric   | `aes`, `cbc`, `ecb`, `sm4`, `cipher`          | AES/SM4 加解密               |
| 非对称 | Asymmetric  | `rsa`, `p256`, `ecdsa`, `rand`, `rand_core`   | RSA/ECDSA 与随机数           |
| 哈希   | Hash        | `sha2`, `sm3`                                 | SHA-2/SM3                    |
| 后量子 | PQ          | `ml-kem`, `ml-dsa`                            | ML-KEM / ML-DSA              |
| 工具   | Utility     | `chrono`                                      | 证书时间格式化               |

---

## 当前限制 | Known Limitations

- 证书链页面展示解析到的证书顺序和角色标签，不执行完整信任链验证。
- `.p12/.pfx` 当前没有密码输入界面，只自动尝试空密码。
- OID 识别覆盖面大于交互式算法工具范围；部分算法仅用于证书解析显示。
- GPUI 是原生 GUI 框架，不能在无显示服务的纯 headless 环境中直接运行应用窗口。

---

## 许可证 | License

[MIT](LICENSE) © 2026 lyonmu@foxmail.com
