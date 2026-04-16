# 🛠️ v0k 项目设计文档 (v1.0)

**项目定位**：开发者专属的“语义级”智能命令行代理。
**核心理念**：不重复造轮子。通过 Rust 驱动的小模型 Agent，将用户的自然语言或模糊意图，编译为高性能系统原生命令（如 `curl`, `git`, `ffmpeg`）的精准调用。

---

## 1. 架构设计 (Architecture)

v0k 采用 **“三核架构”**，确保在不同环境下的极致兼容性与速度。

### 1.1 分发层 (The Dispatcher - Node.js)

- **职责**：作为全局 CLI 入口，处理跨平台安装与环境预检。
- **理由**：利用 npm 的无处不在，解决二进制文件分发的最后 1 公里问题。
- **行为**：检测并调用对应系统的原生 Rust 二进制内核；若不存在则自动下载。

### 1.2 推理内核 (The Agent Kernel - Rust)

- **职责**：意图识别、参数纠错、子进程管理。
- **组件**：
  - **Parser**: 快速静态解析（Levenshtein 距离纠错）。
  - **Brain**: 嵌入式小模型引擎（基于 `candle` + Qwen-0.5B）。
  - **Executor**: 异步子进程管理（基于 `tokio::process`）。

### 1.3 逻辑流 (The Logic Flow)

1.  **输入**：用户输入模糊命令（如 `v0k curl POST /users name=jack`）。
2.  **初筛**：Rust 检测是否完全匹配标准参数。
3.  **补全/纠错**：若不匹配，Agent 介入，通过历史上下文（Context）和语义理解，补全缺失参数（如 URL 前缀、Headers）。
4.  **代理执行**：将转换后的命令派生（Spawn）给系统原生工具。

---

## 2. 核心功能模块 (Core Modules)

### 2.1 智能包装器 (Smart Wrapper)

- **协议适配层**：内置对常用硬核工具（`curl`, `git`, `openssl`, `ffmpeg`, `docker`）的语义映射。
- **透明执行**：在执行前，控制台会以极简 UI 提示：`→ Running: curl -X POST ...`。
- **安全沙箱**：通过强类型参数绑定，防止任何形式的命令注入。

### 2.2 原子工具集 (Atomic Utilities)

内置于 Rust 内核中的高频免配置工具：

- **`v0k ts`**: 全能时间戳处理（自动识别输入格式）。
- **`v0k b64`**: 自动感知编码/解码。
- **`v0k json`**: 极速格式化与字段提取。

### 2.3 状态管理 (State & Context)

- **Local DB (Sled)**：记录最近 100 条成功执行的命令意图，作为小模型推理的 Few-shot 上下文，提升识别准确度。
- **Environment Awareness**：感知当前目录下的 `package.json` 或 `.env` 文件，自动推断 Base URL 或环境变量。

---

## 3. 交互逻辑 (UX Design)

### 3.1 混合模式指令 (Hybrid Commands)

v0k 必须支持三种输入维度：

1.  **纯指令**：`v0k ts now` (执行毫秒级响应)。
2.  **模糊参数**：`v0k curl POST /api id=1` (Agent 补全)。
3.  **纯自然语言**：`v0k "帮我把刚才的 JSON 存到 output.txt"` (Agent 全力驱动)。

当前实现约束：

- 已实现的内建包装器（目前为 `curl`）仍然优先走“静态解析 → AI fallback”。
- 未实现包装器的顶层命令不会再被 CLI 直接拦截，而是按原始 `program + args` 透传执行。
- 如果配置了 AI，未知命令会在执行前做一次审阅；若 AI 建议改写命令，必须显式确认后才会执行建议命令。
- 如果未配置 AI，未知命令仍直接透传，不阻断执行。

### 3.2 确认与反馈

- **低置信度警告**：当 Agent 转化命令的置信度低于 85% 时，强制进入交互模式：
  > `v0k` 想执行: `git push origin --force`
  > ⚠️ **这具有破坏性，是否确认？ [y/N]**

---

## 4. 技术栈 (Tech Stack)

| 模块              | 技术方案             | 关键特性                         |
| :---------------- | :------------------- | :------------------------------- |
| **CLI Runtime**   | Node.js (Dispatcher) | 极致的分发便利性。               |
| **Core Engine**   | Rust (Static Binary) | 零内存安全隐患，极致启动速度。   |
| **Async / IO**    | `tokio`              | 高并发子进程管理。               |
| **Inference**     | `candle` + GGUF      | 纯 Rust 实现的无依赖模型推理。   |
| **Persistence**   | `sled`               | 轻量级、线程安全的嵌入式 KV 库。 |
| **Communication** | Stdio Pipe           | 完美融入 Unix 管道哲学。         |

---

## 5. 路线图 (Roadmap)

### 第一阶段：MVP (最小可行性产品)

- [ ] 实现 Node.js 到 Rust 内核的透传。
- [ ] 支持 `v0k curl` 的基础语义转换。
- [ ] 集成内置时间戳和 Base64 原子工具。

### 第二阶段：Intelligence (智能化提升)

- [ ] 嵌入 Qwen-0.5B 模型，支持复杂参数纠错。
- [ ] 实现基于 `sled` 的本地命令上下文学习。
- [ ] 增加 `v0k git` 和 `v0k docker` 语义层。

### 第三阶段：Ecosystem (生态扩展)

- [ ] 支持用户通过简单的 JSON 配置自定义“包装器”。
- [ ] 实现 `v0k --explain` 教学模式。
- [ ] 插件系统：允许开发者用 JS 编写自定义的 Agent 逻辑插件。

---

## 6. 设计哲学总结

> **v0k 不创造新的命令，它赋予旧命令以灵魂。**
> 它旨在消除开发者在“想要做什么”和“怎么查文档写参数”之间的摩擦力。
