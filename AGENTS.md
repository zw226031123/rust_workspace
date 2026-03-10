# Rust 工作区项目概述

## 项目描述

这是一个综合性的 Rust 工作区，包含多个项目，展示了 Rust 编程的各个方面，从基本网络编程到 Web 开发和异步编程。工作区结构为在单个 Cargo 工作区下管理的相关包集合，使用 "1" 版本的解析器。这些项目展示了不同的 Rust 概念和库，包括 TCP 网络、HTTP 协议、Web 框架、异步编程和数据库集成。

## 工作区成员

工作区包含以下项目：

1. **tcp_server** - 基本 TCP 服务器实现 (edition 2024)
2. **tcp_client** - 与服务器通信的 TCP 客户端 (edition 2024)
3. **http** - HTTP 请求/响应解析库 (edition 2024)
4. **http_server** - HTTP 服务器实现，使用 http crate、serde 和 serde_json (edition 2024)
5. **web_service** - 全功能 Web 服务，使用 Actix-web 和 PostgreSQL 集成 (edition 2024)
6. **web_app** - Web 应用程序框架，使用 Actix-web 和 Tera 模板 (edition 2024)
7. **rust_game** - 使用 bracket-lib 的简单游戏实现 (edition 2024)
8. **mini_grep** - 类似于 Rust 书籍中的 grep 类文本搜索工具 (edition 2024)
9. **rust_elasticsearch** - 使用 elasticsearch crate、serde 和 tokio 的 Elasticsearch 集成示例 (edition 2024)
10. **rust_async** - 使用 async-std、futures 和 Tokio 的异步编程示例 (edition 2024)
11. **rust_server** - 使用 async-std 和 futures 的另一个服务器实现 (edition 2024)

## 项目详情

### rust_async
该项目专注于异步 Rust 编程，展示了：
- futures 和 async/await 语法的使用
- 基于通道的通信
- 流处理
- Tokio 运行时示例
- 现代异步 Rust 模式 (edition 2024)

### web_service
全功能 Web 服务包含：
- 针对课程和教师的 REST API 端点
- 使用 SQLx 的 PostgreSQL 数据库集成
- Actix-web 框架进行 HTTP 处理
- 通过 actix-cors 的 CORS 支持
- 使用 dotenv 的环境配置
- Chrono 与 serde 支持的日期/时间处理
- 具有 vendored 功能的 OpenSSL 用于 TLS 支持
- 包含二进制可执行文件：server, teacher-service-v2
- 使用 edition 2024

### http
基础 HTTP 库提供：
- HTTP 请求解析功能
- HTTP 响应构建
- 对不同 HTTP 方法和版本的支持
- 头部和正文解析
- 无外部依赖 (edition 2024)

### http_server
HTTP 服务器实现基于 http crate：
- 使用来自此工作区的自定义 http crate
- 使用 serde 和 serde_json 进行 JSON 序列化/反序列化
- 静态文件服务功能
- 基本路由功能
- 健康检查端点
- 使用 edition 2024

### tcp_server/tcp_client
基本 TCP 网络示例展示：
- Rust 中的套接字编程
- 服务器-客户端通信模式
- 通过 TCP 连接读写数据
- 除标准库外无外部依赖
- 现代 edition 2024 代码

### rust_elasticsearch
使用 Rust 的 Elasticsearch 示例包括：
- Elasticsearch 客户端库版本 8.19.0-alpha.1
- 使用 Tokio 的异步运行时
- 使用 serde 和 serde_json 的 JSON 处理
- 现代 async/await 模式
- 使用 edition 2024

### web_app
Web 应用程序框架包含：
- 用于 HTTP 处理的 Actix-web
- Tera 模板引擎用于动态内容
- 用于静态文件服务的 Actix-files
- 使用 awc 的 HTTP 客户端功能
- 支持表单处理和 JSON API
- 使用 edition 2024

### rust_game
使用以下技术的简单游戏实现：
- 用于游戏开发的 bracket-lib crate
- 回合制游戏机制
- 基于控制台的用户界面
- 使用 edition 2024

### mini_grep
展示以下功能的类 grep 文本搜索工具：
- 命令行参数解析
- 文件 I/O 操作
- 字符串处理技术
- 错误处理模式
- 类似于"Rust 程序设计语言"书中的实现
- 使用 edition 2024

### rust_server
展示以下内容的另一个服务器实现：
- 使用 async-std 的异步编程
- 用于处理异步操作的 Futures
- 用于异步函数的属性宏
- 使用 edition 2024

## 构建和运行

构建整个工作区：
```bash
cargo build
```

构建特定项目：
```bash
cargo build -p project_name
```

运行特定项目：
```bash
cargo run -p project_name
```

某些项目可能需要特定的配置文件或环境变量。例如，web_service 项目需要包含数据库连接详情的 `.env` 文件。

运行特定的二进制目标：
```bash
cargo run -p web_service --bin teacher-service-v2
```

## 测试

在工作区中运行测试：
```bash
cargo test
```

为特定项目运行测试：
```bash
cargo test -p project_name
```

## 开发约定

- 所有项目使用 Rust edition 2024
- 异步代码使用 async/await 语法和适当的运行时
- 错误处理遵循使用 Result 和 Option 类型的 Rust 惯用法
- 项目使用适合其用例的外部 crate
- 代码按照 Rust 最佳实践组织在模块中
- 项目遵循语义化版本控制（从 0.1.0 开始）

## 依赖项

工作区使用多种 Rust crate，包括：
- **Actix 生态系统** (actix-web、actix-rt、actix-cors、awc、actix-files) 用于 Web 开发
- **Tokio** 用于多个项目的异步运行时
- **SQLx** 用于 web_service 中的数据库访问
- **Serde** 用于多个项目中的序列化/反序列化
- **Chrono** 用于带 serde 支持的日期/时间处理
- **Elasticsearch** 客户端用于搜索集成
- **Bracket-lib** 用于 rust_game 中的游戏开发
- **Async-std** 用于某些项目的替代异步运行时
- **Futures** 用于异步编程原语
- **OpenSSL** 使用 vendored 功能支持 TLS
- **Tera** 用于 Web 应用程序中的模板
- **Dotenv** 用于环境变量管理