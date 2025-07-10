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

## KMobile MCP Server

A Model Context Protocol (MCP) server that provides mobile development and testing automation capabilities. This server enables LLMs to interact with mobile devices, simulators, and development workflows through structured mobile automation, bypassing the need for manual CLI operations or complex SDK integrations.

### Key Features

- **Cross-Platform Mobile Support**: Unified interface for iOS and Android devices and simulators
- **Intelligent Device Management**: Automatic device detection, connection, and lifecycle management
- **Mobile-First Testing**: Comprehensive UI automation specifically designed for mobile applications
- **Project-Aware Operations**: Context-aware operations that understand mobile project structures
- **Real-Time Monitoring**: Live device status, build progress, and test execution monitoring
- **Deterministic Tool Application**: Structured mobile operations that avoid platform-specific complexities

### Requirements

- Rust 1.70 or newer
- Platform-specific SDKs (Android SDK, Xcode for iOS)
- VS Code, Cursor, Windsurf, Claude Desktop or any other MCP client

### Getting Started

First, install the KMobile MCP server with your client. A typical configuration looks like this:

```json
{
  "mcpServers": {
    "kmobile": {
      "command": "kmobile",
      "args": [
        "mcp"
      ]
    }
  }
}
```

**Install in VS Code** | **Install in VS Code Insiders** | **Install in Cursor** | **Install in Windsurf** | **Install in Claude Desktop** | **Install in Claude Code** | **Install in Qodo Gen**

### Configuration

KMobile MCP server supports the following arguments. They can be provided in the JSON configuration above, as part of the "args" list:

```bash
> kmobile mcp --help
  --config <path>              Path to the configuration file (kmobile.toml)
  --port <port>                Port to listen on for SSE transport
  --host <host>                Host to bind server to. Default is localhost. Use
                               0.0.0.0 to bind to all interfaces
  --android-sdk <path>         Path to Android SDK directory
  --ios-sim-path <path>        Path to iOS Simulator tools
  --device-timeout <seconds>   Device connection timeout in seconds
  --test-output-dir <path>     Directory for test artifacts and screenshots
  --log-level <level>          Log level: trace, debug, info, warn, error
  --tools <tools>              Comma-separated list of tools to enable:
                               device, simulator, project, testing, all
  --headless                   Run device operations in headless mode
  --no-auto-install            Disable automatic app installation
  --record-sessions            Record device interaction sessions
  --parallel-devices           Enable parallel device operations
  --cache-builds               Enable build artifact caching
```

### Project Configuration

KMobile MCP can be configured using a `kmobile.toml` configuration file:

```toml
[project]
name = "MyMobileApp"
version = "1.0.0"
platforms = ["android", "ios"]

[android]
sdk_path = "/usr/local/android-sdk"
api_level = 34
build_tools = "34.0.0"
emulator_name = "Pixel_7_API_34"

[ios]
xcode_path = "/Applications/Xcode.app"
simulator_name = "iPhone 15 Pro"
ios_version = "17.0"

[mcp]
enabled = true
port = 3001
tools = ["device", "simulator", "project", "testing"]
auto_detect_devices = true
session_recording = true

[testing]
framework = "kmobile"
timeout = 60
screenshot_on_failure = true
video_recording = true
output_dir = "./test-results"
parallel_execution = true
```

### Standalone MCP Server

When running on systems without display or from worker processes of IDEs, run the MCP server with the `--port` flag to enable SSE transport:

```bash
kmobile mcp --port 8931
```

And then in MCP client config, set the url to the SSE endpoint:

```json
{
  "mcpServers": {
    "kmobile": {
      "url": "http://localhost:8931/sse"
    }
  }
}
```

### Docker Support

Run KMobile MCP server in a containerized environment:

```dockerfile
FROM rust:1.70
RUN cargo install kmobile
EXPOSE 8931
CMD ["kmobile", "mcp", "--port", "8931", "--host", "0.0.0.0"]
```

### Tools

The KMobile MCP server provides comprehensive mobile development tools organized into functional categories:

#### Device Management Tools
- `device_list` - List all connected physical devices with detailed information
- `device_connect` - Establish connection to a specific device
- `device_disconnect` - Safely disconnect from device
- `device_info` - Get detailed device specifications and capabilities
- `device_screenshot` - Capture device screen
- `device_logs` - Stream device logs in real-time

