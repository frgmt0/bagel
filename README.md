# 🥯 Bagel Browser

A minimal, performant web browser built with Rust and Tauri, focusing on privacy, speed, and simplicity.

## Features

### Core Browser Functionality
- **Modern Tab Management**: Chrome-style tabs with easy creation, closing, and switching
- **Smart Address Bar**: Unified search and navigation with auto-suggestions
- **Privacy-First Search**: Integrated with 4get search engine for private browsing
- **Security Features**: HTTPS upgrade suggestions, basic ad blocking, and tracking protection

### Storage & Privacy
- **Bookmarks**: Traditional folder-based bookmark management
- **History**: Browsing history with search capabilities
- **Automatic Cookie Cleanup**: Cookies auto-cleared after 30 days
- **Data Management**: Local SQLite storage for all user data

### Built for Performance
- **Minimal Resource Usage**: Targeting under 4GB RAM usage
- **Fast Startup**: Sub-3 second startup time on modern hardware
- **Native Performance**: Built with Rust for memory safety and speed

## Architecture

The browser is built with a modular architecture using Rust and Tauri:

```
src/
├── browser/          # Core browser functionality
│   ├── webview.rs    # WebView management and navigation
│   ├── tabs.rs       # Tab management and state
│   └── security.rs   # Security features and SSL validation
├── storage/          # Data persistence
│   ├── bookmarks.rs  # Bookmark management
│   ├── history.rs    # Browsing history
│   └── cookies.rs    # Cookie management with auto-clear
├── search/           # Search integration
│   └── provider.rs   # 4get search engine integration
├── ui/               # User interface components
├── utils/            # Configuration and utilities
└── main.rs           # Application entry point
```

## Development

### Prerequisites
- Rust (latest stable)
- Node.js (for frontend dependencies, if needed)
- Platform-specific dependencies for Tauri

### Building
```bash
# Check code compilation
cargo check

# Build the application
cargo build

# Run in development mode
cargo tauri dev

# Build for production
cargo tauri build
```

### Git Workflow
This project follows the Git workflow defined in [GIT_WORKFLOW.md](GIT_WORKFLOW.md). Key points:

- Use conventional commits (feat:, fix:, docs:, etc.)
- Branch from `dev` for new features
- Submit PRs to `dev` branch
- Automatic releases triggered with `[release]` in commit messages

### Project Structure
The project uses a clean modular structure where each file handles 1-2 specific functions, making it easy to understand and maintain.

## Configuration

The browser creates its configuration and data directories automatically:
- **Config**: `~/.config/bagel-browser/config.json`
- **Data**: `~/.local/share/bagel-browser/` (Linux) or equivalent on other platforms

### Default Settings
- Search Engine: 4get (https://4get.ca)
- Cookie Auto-Clear: 30 days
- Privacy Features: Enabled (tracking protection, ad blocking)
- Theme: Light theme with Ubuntu font family

## Security & Privacy

### Privacy Features
- **Default Search**: Privacy-focused 4get search engine
- **Cookie Management**: Automatic cleanup after 30 days
- **Tracking Protection**: Basic tracking script blocking
- **Ad Blocking**: Simple ad blocking using filter lists
- **HTTPS Upgrade**: Automatic HTTP to HTTPS suggestions

### Security
- **Memory Safety**: Built with Rust for inherent memory safety
- **SSL Validation**: Certificate verification with security warnings
- **Sandboxing**: Tauri's built-in security model
- **No Telemetry**: No data collection or telemetry

## Roadmap

### Phase 1 (MVP) ✅
- [x] Basic Tauri setup with WebView
- [x] Minimal UI with toolbar and tabs
- [x] Navigation and search integration
- [x] Basic bookmark and history
- [x] Simple settings system

### Phase 2 (Enhanced)
- [ ] Security features and SSL validation
- [ ] Basic ad blocking and tracking protection
- [ ] Cookie management system
- [ ] Auto-update mechanism

### Phase 3 (Extensions)
- [ ] Userscript support
- [ ] Userstyle injection
- [ ] Extension management UI
- [ ] Performance optimizations

## Contributing

1. Fork the repository
2. Create a feature branch from `dev`
3. Follow the conventional commit format
4. Submit a PR to the `dev` branch
5. Ensure all tests pass and follow the code style

See [GIT_WORKFLOW.md](GIT_WORKFLOW.md) for detailed contribution guidelines.

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/) for the application framework
- Uses [4get](https://4get.ca/) as the default search engine
- Font: Ubuntu font family
- Inspired by the need for a minimal, privacy-focused browser