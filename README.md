<div align="center">
    <h1>🕳️ Tunneled</h1>
    <p><em>A simple and powerful CLI tool for creating TCP tunnels</em></p>
    
[![License: MIT](https://img.shields.io/badge/License-GPL--3.0-yellow.svg)](https://opensource.org/licenses/GPL)
[![Rust](https://img.shields.io/badge/Rust-orange.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Strawberry-Foundations/tunneled/ci.yml)](https://github.com/Strawberry-Foundations/tunneled/actions)
</div>

## ✨ Features

- 🚀 **Fast & Lightweight** - Built with Rust for optimal performance
- 🔒 **Secure** - Encrypted connections with authentication support
- 🔌 **Plugin System** - Extensible architecture with dynamic plugin loading
- 🌐 **Multiple Servers** - Support for various tunnel server types
- 📦 **Easy Installation** - Single binary with no dependencies

## 🚀 Quick Start

### Installation

#### Install from cargo
We are currently working on publishing to crates.io. Please check back later.

#### Install from GitHub Releases
```bash
# for Linux x86_64
wget https://github.com/Strawberry-Foundations/tunneled/releases/latest/download/tunneled-x86_64 -O tunneled

# for Linux aarch64
wget https://github.com/Strawberry-Foundations/tunneled/releases/latest/download/tunneled-aarch64 -O tunneled

chmod +x tunneled
```

### Basic Usage

#### Local Tunneling
```bash
# Create a tunnel for local port 3000
tunneled local 3000

# Specify remote server
tunneled local 8080 --use exampleserver.org

# Use authentication
tunneled auth
tunneled local 3000 --auth
```

#### Server
```bash
# Start a tunnel server
tunneled server --min-port 5000 --max-port 6000
```

For more options, run:
```bash
tunneled help
```

## 🏗️ Available Tunnel Servers

| Server                    | Type        | Authentication | Cost | Status   |
| ------------------------- | ----------- | -------------- | ---- | -------- |
| strawberryfoundations.org | Public-Auth | ✅ Required     | Free | 🟢 Active |

### Server Types
- **Public**: No authentication required, open access
- **Public-Auth**: Free but requires account registration
- **Private-Auth**: Paid service with account authentication  
- **Private**: Password-protected, not for public use

## 🔌 Plugin System

Tunneled supports a powerful plugin system for extending functionality:

```bash
# List available plugins
tunneled plugin list

# Run a plugin
tunneled plugin example-plugin test
```

Plugins can be used to add new tunnel servers, authentication methods, or other features.<br>
Add plugins by copying them to the plugins directory:

```bash
cp /path/to/plugin.so ~/.config/tunneled/plugins/
```

## 🛠️ Configuration

Configuration files are stored in:
- **Linux/macOS**: `~/.config/tunneled/`
- **Windows**: `%APPDATA%\tunneled\`

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details (they dont exist yet 💀)

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Special thanks to:
- **[ekzhang](https://github.com/ekzhang)** for creating [bore](https://github.com/ekzhang/bore)
  
  > This project would not be possible without your foundational work. Tunneled builds upon and extends bore's excellent architecture, originally licensed under the MIT license.


## 🔗 Links

- [Issue Tracker](https://github.com/Strawberry-Foundations/tunneled/issues)
- [Changelog](CHANGELOG.md)

---

<div align="center">
    <p>Made with ❤️ and Rust</p>
</div>