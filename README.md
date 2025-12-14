# 🚦 npm-guard

**The "Traffic Light" for your terminal.** Stops AI Hallucinations, Typosquatting, and Phantom Dependencies *before* they hit your disk.

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Built%20with-Rust-orange)
![Platform](https://img.shields.io/badge/Platform-MacOS%20%7C%20Linux-lightgrey)

---

## ⚠️ The Problem: "The Hallucination Attack"

You ask an LLM (ChatGPT, Claude, Copilot) for a quick solution. It suggests:
> "Just install `react-query-helper-v9`!"

You run `npm install react-query-helper-v9`.
* **Reality:** That package does not exist.
* **The Threat:** Attackers scan for these "Phantom Dependencies" and register them with malware.
* **The Result:** You accidentally install a malicious package that steals your `.env` file.

## 🛡️ The Solution

`npm-guard` acts as a **Middleware for your Shell**. It intercepts `npm install` commands and checks the package's reputation against the public registry **in real-time**.

### The Traffic Light System

| Signal | Condition | Action |
| :--- | :--- | :--- |
| 👻 **PHANTOM** | Package returns `404 Not Found` | **BLOCK.** Prevents you from being the first victim of a future squatter. |
| 🔴 **DANGER** | Package is **< 3 Days Old** | **BLOCK.** Likely a newly registered typosquat or malware. |
| 🟡 **CAUTION** | Package is **< 30 Days Old** | **WARN.** "Check the author before you trust this." |
| 🟢 **SAFE** | Package is **> 30 Days Old** | **PASS.** Silent execution. Zero friction. |

---

## 📦 Installation (MacOS / Linux)

### Step 1: Install from Source

```bash
# Clone the repo
git clone https://github.com/pchavali09/npm-guard.git
cd npm-guard

# Run the installer (Requires Rust)
chmod +x install.sh
./install.sh

# Reload your shell
source ~/.zshrc  # or ~/.bashrc
```

### Step 2: Verify Installation

Run a check on a known safe package:

```bash
npm install is-odd
# Output: 🚦 Vibe Check: Scanning registry...
#         ✅ is-odd is established (3000+ days old).
```

## 🗑️ Uninstall

We respect your machine. To remove the tool and the shell alias cleanly:

```bash
# From the source directory
./uninstall.sh
```

## 🔐 Privacy & Security Architecture

We believe security tools should not spy on you.

- **No Servers:** npm-guard is a client-side CLI. It communicates directly with the NPM registry at `https://registry.npmjs.org`.
- **No Data Collection:** We do not track your IP, package list, or project name.
- **Fail Open:** If the NPM registry is unavailable or your connection drops, npm-guard warns you but allows the installation to proceed. We will never block your workflow due to network errors.

## 🗺️ Roadmap

- [x] Phase 1: Shell Interception (Shim)
- [x] Phase 2: Phantom & Age Detection (Rust)
- [ ] Phase 3: Windows Support (PowerShell) *(Help Wanted!)*
- [ ] Phase 4: Local Caching (Speed optimization)
- [ ] Phase 5: Universal Support (pip, cargo, gem)
  - *The architecture is agnostic. We welcome contributors to build adapters for Python, Rust, and Ruby!*

## 🤝 Contributing

We need help bringing this to Windows. See [CONTRIBUTING.md](./CONTRIBUTING.md) to get started.

## 📄 License

MIT License. Build safely.