#### Simulator Management Tools
- `simulator_list` - List available simulators/emulators
- `simulator_start` - Start simulator with specified configuration
- `simulator_stop` - Stop running simulator
- `simulator_reset` - Reset simulator to clean state
- `simulator_install_app` - Install application on simulator
- `simulator_uninstall_app` - Remove application from simulator

#### Project Operations Tools
- `project_init` - Initialize new mobile project with templates
- `project_build` - Build project for specified platform
- `project_clean` - Clean build artifacts
- `project_status` - Get comprehensive project status
- `project_dependencies` - Manage project dependencies

#### Testing Automation Tools
- `test_run` - Execute test suites with configurable options
- `test_record` - Record user interactions for test creation
- `test_replay` - Replay recorded test scenarios
- `test_generate` - AI-powered test generation from app analysis
- `test_report` - Generate comprehensive test reports

#### App Management Tools
- `app_install` - Install application on device/simulator
- `app_uninstall` - Remove application
- `app_launch` - Launch application with parameters
- `app_terminate` - Force terminate application
- `app_background` - Send application to background

#### Development Workflow Tools
- `workflow_ci_setup` - Configure CI/CD pipeline for mobile projects
- `workflow_deploy` - Deploy applications to app stores (development)
- `workflow_signing` - Manage code signing and certificates

### Integration Examples

#### VS Code Integration
Configure KMobile MCP in your VS Code settings:

```json
{
  "mcp.servers": {
    "kmobile": {
      "command": "kmobile",
      "args": ["mcp", "--tools", "all", "--log-level", "info"]
    }
  }
}
```

#### CI/CD Integration
```yaml
# GitHub Actions example
name: Mobile CI with KMobile MCP
on: [push, pull_request]

jobs:
  test:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup KMobile
        run: |
          cargo install kmobile
          kmobile mcp --config ci.toml &
      
      - name: Run Mobile Tests
        run: |
          # MCP client calls KMobile tools
          curl -X POST http://localhost:3001/tools/test_run \
            -d '{"suite": "regression", "platforms": ["ios", "android"]}'
```

#### Custom Tool Integration
Integrate KMobile MCP with your existing development tools:

```bash
# Example: Integration with custom deployment script
kmobile mcp --port 9000 &
MCP_PID=$!

# Your deployment script can now call MCP tools
curl -X POST http://localhost:9000/tools/project_build \
  -d '{"platform": "android", "variant": "release"}'

kill $MCP_PID
```

### Advanced Usage

#### Parallel Device Testing
Configure multiple devices for parallel test execution:

```json
{
  "mcpServers": {
    "kmobile": {
      "command": "kmobile",
      "args": [
        "mcp",
        "--parallel-devices",
        "--tools", "device,testing",
        "--device-timeout", "30"
      ]
    }
  }
}
```

#### Session Recording
Enable comprehensive session recording for debugging:

```json
{
  "mcpServers": {
    "kmobile": {
      "command": "kmobile", 
      "args": [
        "mcp",
        "--record-sessions",
        "--test-output-dir", "./debug-sessions"
      ]
    }
  }
}
```

The KMobile MCP server is designed to be the bridge between AI agents and mobile development workflows, providing reliable, structured access to mobile development operations that traditionally require deep platform knowledge.

## Configuration

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

### Editor Integration

#### VS Code
Add KMobile MCP server to your VS Code settings. Create or edit your `settings.json`:

```json
{
  "mcp.servers": {
    "kmobile": {
      "command": "kmobile",
      "args": ["mcp"],
      "env": {
        "KMOBILE_LOG_LEVEL": "info"
      }
    }
  }
}
```

#### Cursor
Configure KMobile MCP in Cursor by adding to your MCP configuration:

```json
{
  "mcpServers": {
    "kmobile": {
      "command": "kmobile",
      "args": ["mcp", "--tools", "all"],
      "description": "Mobile development automation"
    }
  }
}
```

#### Windsurf
Add KMobile to your Windsurf MCP configuration:

```json
{
  "mcpServers": {
    "kmobile": {
      "command": "kmobile",
      "args": ["mcp", "--config", "windsurf.toml"],
      "cwd": "${workspaceFolder}"
    }
  }
}
```

#### Claude Desktop
Configure KMobile in Claude Desktop's configuration file:

