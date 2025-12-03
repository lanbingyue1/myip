# myip（中文）

一个简单美观的命令行工具，用于显示本地IP地址及网络接口信息。

[English](README.md) | [中文](README_zh.md) | [日本語](README_ja.md)

## 功能特性
- 显示主IP地址（用于互联网连接的IP）并以绿色高亮
- 显示IP地址及其对应的网络接口名称
- 以CIDR格式显示IP地址（例如：192.168.1.100/24）
- 支持IPv4和IPv6地址显示
- 命令行选项用于筛选IP地址类型
- 跨平台兼容性

## 依赖库
- `clap` - 命令行参数解析
- `colored` - 彩色输出支持
- `getifs` - 网络接口信息检索

## 安装方法

### 前提条件
- 已安装Rust编程语言（版本1.56.0或更高）

### 从源代码构建
```bash
git clone <仓库URL>
cd myip
cargo build --release
```

可执行文件将位于 `target/release/myip`。

## 使用方法

### 基本用法（仅显示IPv4地址）
```bash
myip
```

![myip命令输出](static/myip.png)

### 仅显示IPv6地址
```bash
myip -6
```

![myip -6命令输出](static/myip-6.png)

### 显示所有IP地址（包括IPv4和IPv6）
```bash
myip -a
```

![myip -a命令输出](static/myip-a.png)

### 显示帮助信息
```bash
myip -h
```

![myip -h命令输出](static/myip-h.png)

## 命令行选项

| 选项 | 描述 |
|------|------|
| `-6`, `--ipv6` | 仅显示IPv6地址 |
| `-a`, `--all` | 显示所有IP地址（包括IPv4和IPv6） |
| `-h`, `--help` | 打印帮助信息 |
| `-V`, `--version` | 打印版本信息 |

## 输出示例

```
Ethernet: 192.168.1.100/24
Wi-Fi: 192.168.10.5/24  <-- 主IP（以绿色高亮显示）
Loopback: 127.0.0.1/8
```

## 许可证

MIT许可证

---

[English](README.md) | [中文](README_zh.md) | [日本語](README_ja.md)