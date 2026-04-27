# DevTools — 密码学桌面工具箱

一个基于 [GPUI](https://www.gpui.rs/) 框架构建的跨平台密码学桌面工具，支持证书解析、算法分析与密码学运算。

---

## 功能概览 | Features

### 证书解析 | Certificate Parsing

- **多格式支持** — `.pem`, `.der`, `.crt`, `.cer`, `.p12`, `.pfx`
- **基本信息** — 主题 / 颁发者 / 序列号 / 有效期 / 签名算法 / 公钥算法
- **证书链** — 叶证书 → 中间证书 → 根证书层次展示
- **扩展项** — Key Usage, SAN, Basic Constraints, EKU 等解析
- **OID 解析** — 国密 (SM2/SM3/SM4)、后量子 (ML-DSA/SLH-DSA/FN-DSA/HQC/ML-KEM) 及标准算法 OID 识别

### 算法工具 | Algorithm Tools

| 分类       | Category   | 功能                                       | Operations                   |
| ---------- | ---------- | ------------------------------------------ | ---------------------------- |
| 对称算法   | Symmetric  | AES-128-ECB, AES-256-CBC, SM4-ECB, SM4-CBC | 加密 / 解密                  |
| 非对称算法 | Asymmetric | RSA (2048/3072/4096), ECDSA (P-256)        | 密钥生成 / 加解密 / 签名验签 |
| 哈希算法   | Hash       | SHA-256, SHA-384, SHA-512, SM3             | 哈希计算 (文本/十六进制)     |
| 密码封装   | KEM        | ML-KEM-512/768/1024 (FIPS 203)             | 密钥生成 / 封装 / 解封装     |
| 数字签名   | Signature  | ML-DSA-44/65/87 (FIPS 204)                 | 密钥生成 / 签名 / 验签       |
|            |            | FN-DSA-512/1024 (FIPS 206 predicted)       | OID 识别                     |
|            |            | SLH-DSA 12 变体 (FIPS 205)                 | OID 识别                     |

### 后量子密码学 | Post-Quantum Cryptography

- **FIPS 203** — ML-KEM-512/768/1024 密钥封装
- **FIPS 204** — ML-DSA-44/65/87 数字签名
- **FIPS 205** — SLH-DSA-SHA2/SHAKE 全系列 OID 识别
- **FIPS 206** — FN-DSA-512/1024 OID 识别 (预测值，标准尚未正式发布)
- **HQC** — HQC-128/192/256 候选算法 OID 识别

---

## 快速开始 | Quick Start

### 环境要求 | Prerequisites

- **Rust** edition 2024 (stable)
- **macOS** 或 **Linux** (X11/Wayland) — GPUI 需要原生窗口系统

### 编译与运行 | Build & Run

```bash
# 编译
cargo build

# 运行
cargo run

# 运行测试
cargo test
```

---

## 项目结构 | Project Structure

```
devtools/
├── src/
│   ├── main.rs              # 入口 | Entry point
│   ├── app.rs               # 主窗口、标签页管理 | Window & tab management
│   ├── tabs/
│   │   └── mod.rs           # CertTab + AlgoTab 渲染 | Tab rendering
│   ├── cert/
│   │   ├── mod.rs           # 证书解析 (PEM/DER/PKCS#12) | Certificate parsing
│   │   ├── extensions.rs    # X.509 扩展项解析 | Extension parsing
│   │   └── oid_resolver.rs  # OID → 算法名称解析 | OID to name resolution
│   ├── algo/
│   │   ├── mod.rs           # 算法模块入口 | Module root
│   │   ├── symmetric.rs     # 对称加密 (AES/SM4) | Symmetric crypto
│   │   ├── asymmetric.rs    # 非对称加密 (RSA/ECDSA) | Asymmetric crypto
│   │   ├── hash.rs          # 哈希算法 (SHA/SM3) | Hash algorithms
│   │   ├── pq_kem.rs        # 后量子 KEM (ML-KEM) | PQ KEM
│   │   ├── pq_signature.rs  # 后量子签名 (ML-DSA) | PQ signatures
│   │   ├── registry.rs      # 算法注册表 | Algorithm registry
│   │   └── oid_defs.rs      # OID 常量定义 | OID constants
│   └── components/          # 可复用 GPUI 组件 | Reusable GPUI widgets
├── Cargo.toml
└── LICENSE                  # MIT
```

---

## 依赖 | Dependencies

| 类别   | Category    | 核心依赖                              | Key Crates                   |
| ------ | ----------- | ------------------------------------- | ---------------------------- |
| GUI    | 界面        | `gpui`                                | GPU-accelerated UI framework |
| 证书   | Certificate | `x509-parser`, `p12-keystore`, `rfd`  | File dialog                  |
| 对称   | Symmetric   | `aes`, `cbc`, `ecb`, `sm4`, `cipher`  | RustCrypto                   |
| 非对称 | Asymmetric  | `rsa`, `p256`, `ecdsa`, `rand`        | RustCrypto                   |
| 哈希   | Hash        | `sha2`, `sm3`                         | SHA & SM3                    |
| 后量子 | PQ          | `ml-kem`, `ml-dsa`, `const-oid`       | FIPS 203/204                 |
| 工具   | Utility     | `chrono`, `once_cell`, `oid-registry` |                              |

---

## 许可证 | License

[MIT](LICENSE) © 2026 lyonmu@foxmail.com
