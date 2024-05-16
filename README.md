## 环境设置
.env文件中配置
+ 鉴权密钥 -> PRIVATE_KEY
+ 账号id -> UID
+ 链接签名有效期，单位：毫秒 -> VALID_DURATION

## 部署
如果是服务器部署需要`bind("127.0.0.1:7878")`
改为`bind("0.0.0.0:7878")`

## postman 测试

![image](https://github.com/cppcloud/pan123-authenticator/assets/145106225/e7494dbc-0380-4a0f-bce6-2c8fed4b2361)


![image](https://github.com/cppcloud/pan123-authenticator/assets/145106225/7c5a4830-28db-4d7d-9d7e-b9aebd4b5c9a)


### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持


