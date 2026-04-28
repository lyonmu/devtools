# Roadmap — DevTools 密码学桌面工具箱

## 📊 Milestone Overview

| # | Milestone | Status | Progress |
|---|-----------|--------|----------|
| v1 | 全面规划与优化 | 🟢 Active | 50% |

---

## 🎯 v1: 全面规划与优化

**Goal:** 完善功能、优化架构、提高质量

**Success Criteria:**
- 核心算法测试覆盖率>80%
- UI/UX显著改善
- 架构清晰可扩展
- 文档完整

**Timeline:** 2-3个月

---

## 📋 Phase Breakdown

### Phase 1: 架构重构与组件化

**Goal:** 提取可复用组件，统一代码结构

**Plans:** 3 plans

Plans:

**Wave 1**
- [x] 01-01-PLAN.md — Extract shared UI helpers (status banners, result cards, buttons, constants)

**Wave 2** *(blocked on Wave 1 completion)*
- [x] 01-02-PLAN.md — Refactor app.rs and tabs/mod.rs to use shared UI helpers
- [x] 01-03-PLAN.md — Create CryptoTool trait and implement for all algorithm tool states

Cross-cutting constraints:
- Shared UI constants/helpers must preserve existing GPUI behavior and Chinese copy while reducing duplication.
- Full phase verification requires `cargo build -p devtools` and `cargo test -p devtools` to pass.

**Deliverables:**
- [x] 提取共享 GPUI UI helpers/constants
- [x] 重构 app/tabs 使用共享 UI helpers
- [x] 统一算法接口（trait抽象）
- [ ] 改进状态管理

**Dependencies:** None

**Parallelization:**
- **Can Run In Parallel:** NO
- **Parallel Group:** None
- **Blocks:** Phase 2, Phase 3
- **Blocked By:** None

**Acceptance Criteria:**
- [x] 组件可独立使用
- [x] 算法接口统一
- [x] 代码复用率提高

**QA Scenarios:**

```
Scenario: 组件独立渲染
Tool: cargo test
Steps:
1. 创建独立组件实例
2. 验证渲染结果
Expected Result: 组件正常渲染
Evidence: test_components_render

Scenario: 算法接口统一
Tool: cargo test
Steps:
1. 实现新算法
2. 通过统一接口调用
Expected Result: 算法正常执行
Evidence: test_unified_interface
```

**Commit Strategy:**
- 1 commit per component refactor
- 1 commit for interface unification

**Verification Commands:**
```bash
cargo build
cargo test -p devtools
```

---

### Phase 2: 测试完善

**Goal:** 提高测试覆盖率，确保代码质量

**Plans:** 6 plans

Plans:

**Wave 1**
- [x] 02-01-PLAN.md — Establish tarpaulin coverage harness and waiver ledger

**Wave 2** *(blocked on Wave 1 completion)*
- [x] 02-02-PLAN.md — Add deterministic hash and symmetric algorithm tests

**Wave 3** *(blocked on Wave 2 completion)*
- [x] 02-03-PLAN.md — Expand RSA and ECDSA flow/error-path tests

**Wave 4** *(blocked on Wave 3 completion)*
- [x] 02-04-PLAN.md — Add certificate fixtures and parser format tests
- [x] 02-05-PLAN.md — Add certificate extension and OID helper tests

**Wave 5** *(blocked on Wave 4 completion)*
- [x] 02-06-PLAN.md — Add PQ tests and enforce final coverage gate

Cross-cutting constraints:
- Hard coverage gate is measured line coverage `>80%` for `src/algo/*` and `src/cert/*`; GPUI/UI files are excluded.
- Verification requires `cargo test -p devtools` and `cargo tarpaulin -p devtools --out Html`.
- Tests stay inline in source modules; no `tests/` directory, CI, lint, or format tooling is added.
- Certificate fixtures must be checked-in, small, public/non-secret test material under `src/cert/fixtures/`.

**Deliverables:**
- [x] 补充单元测试 (85 tests, was 46)
- [x] 添加集成测试 (certificate fixtures, PQ flows)
- [x] 边界条件测试 (error paths, malformed inputs)
- [ ] 性能基准测试 (deferred to Phase 4)

