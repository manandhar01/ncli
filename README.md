# ncli

A Rust-powered CLI utility for managing and generating resources inside
a NestJS project.

`ncli` helps you keep your NestJS project structure consistent by
syncing and generating common backend components such as entities,
controllers, DTOs, services, modules, and more.

---

## ✨ Features

- Sync NestJS project resources
- Generate common NestJS building blocks
- Ensures execution only inside a valid NestJS project
- Automatically runs formatting and linting after operations
- Fast and portable (compiled Rust binary)

---

## 📦 Installation

### Install from crates.io

```bash
cargo install ncli
```

### Verify installation

```bash
ncli --help
```

---

## ⚠️ Requirements

You must run `ncli` inside the root of a valid NestJS project.

A directory is considered valid if it contains:

- `nest-cli.json`
- `src/main.ts`
- `package.json`

If not, you will see:

    ✖ Not a NestJS project root.
    Make sure you run this from your NestJS project directory.

---

## 🚀 Usage

### Sync resources

```bash
ncli sync
```

Sync a specific resource:

```bash
ncli sync entities
ncli sync jobs
ncli sync components
```

Default target: `all`

---

### Generate resources

```bash
ncli generate
```

Generate a specific resource:

```bash
ncli generate controller
ncli generate dto
ncli generate entity
ncli generate job
ncli generate listing
ncli generate module
ncli generate processor
ncli generate service
ncli generate subscriber
```

Default target: `all`

---

## 🛠 Command Reference

### `sync`

Sync existing NestJS project resources to maintain structural
consistency.

**Targets:**

- `entities`
- `jobs`
- `components`
- `all` (default)

---

### `generate`

Generate new NestJS building blocks.

**Targets:**

- `controller`
- `dto`
- `entity`
- `job`
- `listing`
- `module`
- `processor`
- `service`
- `subscriber`
- `all` (default)

---

## 🔄 Post-Execution Hooks

After every successful command, `ncli` automatically:

1.  Runs the project formatter
2.  Runs the project linter

---

## 🧱 Built With

- Rust
- clap
- anyhow
- colored

---

## 📄 License

MIT
