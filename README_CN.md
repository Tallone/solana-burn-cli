# Solana Burn CLI

中文文档 | [English](README.md)

一个用于Solana burn token以及关闭ATA账户回收资金的TUI工具。

## 功能特性

- 🔥 **Token燃烧**: 支持燃烧指定的SPL Token
- 💰 **ATA账户关闭**: 关闭Associated Token Account并回收SOL
- 🖥️ **终端界面**: 基于ratatui的现代化终端用户界面
- ⚡ **实时数据**: 通过Solana RPC实时获取账户信息
- 🎯 **精确控制**: 可选择性地处理特定的token账户

## 界面说明

### 顶部信息栏
- 显示钱包公钥地址
- 显示当前设置（是否燃烧token，是否关闭ATA）
- 显示已选择账户数量和总账户数量

### 主表格 - Token账户列表
- 显示钱包中所有的ATA账户
- 列：选中状态（✓表示已选中）、地址（前6位...后4位）、Mint地址、余额
- 显示账户总数

### 底部控制栏
- 显示所有可用的键盘快捷键

## 操作说明

### 键盘控制

#### 普通模式
- `↑/↓`: 在表格中上下移动选择
- `Space/Enter`: 切换当前行的选中状态
- `A`: 全选所有账户
- `C`: 清除所有选择
- `F`: 进入搜索模式（按Mint地址搜索）
- `Ctrl+P`: 显示确认对话框，处理选中的账户（执行burn和close操作）
- `Q/Esc/Ctrl+C`: 退出程序

#### 搜索模式
- `输入字符`: 按Mint地址过滤账户（支持部分匹配，不区分大小写）
- `Backspace`: 删除搜索字符
- `↑/↓`: 在过滤结果中上下移动选择
- `Space`: 切换当前行的选中状态
- `Enter/Esc`: 退出搜索模式

#### 确认对话框模式
- `Y/Enter`: 确认处理选中的账户
- `N/Esc`: 取消操作，返回主界面

### 使用流程
1. 启动程序后，显示所有token账户
2. 使用方向键浏览账户列表
3. **可选**: 按F键进入搜索模式，输入Mint地址进行过滤
4. 按Space或Enter选择/取消选择要处理的账户
5. 使用A键全选或C键清除所有选择
6. 按Ctrl+P显示确认对话框
7. 在确认对话框中按Y确认或N取消处理操作

### 搜索功能
- 按F键进入搜索模式
- 输入Mint地址的任意部分进行过滤（不区分大小写）
- 搜索结果会实时更新
- 在搜索模式下仍可以选择/取消选择账户
- 按Enter或Esc退出搜索模式

### 安全确认功能
- 按Ctrl+P触发处理操作时会显示确认对话框
- 对话框显示将要处理的账户数量和操作详情
- 明确提示将执行的操作：燃烧token、关闭ATA账户、回收SOL
- 必须明确确认（按Y）才会执行操作
- 可以随时取消（按N或Esc）

## 安装和使用

### 快速安装（推荐）

#### Linux/macOS
```bash
curl -sSL https://raw.githubusercontent.com/your-username/solana-burn-cli/main/install.sh | bash
```

#### Windows (PowerShell)
```powershell
iwr -useb https://raw.githubusercontent.com/your-username/solana-burn-cli/main/install.ps1 | iex
```

### 手动安装

#### 下载预编译二进制文件
1. 访问 [Releases](https://github.com/your-username/solana-burn-cli/releases) 页面
2. 下载适合您平台的二进制文件：
   - **Linux x86_64**: `solana-burn-cli-linux-x86_64.tar.gz`
   - **Linux x86_64 (musl)**: `solana-burn-cli-linux-x86_64-musl.tar.gz`
   - **Windows x86_64**: `solana-burn-cli-windows-x86_64.zip`
   - **macOS x86_64 (Intel)**: `solana-burn-cli-macos-x86_64.tar.gz`
   - **macOS aarch64 (Apple Silicon)**: `solana-burn-cli-macos-aarch64.tar.gz`
3. 解压文件并将二进制文件移动到您的PATH中

#### 从源码编译
**前置要求**: Rust 1.70+

```bash
git clone https://github.com/your-username/solana-burn-cli.git
cd solana-burn-cli
cargo build --release
```

### 运行
```bash
# 基本用法
cargo run -- --private-key <YOUR_PRIVATE_KEY_BASE58> --rpc-url <RPC_ENDPOINT>

# 使用devnet
cargo run -- --private-key <YOUR_PRIVATE_KEY_BASE58> --rpc-url https://api.devnet.solana.com

# 使用mainnet
cargo run -- --private-key <YOUR_PRIVATE_KEY_BASE58> --rpc-url https://api.mainnet-beta.solana.com

# 只关闭ATA不燃烧token
cargo run -- --private-key <YOUR_PRIVATE_KEY_BASE58> --burn-token false

# 只燃烧token不关闭ATA
cargo run -- --private-key <YOUR_PRIVATE_KEY_BASE58> --close-ata false
```

### 命令行参数
- `-p, --private-key <PRIVATE_KEY>`: 钱包私钥（base58编码）**[必需]**
- `-r, --rpc-url <RPC_URL>`: Solana RPC端点 [默认: https://api.mainnet-beta.solana.com]
- `--burn-token <BOOL>`: 是否燃烧token [默认: true]
- `--close-ata <BOOL>`: 是否关闭ATA账户 [默认: true]

## 安全提醒

⚠️ **重要安全提示**:
- 请确保在测试网络上充分测试后再在主网使用
- 私钥信息敏感，请妥善保管
- 燃烧的token无法恢复
- 建议先在devnet上测试功能

## 技术栈

- **Rust**: 系统编程语言
- **ratatui**: 终端用户界面库
- **tokio**: 异步运行时
- **solana-client**: Solana RPC客户端
- **spl-token**: SPL Token程序接口
- **clap**: 命令行参数解析

## License

Copyright (c) Tallone <tallone.shi@outlook.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