```json
{
  "mcpServers": {
    "kmobile": {
      "command": "kmobile",
      "args": ["mcp", "--port", "3001"],
      "env": {
        "ANDROID_HOME": "/usr/local/android-sdk",
        "XCODE_PATH": "/Applications/Xcode.app"
      }
    }
  }
}
```

#### Claude Code
Add KMobile to your Claude Code MCP servers:

```json
{
  "mcpServers": {
    "kmobile": {
      "command": "kmobile",
      "args": ["mcp", "--headless", "--tools", "device,simulator,testing"]
    }
  }
}
```

### Advanced Configuration Examples

#### Development Environment
For local development with full features:

```json
{
  "mcpServers": {
    "kmobile": {
      "command": "kmobile",
      "args": [
        "mcp",
        "--config", "./kmobile.toml",
        "--tools", "all",
        "--log-level", "debug",
        "--record-sessions",
        "--parallel-devices"
      ],
      "env": {
        "KMOBILE_OUTPUT_DIR": "./debug-output",
        "KMOBILE_CACHE_BUILDS": "true"
      }
    }
  }
}
```

#### Production/CI Environment
For CI/CD and production environments:

```json
{
  "mcpServers": {
    "kmobile": {
      "command": "kmobile",
      "args": [
        "mcp",
        "--headless",
        "--tools", "device,simulator,testing",
        "--device-timeout", "60",
        "--no-auto-install"
      ],
      "env": {
        "KMOBILE_LOG_LEVEL": "warn",
        "KMOBILE_CI_MODE": "true"
      }
    }
  }
}
```

#### Remote/Standalone Server
For remote MCP server setup:

```json
{
  "mcpServers": {
    "kmobile": {
      "url": "http://your-kmobile-server:8931/sse",
      "description": "Remote KMobile MCP server"
    }
  }
}
```

### CI/CD Integration

#### GitHub Actions
Complete GitHub Actions workflow with KMobile MCP:

```yaml
name: Mobile CI with KMobile
on: [push, pull_request]

jobs:
  test:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Install KMobile
        run: |
          cargo install kmobile
          kmobile --version
          
      - name: Setup Android SDK
        uses: android-actions/setup-android@v3
        
      - name: Start KMobile MCP Server
        run: |
          kmobile mcp --port 8931 --headless --tools testing &
          echo $! > kmobile.pid
          sleep 5
          
      - name: Run Mobile Tests
        run: |
          # Use MCP client to run tests
          curl -X POST http://localhost:8931/tools/test_run \
            -H "Content-Type: application/json" \
            -d '{
              "suite": "ci",
              "platforms": ["android"],
              "headless": true,
              "output_format": "junit"
            }'
            
      - name: Upload Test Results
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: test-results
          path: ./test-results/
          
      - name: Stop KMobile MCP Server
        if: always()
        run: |
          kill $(cat kmobile.pid) || true
```

#### GitLab CI
GitLab CI configuration with KMobile:

```yaml
stages:
  - test
  - deploy

mobile_test:
  stage: test
  image: rust:latest
  before_script:
    - apt-get update && apt-get install -y android-sdk
    - cargo install kmobile
  script:
    - kmobile mcp --port 8931 --headless &
    - sleep 5
    - kmobile test run --suite regression --output junit
  artifacts:
    reports:
      junit: test-results/junit.xml
    paths:
      - test-results/
```

#### Jenkins Pipeline
Jenkins pipeline integration:

```groovy
pipeline {
    agent any
    
    stages {
        stage('Setup') {
            steps {
                sh 'cargo install kmobile'
            }
        }
        
        stage('Test') {
            steps {
                sh '''
                    kmobile mcp --port 8931 --headless &
                    MCP_PID=$!
                    sleep 5
                    
                    # Run tests via MCP
                    curl -X POST http://localhost:8931/tools/test_run \
                      -d '{"suite": "smoke", "platforms": ["android", "ios"]}'
                    
                    kill $MCP_PID
                '''
            }
        }
    }
    
    post {
        always {
            publishTestResults testResultsPattern: 'test-results/*.xml'
        }
    }
}
```

### Docker Integration

#### Dockerfile
Create a containerized KMobile MCP server:

