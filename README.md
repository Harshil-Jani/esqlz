# Esqlz

Esqlz is a TUI to manage sequelize migrations easily. It is an extension wrapper of [Sequelize CLI](https://www.npmjs.com/package/sequelize-cli).

Latest Version: `1.1.0` - [Release Notes](https://github.com/Harshil-Jani/esqlz/releases/tag/v1.1.0)

## Installation

For Linux and MacOS only. (For Windows, Do it yourself, I don't have plans to support Windows)

Run the following command in your terminal to install esqlz
```bash
curl -sSL https://raw.githubusercontent.com/Harshil-Jani/esqlz/main/install.sh | bash
```

For Development Purposes
```bash
git clone https://github.com/Harshil-Jani/esqlz.git
cd esqlz
cargo build --release
cargo install --path .
```

## Usage

Create a new migration file
```bash
esqlz generate "Add a new column to the users table"
```

Select the migration you want to apply
```bash
esqlz up <optional : file_name>
```

Select the migration you want to revert
```bash
esqlz down <optional : file_name>
```

Check the migration status
```bash
esqlz status
```

## Contributing

Contributions are welcome. Please open an issue before making a PR. I am flexible with adding new features or being pointed out to bugs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

