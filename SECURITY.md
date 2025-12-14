
Reporting a Vulnerability

Do not open a public GitHub Issue for security vulnerabilities.

If you have discovered a vulnerability in npm-guard (e.g., a way to bypass the check, a remote code execution flaw, or a privacy leak), please report it via one of the following methods:

### Option 1: Email (Direct)

Please send a detailed report to: **npavanchavali@gmail.com**

### Option 2: GitHub Private Reporting

1.  Go to the [Security tab](https://github.com/pchavali09/npm-guard/security) of this repository.
2.  Click **"Report a vulnerability"** to open a private advisory.
3.  We will receive a notification and respond within 48 hours.

## Response Policy

1.  **Acknowledgment:** We will acknowledge your report within 48 hours.
2.  **Investigation:** We will investigate the issue and determine its severity.
3.  **Fix:** If confirmed, we will release a fix as soon as possible.
4.  **Disclosure:** We will publish a security advisory *after* the patch has been released and users have had time to update.

## Scope of Security

### In Scope

-   The `npm-guard` CLI binary.
-   The installation and uninstallation scripts (`install.sh`, `uninstall.sh`).
-   Data privacy (ensuring no data is sent to servers other than `registry.npmjs.org`).

### Out of Scope

-   Vulnerabilities in the official NPM Registry API itself.
-   Vulnerabilities in the user's shell configuration (Bash/Zsh) unrelated to our shim.
-   Malware residing in packages that `npm-guard` has correctly flagged as "Safe" (e.g., an account takeover of a 10-year-old package).