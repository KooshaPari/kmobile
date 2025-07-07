# KMobile 📱

> **K**ompass for **Mobile** Development - A comprehensive CLI, API, and MCP server for mobile app development and testing automation.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/kmobile-dev/kmobile/workflows/CI/badge.svg)](https://github.com/kmobile-dev/kmobile/actions)

## 🚀 Features

### Multi-Interface Support
- **CLI**: Powerful command-line interface for all operations
- **API**: RESTful API server for integration with other tools
- **MCP**: Model Context Protocol server for AI agent integration
- **TUI**: Terminal User Interface (coming soon)

### Device & Simulator Management
- 📱 **Physical Devices**: Detect, connect, and manage iOS/Android devices
- 🔧 **Simulators**: Control iOS Simulators and Android Emulators
- 🔄 **Cross-Platform**: Unified interface for both platforms

### Project Management
- 🏗️ **Multi-Platform Projects**: Support for Android, iOS, React Native, Flutter
- ⚡ **Quick Setup**: Project templates and scaffolding
- 📦 **Build Automation**: Integrated build and deployment pipelines

### Testing Automation
- 🎭 **Playwright-Style**: Comprehensive UI automation for mobile apps
- 📸 **Screenshots & Videos**: Capture test execution
- 🧪 **Test Recording**: Record and replay test scenarios
- 📊 **Reporting**: Detailed test reports and analytics

## 🛠️ Installation

### From Source
```bash
git clone https://github.com/kmobile-dev/kmobile.git
cd kmobile
cargo install --path .
```

### From Crates.io (coming soon)
```bash
cargo install kmobile
```

### From Homebrew (coming soon)
```bash
brew install kmobile
```

## 🚀 Quick Start

### 1. Initialize KMobile
```bash
kmobile init my-mobile-app --template react-native
cd my-mobile-app
```

### 2. List Available Devices
```bash
# List physical devices
kmobile device list

# List simulators
kmobile simulator list
```

### 3. Start a Simulator
```bash
# Start iOS simulator
kmobile simulator start "iPhone 15 Pro"

# Start Android emulator
kmobile simulator start "Pixel_7_API_34"
```

### 4. Deploy Your App
```bash
# Build and deploy to device
kmobile device deploy <device-id>

# Deploy to simulator
kmobile simulator install <simulator-id> ./app.apk
```

### 5. Run Tests
```bash
# Run test suite
kmobile test run --suite e2e --device <device-id>

# Record a new test
kmobile test record --output my-test.json
```

## 📖 Documentation

### CLI Commands

#### Device Management
```bash
kmobile device list                           # List connected devices
kmobile device connect <device-id>           # Connect to device
kmobile device install <device-id> <app>     # Install app
kmobile device deploy <device-id>            # Deploy current project
kmobile device test <device-id> [suite]      # Run tests on device
```

#### Simulator Management
```bash
kmobile simulator list                        # List available simulators
kmobile simulator start <simulator-id>       # Start simulator
kmobile simulator stop <simulator-id>        # Stop simulator
kmobile simulator reset <simulator-id>       # Reset simulator
kmobile simulator install <sim-id> <app>     # Install app on simulator
```

#### Project Management
```bash
kmobile init <name> [--template <template>]  # Initialize new project
kmobile project build [--target <target>]    # Build project
kmobile project clean                         # Clean build artifacts
kmobile project status                        # Show project status
```

#### Testing
```bash
kmobile test run [--suite <suite>] [--device <id>]  # Run tests
kmobile test record --output <file>                 # Record test
kmobile test replay --file <file>                   # Replay test
```

#### Servers
```bash
kmobile serve --host localhost --port 3000          # Start API server
kmobile mcp [--config <file>]                       # Start MCP server
kmobile tui                                          # Start TUI (coming soon)
```

### API Server

Start the API server and access the REST endpoints:

```bash
kmobile serve --port 3000
```

Key endpoints:
- `GET /devices` - List connected devices
- `GET /simulators` - List available simulators
- `POST /deploy` - Deploy app to device/simulator
- `POST /test/run` - Run test suite
- `GET /project/status` - Get project status

### MCP Server

The MCP (Model Context Protocol) server enables AI agents to interact with KMobile:

```bash
# Start MCP server
kmobile mcp

# Or as a binary
kmobile-mcp --config kmobile.toml
```

Available MCP tools:
- `device_list` - List connected devices
- `device_connect` - Connect to a device
- `simulator_start` - Start a simulator
- `project_build` - Build the project
- `test_run` - Run tests

### Configuration

KMobile uses a `kmobile.toml` configuration file:

```toml
[project]
name = "MyApp"
version = "1.0.0"

[android]
sdk_path = "/usr/local/android-sdk"
adb_path = "/usr/local/android-sdk/platform-tools/adb"

[ios]
xcode_path = "/Applications/Xcode.app"
simctl_path = "/usr/bin/simctl"

[testing]
framework = "kmobile"
timeout = 30
screenshot_on_failure = true
output_dir = "./test-results"

[mcp]
enabled = true
port = 3001
tools = ["device_list", "simulator_control", "test_run"]

[api]
enabled = true
port = 3000
```

## 🧩 Project Templates

KMobile supports multiple project templates:

### Android
```bash
kmobile init MyApp --template android
```

### iOS
```bash
kmobile init MyApp --template ios
```

### React Native
```bash
kmobile init MyApp --template react-native
```

### Flutter
```bash
kmobile init MyApp --template flutter
```

## 🔧 Integration

### With VS Code / Cursor
Add KMobile MCP server to your editor configuration.

### With CI/CD
```yaml
# GitHub Actions example
- name: Setup KMobile
  run: |
    cargo install kmobile
    kmobile device list

- name: Run Tests
  run: |
    kmobile test run --suite ci
```

### With Other Tools
Use KMobile's REST API to integrate with your existing toolchain.

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
```bash
git clone https://github.com/kmobile-dev/kmobile.git
cd kmobile
cargo build
cargo test
```

### Running Tests
```bash
cargo test
cargo test --features integration
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Documentation](https://kmobile-dev.github.io/kmobile)
- [API Reference](https://kmobile-dev.github.io/kmobile/api)
- [Issues](https://github.com/kmobile-dev/kmobile/issues)
- [Discussions](https://github.com/kmobile-dev/kmobile/discussions)

## 🙏 Acknowledgments

- Inspired by [Playwright](https://playwright.dev/) for web testing
- Built on the [Model Context Protocol](https://modelcontextprotocol.io/)
- Thanks to the Rust mobile development community

---

Made with ❤️ for the mobile development community