# Phase 4: UI/UX Bug Fixes - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-28
**Phase:** 4-UI/UX Bug Fixes
**Areas discussed:** 生产级布局, 操作控件一致性, 输入与 IMK, 滚动与复制

---

## 生产级布局

| Decision point | Options considered | User's choice |
|---|---|---|
| Overall layout style | 工具台式布局 / 文档式布局 / 控制台式布局 / 你决定 | 工具台式布局 |
| Right content organization | 表单上结果下 / 左右分栏 / 卡片纵向流 / 你决定 | 表单上结果下 |
| Typography and spacing priority | 清晰优先 / 紧凑优先 / 大屏优先 / 你决定 | 清晰优先 |
| Narrow/long-content behavior | 固定左栏+右侧滚动 / 整体滚动 / 折叠左栏 / 你决定 | 固定左栏+右侧滚动 |
| Result card emphasis | 低强调卡片 / 强卡片层级 / 纯文本面板 / 你决定 | 低强调卡片 |
| Color theme | 保留深色主题 / 改成浅色主题 / 支持明暗切换 / 你决定 | 保留深色主题 |
| Empty/error state expression | 内联提示 / 顶部横幅 / 弹窗提示 / 你决定 | 顶部横幅 |
| Layout optimization boundary | 修复+统一即可 / 全面重构 UI / 只修 bug / 你决定 | 全面重构 UI |
| Scope clarification | 现有能力内重构 / 允许新增 UI 能力 / 退回修复+统一 | 现有能力内重构 |

**Notes:** The user wants production-level layout work. Scope was clarified to allow refactoring current UI/layout components, while avoiding new capabilities such as theme switching or navigation redesign.

---

## 操作控件一致性

| Decision point | Options considered | User's choice |
|---|---|---|
| Main action area shape | 紧凑按钮组 / 全宽主按钮 / 顶部工具栏 / 你决定 | 紧凑按钮组 |
| Reset behavior | 清空本页全部状态 / 只清空输入 / 只清空结果 / 你决定 | 清空本页全部状态 |
| Execution feedback | 顶部横幅+结果区 / 只在结果区 / 按钮旁短提示 / 你决定 | 顶部横幅+结果区 |
| Coverage for visual consistency | 证书+算法全部 / 只算法页 / 只问题页面 / 你决定 | 证书+算法全部 |
| Button order | 执行在左重置在右 / 重置在左执行在右 / 按页面语义决定 / 你决定 | 执行在左重置在右 |
| Enter key behavior | 触发执行 / 只换行或无操作 / 仅 OID 查询触发 / 你决定 | 触发执行 |
| Invalid input behavior | 允许点击并报错 / 禁用按钮 / 边输入边校验 / 你决定 | 允许点击并报错 |
| Copy button placement | 结果标题行右侧 / 主按钮组旁 / 每个字段旁 / 你决定 | 结果标题行右侧 |

**Notes:** The app should use consistent compact controls across certificate and algorithm tabs, with clear status feedback and reset semantics.

---

## 输入与 IMK

| Decision point | Options considered | User's choice |
|---|---|---|
| Input capability target | 完整文本输入 / 只支持 ASCII / 最小可输入 / 你决定 | 完整文本输入 |
| macOS IMK target | 消除并稳定输入 / 只要能输入即可 / 只隐藏或忽略日志 / 你决定 | 消除并稳定输入 |
| Input visual states | 聚焦/错误/禁用态 / 只有聚焦态 / 无特殊状态 / 你决定 | 聚焦/错误/禁用态 |
| Long text inputs | 支持多行文本框 / 统一单行输入 / 文件导入为主 / 你决定 | 支持多行文本框 |

**Notes:** The current raw keystroke/focused-field model is not considered sufficient for production input behavior. The desired outcome is stable real text input including IME and paste behavior.

---

## 滚动与复制

| Decision point | Options considered | User's choice |
|---|---|---|
| Scroll boundary | 右侧内容整体滚动 / 每个结果卡片滚动 / 整体窗口滚动 / 你决定 | 右侧内容整体滚动 |
| Long output display | 自动换行+可复制 / 等宽块+横向滚动 / 截断+展开 / 你决定 | 等宽块+横向滚动 |
| Copy granularity | 整块+关键字段 / 只复制整块 / 每一行都可复制 / 你决定 | 整块+关键字段 |
| Copy success feedback | 顶部短提示 / 按钮文字变更 / 无反馈 / 你决定 | 顶部短提示 |

**Notes:** Long cryptographic/certificate material should preserve fidelity through monospaced horizontal-scroll blocks while still allowing convenient whole-block and key-field copy.

---

## the agent's Discretion

- Exact GPUI implementation approach for real input/IME handling.
- Exact reusable component extraction boundaries.
- Exact spacing/color values within the locked layout direction.

## Deferred Ideas

- Theme switching / adding a light theme.
- Collapsible left navigation or new navigation behavior.
