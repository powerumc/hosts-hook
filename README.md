# 🔄 hosts-hook [[Korean](./docs/README.ko.md)]

**hosts-hook** is a **tool** that allows you to override DNS resolution without modifying your system's hosts file. This tool is particularly useful in *development* and *testing environments*.

## ✨ Features

- **Hooks** into the system's DNS resolution functions to intercept DNS lookups.
- Searches for custom hosts files (`.hosts` or `hosts`) in the current directory or parent directories.
- Returns the **specified IP address** instead of performing an actual DNS lookup when a matching hostname is found in the custom hosts file.
- Supports both **IPv4** and **IPv6** addresses.
- Can use different hosts files based on environment variables (`HOSTS_ENV`), enabling various configurations (e.g., *development*, *production*).

## 💻 Supported Platforms

- **macOS**
- **Linux**

## 📥 Installation

### macOS

Installation via **Homebrew** will be supported soon:

```bash
brew tap powerumc/tap
brew install hostshook
```

### 🛠️ Manual Build

You can also clone the repository and build it yourself:

```bash
# Clone the repository
git clone https://github.com/powerumc/hosts-hook.git
cd hosts-hook

# Build
cargo build
```

## 📚 Usage

### 🖥️ Using the CLI Tool

You can manage hosts files using the `hostshook` binary installed via Homebrew:

```bash
# Initialize .hosts file
hostshook init

# Initialize .hosts file with a specific name
hostshook init --file .hosts.development
```

### 📚 Loading the Library

```bash
hostshook
# export DYLD_INSERT_LIBRARIES=/opt/homebrew/lib/libhostshook.dylib
# Or
source <(hostshook)
```

### ⚙️ Setting Environment Variables

To use different hosts files for different environments:

```bash
# Set environment variable
export HOSTS_ENV=development

# Now it will use .hosts.development or hosts.development file
```

## 🗂️ Hosts File Names

Custom hosts files are searched in the following order:
1. `hosts.<environment>` (using the `HOSTS_ENV` environment variable, e.g., `HOSTS_ENV=development`)
2. `.hosts.<environment>` (using the `HOSTS_ENV` environment variable, e.g., `HOSTS_ENV=development`)
3. `hosts`
4. `.hosts`

The search for hosts files in the current and parent directories follows this order:

1. Look for hosts files in the current directory.
2. If not found, search for hosts files up to the root directory.
3. If no custom hosts files are found, the system's default hosts information is used.

## 📄 Hosts File Format

The hosts file follows the **standard hosts file format**:

```
# Comment
127.0.0.1 example.com
127.0.0.2 example2.com
```

## 🧪 Examples

You can find **C language** and **Node.js** examples in the `examples` directory of the repository.

### 🔍 Running the C Language Example

```bash
cd examples/clang
./test.sh
```

You might see output similar to this:
```text
# Before hostshook is loaded
## Testing gethostbyname...
gethostbyname resolved: example.com -> 96.7.128.198

# Hostshook is loaded
## Testing getaddrinfo...
2025-06-11T13:25:32.299Z DEBUG [hostshook] Not found: /..../hosts-hook/examples/clang/hosts
2025-06-11T13:25:32.299Z DEBUG [hostshook] Found: /..../hosts-hook/examples/clang/.hosts
2025-06-11T13:25:32.299Z DEBUG [hostshook] Found IpAddr: 127.0.0.1
2025-06-11T13:25:32.299Z DEBUG [hostshook] Hooked getaddrinfo for: example.com -> 127.0.0.1
getaddrinfo resolved addresses:
  IPv4: 127.0.0.1
```

### 🔍 Running the Node.js Example

```bash
cd examples/nodejs
./test.sh
```

## ⚙️ How It Works

**hosts-hook** **hooks** into system DNS resolution functions like `gethostbyname`, `gethostbyname2`, and `getaddrinfo` to intercept DNS lookups.
When a hostname is found in the custom hosts file, it returns the **specified IP address** instead of performing an actual DNS lookup.
The hosts file is searched from the current directory up to the root directory until it is found.

## ⚠️ Limitations

### 🍎 macOS SIP (System Integrity Protection) Limitation

On macOS, there are some limitations due to the System Integrity Protection (SIP) security feature. System commands (`ping`, `curl`, etc.) are protected by SIP and are not affected by hosts-hook's DNS hooking.

This is by design in macOS's security architecture, as SIP prevents certain system processes and commands from being modified or hooked to protect system integrity. Therefore, these commands will always use the system's default DNS resolution path.
Regular applications work normally with hosts-hook, but please keep this limitation in mind when using system commands.

## 📜 License

This project is distributed under the **MIT License**.

## 👥 Contributing

Issues and pull requests are welcome! 🙏