```dockerfile
FROM rust:1.75-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# Install Android SDK
ENV ANDROID_HOME=/opt/android-sdk
RUN mkdir -p $ANDROID_HOME && \
    curl -o sdk-tools.zip https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip && \
    unzip sdk-tools.zip -d $ANDROID_HOME && \
    rm sdk-tools.zip

# Install KMobile
RUN cargo install kmobile

# Configuration
COPY kmobile.toml /etc/kmobile/
EXPOSE 8931

# Run MCP server
CMD ["kmobile", "mcp", "--port", "8931", "--host", "0.0.0.0", "--config", "/etc/kmobile/kmobile.toml"]
```

#### Docker Compose
Multi-service setup with KMobile:

```yaml
version: '3.8'

services:
  kmobile-mcp:
    build: .
    ports:
      - "8931:8931"
    volumes:
      - ./kmobile.toml:/etc/kmobile/kmobile.toml
      - ./test-results:/app/test-results
    environment:
      - KMOBILE_LOG_LEVEL=info
      - KMOBILE_HEADLESS=true
      
  test-runner:
    image: node:18
    depends_on:
      - kmobile-mcp
    volumes:
      - ./tests:/app/tests
    command: |
      sh -c "
        npm install
        npm run test:mobile
      "
```

### IDE Plugin Integration

#### VS Code Extension
Example package.json for VS Code extension integrating KMobile MCP:

```json
{
  "contributes": {
    "configuration": {
      "title": "KMobile",
      "properties": {
        "kmobile.mcp.enabled": {
          "type": "boolean",
          "default": true,
          "description": "Enable KMobile MCP server"
        },
        "kmobile.mcp.port": {
          "type": "number",
          "default": 3001,
          "description": "KMobile MCP server port"
        }
      }
    },
    "commands": [
      {
        "command": "kmobile.startMCP",
        "title": "Start KMobile MCP Server"
      },
      {
        "command": "kmobile.deviceList",
        "title": "List Mobile Devices"
      }
    ]
  }
}
```

### API Integration

#### REST API Client
Example Python client for KMobile REST API:

```python
import requests
import json

class KMobileClient:
    def __init__(self, base_url="http://localhost:3000"):
        self.base_url = base_url
        
    def list_devices(self):
        response = requests.get(f"{self.base_url}/devices")
        return response.json()
        
    def run_tests(self, suite="default", device_id=None):
        payload = {"suite": suite}
        if device_id:
            payload["device_id"] = device_id
        
        response = requests.post(
            f"{self.base_url}/test/run",
            json=payload
        )
        return response.json()

# Usage
client = KMobileClient()
devices = client.list_devices()
results = client.run_tests("regression", devices[0]["id"])
```

### Custom Tool Integration

#### Webpack Plugin
Integrate KMobile with your build process:

```javascript
class KMobileWebpackPlugin {
  constructor(options = {}) {
    this.options = options;
    this.mcpUrl = options.mcpUrl || 'http://localhost:8931';
  }
  
  apply(compiler) {
    compiler.hooks.afterEmit.tapAsync('KMobileWebpackPlugin', (compilation, callback) => {
      // Deploy to devices after build
      fetch(`${this.mcpUrl}/tools/app_install`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          app_path: this.options.outputPath,
          devices: this.options.devices || 'all'
        })
      }).then(() => callback()).catch(callback);
    });
  }
}

module.exports = KMobileWebpackPlugin;
```

### Testing Framework Integration

#### Jest Integration
Custom Jest environment for mobile testing:

```javascript
// jest-environment-kmobile.js
const { TestEnvironment } = require('jest-environment-node');
const fetch = require('node-fetch');

class KMobileEnvironment extends TestEnvironment {
  constructor(config, context) {
    super(config, context);
    this.mcpUrl = config.projectConfig.testEnvironmentOptions?.mcpUrl || 'http://localhost:8931';
  }
  
  async setup() {
    await super.setup();
    
    // Setup mobile test environment
    const response = await fetch(`${this.mcpUrl}/tools/test_setup`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        suite: 'jest',
        cleanup: true
      })
    });
    
    this.global.kmobile = await response.json();
  }
  
  async teardown() {
    // Cleanup mobile test environment
    await fetch(`${this.mcpUrl}/tools/test_cleanup`, {
      method: 'POST'
    });
    
    await super.teardown();
  }
}

module.exports = KMobileEnvironment;
```

These integration examples provide comprehensive coverage for using KMobile MCP across different development environments and workflows.

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