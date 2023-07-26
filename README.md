# Rust AutoAuth CLI

![Rust](https://img.shields.io/badge/Rust-1.55.0-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

This is a simple command-line application written in Rust for user authentication using HTTP requests and an INI configuration file.

## Features

- User authentication via HTTP POST request to a specified URL.
- Configurable username and password storage in an INI file.
- Basic user input using the `dialoguer` crate.

## Dependencies

- [minreq](https://crates.io/crates/minreq): A minimalistic Rust HTTP client.
- [ini](https://crates.io/crates/ini): A Rust library to read and write INI configuration files.
- [dialoguer](https://crates.io/crates/dialoguer): A Rust library for interactive user input on the command line.

## Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

1. Clone this repository:

```
git clone https://github.com/kaustavsahoo/autoauth-rs.git
```

2. Change into the project directory:

```
cd autoauth-rs
```

3. Build the project:

```
cargo build --release
```

## Usage

Run the application from the command line:

```
cargo run --release
```

The application will prompt you for your username and password. If you've already provided your credentials before, they will be read from the configuration file (`config.ini`) and used for authentication. If not, the application will ask you to input your username and password and store them in the configuration file for future use.

## Configuration

The application stores your username and password in an INI file named `config.ini` in the same directory as the executable. If you want to manually edit or reset your credentials, you can modify this file directly.

### Configuration File Format

```
[user]
username = "your_username"
password = "your_password"
```

**Note:** It's essential to keep your configuration file secure, as it contains sensitive information.

## Contributing

Contributions are welcome! If you find a bug or want to add a new feature, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.