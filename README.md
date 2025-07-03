# RuoYi-Vue Rust 重写项目

本项目基于 [abs-admin](https://github.com/rbatis/abs_admin) 开发，并参考了 [若依](https://gitee.com/y_project) 的 [RuoYi-Vue](https://gitee.com/y_project/RuoYi-Vue) 项目，使用 Rust 语言进行重写。

我们正在寻找志同道合的开发者共同完成这个项目。目前部分功能已经实现，但还需要进一步完善。

## 技术栈

- Rust 语言
- rbatis ORM 框架
- Axum Web 框架
- rbatis-sql 用于 SQL 映射
- JWT 用于身份验证
- Redis/Memcached 用于缓存
- Rust-xlsxwriter 用于 Excel 导出
- MiniJinja 模板引擎

## 功能模块

- 系统管理：用户管理、角色管理、菜单管理、部门管理、岗位管理、字典管理、参数设置等
- 监控管理：在线用户、登录日志、操作日志
- 代码生成：支持从数据库表自动生成代码
- 文件存储：支持本地存储和 S3/OSS 存储
- 数据权限控制
- 多租户支持

## 项目结构

- `src/main.rs`: 项目入口
- `src/lib.rs`: 核心库文件
- `src/config`: 配置相关模块
- `src/context.rs`: 全局上下文
- `src/error.rs`: 错误处理
- `src/modules`: 各功能模块
  - `system`: 系统管理模块
  - `gen`: 代码生成模块
  - `oa`: 办公自动化模块（待实现）
- `src/utils`: 工具类
- `src/token_auth`: 身份验证中间件
- `src/web_data.rs`: Web 请求上下文数据
- `macros`: 自定义宏定义

## 开发状态

- ✅ 功能已实现，但需要完善
- ⚠️ 功能部分实现
- ❌ 功能未实现

## 贡献指南

我们欢迎任何有兴趣的开发者加入项目。您可以从以下几个方面参与贡献：

1. 完善已实现的功能模块
2. 实现未完成的功能
3. 编写单元测试
4. 优化代码结构和性能
5. 完善文档

## 安装和运行

请参考项目中的 `application.yml` 文件进行配置，然后运行：

```bash
cargo run
```

## 致谢

感谢 [若依](https://gitee.com/y_project) 提供的优秀项目模板，以及 [abs-admin](https://github.com/rbatis/abs_admin) 提供的 Rust 开发基础。