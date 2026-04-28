# DevTools — 密码学桌面工具箱

## TL;DR

> **Quick Summary:** 一个基于GPUI的原生桌面应用，为开发者提供密码学工具集，涵盖经典算法、国密算法、后量子密码和证书解析。
>
> **Delivers:** 完整的密码学工具箱，支持对称/非对称加密、哈希、KEM、数字签名、证书解析等功能。
>
> **Stack:** Rust + GPUI (GPU加速原生GUI框架)
>
> **Status:** 🟢 Active Development — 核心功能已完成，持续扩展优化中

---

## 🏗️ Architecture

### Stack Layer

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Language | Rust (edition 2024) | 内存安全、高性能、优秀的密码学生态 |
| GUI框架 | GPUI 0.2 | GPU加速、原生体验、Rust原生 |
| 密码库 | RustCrypto系列 | 成熟可靠、社区维护 |
| 后量子 | ml-kem/ml-dsa | NIST标准化实现 |

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    DevToolsApp (GPUI)                        │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   CertTab    │  │   AlgoTab    │  │  Components  │      │
│  │  证书解析    │  │  算法解析    │  │  可复用组件  │      │
│  └──────┬───────┘  └──────┬───────┘  └──────────────┘      │
│         │                 │                                  │
│  ┌──────┴───────┐  ┌──────┴───────┐                        │
│  │   src/cert/  │  │   src/algo/  │                        │
│  │  证书处理    │  │  算法实现    │                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

### Source Layout

```
src/
├── main.rs              # 入口：创建GPUI Application
├── app.rs               # 根视图：Tab管理、布局、渲染逻辑
├── tabs/
│   └── mod.rs           # CertTab + AlgoTab 结构定义
├── cert/
│   ├── mod.rs           # 证书解析（PEM/DER/PKCS#12）
│   ├── extensions.rs    # X.509扩展项解析
│   └── oid_resolver.rs  # OID解析器（算法/扩展）
├── algo/
│   ├── mod.rs           # 类型导出
│   ├── symmetric.rs     # 对称加密（AES/SM4）
│   ├── asymmetric.rs    # 非对称加密（RSA/ECDSA）
│   ├── hash.rs          # 哈希算法（SHA-2/SM3）
│   ├── pq_kem.rs        # 后量子KEM（ML-KEM）
│   ├── pq_signature.rs  # 后量子签名（ML-DSA）
│   ├── registry.rs      # 算法注册表
│   └── oid_defs.rs      # OID常量定义
└── components/
    ├── mod.rs            # 组件导出
    ├── input.rs          # 文本输入组件（IME支持）
    ├── left_menu.rs      # 左侧菜单（未使用）
    └── tab_bar.rs        # Tab栏（未使用）
```

### Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| gpui | * | GPU加速GUI框架 |
| x509-parser | 0.17 | X.509证书解析 |
| p12-keystore | 0.1 | PKCS#12解析 |
| aes/cbc/ecb | 0.8 | AES加密 |
| sm4 | 0.5 | SM4加密 |
| rsa | 0.9 | RSA加密 |
| p256/ecdsa | 0.13/0.16 | ECDSA签名 |
| ml-kem | 0.2 | ML-KEM后量子KEM |
| ml-dsa | 0.1.0-rc.8 | ML-DSA后量子签名 |
| sha2/sm3 | 0.10/0.4 | 哈希算法 |

---

## 👥 Target Users

### Primary: 开发者

- **Who:** 需要使用密码学工具的软件开发者
- **Context:** 开发过程中需要测试加密解密、签名验证、证书解析等功能
- **Needs:** 快速验证算法实现、调试密码学问题、理解证书结构
- **Success:** 能够高效完成密码学相关的开发和调试任务

---

## 🎯 Core Objectives

1. **算法覆盖全面** — 支持经典、国密、后量子三大类密码算法
2. **开发者友好** — 提供清晰的输入输出、错误提示、格式转换
3. **原生体验** — 利用GPUI提供流畅的桌面GUI体验
4. **可扩展性** — 架构支持方便地添加新算法和功能

---

## ⚙️ Technical Constraints

| Constraint | Rationale |
|------------|-----------|
| Rust + GPUI | 项目已选定，保持技术栈一致性 |
| macOS/Linux | GPUI需要窗口系统，不支持无头模式 |
| 单crate结构 | 项目规模适中，保持简单 |
| 中文UI | 面向中文开发者群体 |

---

## 🧱 Key Technical Decisions

| Decision | Rationale |
|----------|-----------|
| 内联测试 | Rust惯例，测试与代码同文件 |
| 自定义SM4实现 | 学习目的，展示算法细节 |
| OID静态映射 | 性能优先，编译时确定 |
| 文件对话框异步 | GPUI要求，使用cx.spawn() |

---

## 📐 Code Patterns & Standards

### GPUI API约定

```rust
// .id() 返回 Stateful<Div> 不是 Div
fn render_something() -> Stateful<Div> {
    div().id("unique-id").child(...)
}

// .child() 需要 owned 类型
.child(s.clone())  // ✅ 正确
.child(&s)         // ❌ 错误
```

### 测试模式

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // 测试逻辑
    }
}
```

---

## 📎 Quick Reference

### 常用命令

```bash
cargo build                    # 编译
cargo test -p devtools         # 运行所有测试
cargo run                      # 启动GUI应用
cargo test -p devtools -- cert::oid_resolver  # 运行单个模块测试
```

### 关键文件

| 文件 | 用途 |
|------|------|
| `src/app.rs` | 根视图，所有渲染逻辑 |
| `src/tabs/mod.rs` | Tab结构定义 |
| `src/cert/mod.rs` | 证书解析入口 |
| `src/algo/symmetric.rs` | 对称加密实现 |
| `src/algo/asymmetric.rs` | 非对称加密实现 |

---

## 🔗 Dependencies & Integrations

### 外部依赖

- **RustCrypto系列** — AES/SM4/RSA/ECDSA/SHA/SM3
- **ml-kem/ml-dsa** — 后量子密码实现
- **x509-parser** — 证书解析
- **GPUI** — GUI框架

### 内部依赖

- `src/components/input.rs` — 文本输入组件
- `src/cert/oid_resolver.rs` — OID解析
- `src/algo/registry.rs` — 算法注册表

---

## 📊 Success Criteria

| Metric | Target | How to Verify |
|--------|--------|---------------|
| 算法覆盖 | 经典+国密+后量子全覆盖 | 功能测试 |
| 测试覆盖率 | 核心模块>80% | cargo test |
| 构建时间 | <30秒 | cargo build |
| 启动时间 | <2秒 | 手动测试 |

---

## 📝 Decision Log

| # | Decision | Rationale | Date |
|---|----------|-----------|------|
| 1 | 使用GPUI作为GUI框架 | GPU加速、原生体验 | 项目初期 |
| 2 | 实现自定义SM4 | 学习目的，展示算法细节 | 项目初期 |
| 3 | 支持后量子密码 | 前瞻性，NIST标准化 | 项目中期 |
| 4 | 单crate结构 | 项目规模适中，保持简单 | 项目初期 |

---

## 🚀 Future Directions

### 短期（1-3个月）

- 完善测试覆盖率
- 优化UI/UX体验
- 添加更多算法支持

### 中期（3-6个月）

- 性能测试和对比功能
- 证书链验证
- 导入导出功能

### 长期（6个月+）

- 插件系统
- 跨平台优化
- 国际化支持

---

*Last Updated: 2026-04-28*
*Next Review: 待定*