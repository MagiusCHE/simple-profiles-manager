# Simple Profiles Manager

A lightweight desktop application for managing user profiles, built with Rust and [egui](https://github.com/emilk/egui).

## Features

- Create, edit, and delete profiles
- Select an active profile
- Multi-application support (each app has its own profile storage)
- Persistent storage (profiles saved to local config directory)
- Clean, modern dark-themed UI

## Screenshot

The application provides a simple interface to manage profiles:

- **Profile List View**: Browse and select existing profiles
- **New Profile View**: Create a new profile with a unique name
- **Edit Profile View**: Rename existing profiles

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later recommended)

### Build from source

```bash
git clone https://github.com/MagiusCHE/simple-profiles-manager.git
cd simple-profiles-manager
cargo build --release
```

The compiled binary will be located at `target/release/simple-profiles-manager`.

### Run directly

```bash
cargo run --release -- --app-id myapp
```

## Usage

The application requires an `--app-id` argument to identify which application's profiles to manage.

### Command Line Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `--app-id`, `-a` | Yes | Application ID (used for storage directory) |
| `--title`, `-t` | No | Application title (displayed in the UI, defaults to app_id) |

### Examples

```bash
# Basic usage with app ID only
simple-profiles-manager --app-id myapp

# With custom title
simple-profiles-manager --app-id com.example.myapp --title "My Application"

# Short form
simple-profiles-manager -a myapp -t "My App"
```

### Workflow

1. Launch the application with an `--app-id`
2. If no profiles exist, you'll be prompted to create one
3. Use the **New** button to create additional profiles
4. Select a profile from the list and click **Select Profile** to mark it as active
5. Use **Edit** to rename a profile or **Delete** to remove it

## Data Storage

Profiles are stored in your system's config directory, organized by app ID:

| Platform | Location |
|----------|----------|
| Linux | `~/.config/simple-profiles-manager/<app_id>/` |
| macOS | `~/Library/Application Support/simple-profiles-manager/<app_id>/` |
| Windows | `C:\Users\<User>\AppData\Roaming\simple-profiles-manager\<app_id>\` |

Files (per app):
- `profiles.json` - List of all profiles
- `selected-profile` - Name of the currently selected profile

The `app_id` is automatically sanitized to remove invalid path characters (`/`, `\`, `:`, `*`, `?`, `"`, `<`, `>`, `|`) ensuring profiles are always stored safely within the designated directory.

## Tech Stack

- **[Rust](https://www.rust-lang.org/)** - Programming language
- **[eframe/egui](https://github.com/emilk/egui)** - Immediate mode GUI framework
- **[clap](https://crates.io/crates/clap)** - Command line argument parsing
- **[serde](https://serde.rs/)** - Serialization/deserialization
- **[dirs](https://crates.io/crates/dirs)** - Cross-platform config directory detection

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