**Dependencies:** Phase 1

**Parallelization:**
- **Can Run In Parallel:** NO
- **Parallel Group:** None
- **Blocks:** Phase 4
- **Blocked By:** Phase 1

**Acceptance Criteria:**
- [x] 核心模块测试覆盖率>80% (86.8% achieved)
- [x] 所有算法有单元测试
- [x] 集成测试覆盖主要流程

**QA Scenarios:**

```
Scenario: 测试覆盖率检查
Tool: cargo tarpaulin
Steps:
1. 运行覆盖率工具
2. 检查报告
Expected Result: 覆盖率>80%
Evidence: coverage_report.html

Scenario: 单元测试完整性
Tool: cargo test
Steps:
1. 运行所有测试
2. 检查测试结果
Expected Result: 所有测试通过
Evidence: test_results.txt
```

**Commit Strategy:**
- 1 commit per module test addition
- 1 commit for integration tests

**Verification Commands:**
```bash
cargo test -p devtools
cargo tarpaulin -p devtools --out Html
```

---

### Phase 3: UI/UX改进

**Goal:** 改善用户界面和交互体验

**Plans:** 3 plans

Plans:

**Wave 1**
- [x] 03-01-PLAN.md — Error display enhancements: Warning variant, icon prefix, expandable details

**Wave 2** *(blocked on Wave 1 completion)*
- [x] 03-02-PLAN.md — Copy buttons: per-output copy with "已复制" feedback, cert field copy

**Wave 3** *(blocked on Wave 2 completion)*
- [x] 03-03-PLAN.md — Loading spinners for operations (drag-drop skipped: GPUI 0.2 limitation)

Cross-cutting constraints:
- All new UI text must be Chinese (project convention from AGENTS.md).
- GPUI 0.2 API: `.id()` returns `Stateful<Div>`, `.child()` needs owned types.
- Full phase verification requires `cargo build -p devtools` and `cargo test -p devtools` to pass.

