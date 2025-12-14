# Contributing to npm-guard

First off, thank you for considering contributing to `npm-guard`. This tool protects developers from supply chain attacks, so your code helps make the ecosystem safer for everyone.

## 🎯 Current Priorities

We are currently looking for contributions in these specific areas:
1.  **Windows Support:** Our installer (`install.sh`) is Bash-only. We need a `install.ps1` and a strategy for PowerShell Profiles.
2.  **Performance:** Parallelizing HTTP requests for `npm install pkg1 pkg2 pkg3`.

## 🛠️ Development Setup

1.  **Prerequisites:** Ensure you have Rust installed (`cargo`).
2.  **Fork & Clone:**
    ```bash
    git clone https://github.com/pchavali09/npm-guard.git
    cd npm-guard
    ```
3.  **Build:**
    ```bash
    cargo build
    ```
4.  **Test:**
    We don't have a massive test suite yet, but you can run the binary manually:
    ```bash
    cargo run -- install react
    ```

## 📐 Coding Standards

This is a security tool. Code quality is non-negotiable.

* **No Unwraps:** Avoid `unwrap()` in production code. Use `match` or `?` to handle errors gracefully. If the tool crashes, it breaks the user's terminal.
* **Privacy First:** Never add code that sends data to a third-party server (other than the official NPM registry).
* **Format:** Run `cargo fmt` before committing.
* **Lint:** Run `cargo clippy` and fix any warnings.

## 🚀 Submitting a Pull Request

1.  Create a new branch: `git checkout -b feature/my-new-feature`
2.  Commit your changes: `git commit -m 'Add some feature'`
3.  Push to the branch: `git push origin feature/my-new-feature`
4.  Open a Pull Request.

### 🪟 Windows Contributors

If you are working on Windows support, please verify:
1.  The `npm` shim works in PowerShell.
2.  The `npm` shim works in Command Prompt (cmd.exe).
3.  The uninstaller cleanly removes the changes.

Thank you for building with us!