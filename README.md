# Esqlz

Esqlz is a TUI to manage sequelize migrations easily. It is an extension wrapper of [Sequelize CLI](https://www.npmjs.com/package/sequelize-cli).

# Usage

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

# Installation

For Development
```bash
git clone https://github.com/Harshil-Jani/esqlz.git
cd esqlz
cargo build --release
cargo install --path .
```

# License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