**Deliverables:**
- [x] 改进错误提示
- [x] 添加一键复制功能
- [ ] ~~支持文件拖放~~ *(skipped — GPUI 0.2 doesn't support OS file drag-drop)*
- [x] 添加进度指示
- [ ] ~~支持快捷键~~ *(deferred — not selected for discussion)*

**Dependencies:** Phase 1

**Parallelization:**
- **Can Run In Parallel:** YES
- **Parallel Group:** Phase 3 + Phase 5
- **Blocks:** Phase 6
- **Blocked By:** Phase 1

**Acceptance Criteria:**
- [x] 错误提示清晰友好
- [x] 复制功能正常工作
- [ ] ~~文件拖放支持~~ *(skipped — GPUI 0.2 limitation)*
- [x] 长时间操作有进度指示

**QA Scenarios:**

```
Scenario: 错误提示测试
Tool: 手动测试
Steps:
1. 输入无效数据
2. 观察错误提示
Expected Result: 提示清晰友好
Evidence: screenshot_error.png

Scenario: 复制功能测试
Tool: 手动测试
Steps:
1. 执行加密操作
2. 点击复制按钮
3. 粘贴到文本编辑器
Expected Result: 内容正确复制
Evidence: screenshot_copy.png
```

**Commit Strategy:**
- 1 commit per feature
- 1 commit for UI polish

**Verification Commands:**
```bash
cargo build
cargo run
```

---

### Phase 4: 功能扩展

**Goal:** 添加新算法和功能

**Deliverables:**
- [ ] SM2加密签名支持
- [ ] SHA-3哈希支持
- [ ] 算法性能测试
- [ ] 批量处理支持
- [ ] 密钥导入导出

**Dependencies:** Phase 2

**Parallelization:**
- **Can Run In Parallel:** NO
- **Parallel Group:** None
- **Blocks:** Phase 6
- **Blocked By:** Phase 2

**Acceptance Criteria:**
- [ ] SM2功能完整
- [ ] SHA-3功能完整
- [ ] 性能测试准确
- [ ] 批量处理正常

**QA Scenarios:**

```
Scenario: SM2功能测试
Tool: cargo test
Steps:
1. 测试SM2加密
2. 测试SM2解密
3. 测试SM2签名
4. 测试SM2验证
Expected Result: 所有功能正常
Evidence: test_sm2.txt

Scenario: 性能测试
Tool: cargo bench
Steps:
1. 运行基准测试
2. 检查结果
Expected Result: 性能数据准确
Evidence: bench_results.txt
```

**Commit Strategy:**
- 1 commit per algorithm
- 1 commit for performance testing

**Verification Commands:**
```bash
cargo test -p devtools
cargo bench
```

---

### Phase 5: 文档完善

**Goal:** 提供完整的用户和开发者文档

**Deliverables:**
- [ ] 完善README
- [ ] 添加API文档
- [ ] 编写架构文档
- [ ] 创建贡献指南

**Dependencies:** Phase 3

**Parallelization:**
- **Can Run In Parallel:** YES
- **Parallel Group:** Phase 3 + Phase 5
- **Blocks:** Phase 6
- **Blocked By:** Phase 3

**Acceptance Criteria:**
- [ ] README完整清晰
- [ ] API文档可生成
- [ ] 架构文档准确
- [ ] 贡献指南实用

**QA Scenarios:**

```
Scenario: README检查
Tool: 手动检查
Steps:
1. 阅读README
2. 验证说明准确性
Expected Result: 说明完整准确
Evidence: README.md

Scenario: API文档生成
Tool: cargo doc
Steps:
1. 生成文档
2. 检查完整性
Expected Result: 文档生成成功
Evidence: doc_output/
```

**Commit Strategy:**
- 1 commit per doc type
- 1 commit for final review

**Verification Commands:**
```bash
cargo doc --open
```

---

### Phase 6: 最终优化与发布

**Goal:** 最终优化、测试、准备发布

**Deliverables:**
- [ ] 性能优化
- [ ] 最终测试
- [ ] 版本号更新
- [ ] 发布准备

**Dependencies:** Phase 4, Phase 5

**Parallelization:**
- **Can Run In Parallel:** NO
- **Parallel Group:** None
- **Blocks:** None
- **Blocked By:** Phase 4, Phase 5

**Acceptance Criteria:**
- [ ] 性能达标
- [ ] 所有测试通过
- [ ] 版本号正确
- [ ] 发布就绪

**QA Scenarios:**

```
Scenario: 性能测试
Tool: cargo bench
Steps:
1. 运行基准测试
2. 对比目标
Expected Result: 性能达标
Evidence: bench_final.txt

Scenario: 最终测试
Tool: cargo test
Steps:
1. 运行所有测试
2. 检查结果
Expected Result: 所有测试通过
Evidence: test_final.txt
```

**Commit Strategy:**
- 1 commit for optimization
- 1 commit for version bump
- 1 commit for release prep

**Verification Commands:**
```bash
cargo test -p devtools
cargo build --release
```

---

## 📊 Phase Dependencies

```
Phase 1 (架构重构)
    ↓
Phase 2 (测试完善) ← Phase 3 (UI/UX改进) ← Phase 5 (文档完善)
    ↓                    ↓
Phase 4 (功能扩展) ← Phase 6 (最终优化)
```

---

## 🚀 Execution Strategy

### Wave 1 (Week 1-2)
- Phase 1: 架构重构与组件化

### Wave 2 (Week 3-4)
- Phase 2: 测试完善
- Phase 3: UI/UX改进 (并行)

### Wave 3 (Week 5-8)
- Phase 4: 功能扩展
- Phase 5: 文档完善 (并行)

### Wave 4 (Week 9-10)
- Phase 6: 最终优化与发布

---

## 📈 Progress Tracking

| Phase | Status | Progress | Start | End |
|-------|--------|----------|-------|-----|
| Phase 1 | ✅ Complete | 100% | 2026-04-28 | 2026-04-28 |
| Phase 2 | ✅ Complete | 100% | 2026-04-28 | 2026-04-28 |
| Phase 3 | ✅ Complete | 100% | 2026-04-28 | 2026-04-28 |
| Phase 4 | ⏳ Pending | 0% | - | - |
| Phase 5 | ⏳ Pending | 0% | - | - |
| Phase 6 | ⏳ Pending | 0% | - | - |

---

*Last Updated: 2026-04-28*