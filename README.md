# 🔄 hosts-hook [[Korean](./docs/README.ko.md)]

**hosts-hook** is a tool that allows you to override DNS resolution for specific hostnames without modifying your system's `hosts` file. This is particularly useful in development and testing environments where you need to redirect traffic to local or different servers.

## ✨ Features

- **Dynamic DNS Overriding**: Intercepts system DNS lookups and provides custom responses.
- **Flexible Configuration**: Searches for custom hosts files (`.hosts` or `hosts`) in the current directory and parent directories.
- **Environment-Specific Hosts**: Use different hosts files for different environments (e.g., development, staging) using the `HOSTS_ENV` variable.
- **IPv4 & IPv6 Support**: Works with both IPv4 and IPv6 addresses.
- **Easy to Use**: Simple commands to initialize configuration and activate the hook.

## 💻 Supported Platforms

- **macOS**
- **Linux**

## 📥 Installation

### 🛠️ Manual Build (Current Method)

Clone the repository and build the project using Cargo:

```bash
# Clone the repository
git clone https://github.com/powerumc/hosts-hook.git
cd hosts-hook

# Build the project
cargo build
```

### macOS (Homebrew)

```bash
brew tap powerumc/tap

brew install hostshook
```

## 📚 Usage

`hosts-hook` has two main parts: the CLI tool for managing hosts files and the hooking mechanism to activate DNS overriding.

### 1. Initialize Your Hosts File

Use the `hostshook init` command to create a `.hosts` file in your current directory. This file will contain your custom DNS rules.

```bash
# Create a default .hosts file
hostshook init

# Create a named hosts file for a specific environment
hostshook init --file .hosts.development
```

### 2. Activate the DNS Hook

To activate the DNS overriding, you need to load the `hosts-hook` dynamic library into your shell session. The `hostshook` command helps you do this easily.

```bash
# This command prints the necessary export statement
# The `source` command then executes it in the current shell
source <(hostshook)

# Or

hostshook
# It will output something like:
# export DYLD_INSERT_LIBRARIES=/path/to/your/libhostshook.dylib
```
Once activated, any new processes launched from this shell will have their DNS lookups intercepted by `hosts-hook`.

### 3. Set Environment for Different Hosts Files

You can switch between different hosts configurations by setting the `HOSTS_ENV` environment variable.

```bash
# Set the environment to 'development'
export HOSTS_ENV=development

# Now, hosts-hook will look for .hosts.development or hosts.development
```

## ⚙️ How It Works

**hosts-hook** works by intercepting system calls related to DNS resolution (like `getaddrinfo`). When an application tries to resolve a domain name, the hook checks for a matching entry in your custom `.hosts` file.

If a match is found, the hook returns the IP address you specified, bypassing the standard DNS resolution process. If no match is found, the request is passed on to the system's default DNS resolver.

The search for the hosts file starts in the current directory and moves up to the root directory.

## 🗂️ Hosts File Search Order

`hosts-hook` searches for hosts files in the following order of priority:

1.  `hosts.<environment>` (e.g., `hosts.development` if `HOSTS_ENV=development`)
2.  `.hosts.<environment>` (e.g., `.hosts.development`)
3.  `hosts`
4.  `.hosts`

The search happens first in the current directory, then recursively up to the root directory until a file is found.

## 📄 Hosts File Format

The file uses the standard hosts format. Lines starting with `#` are comments.

```
# This is a comment
127.0.0.1 example.com
::1       ipv6.example.com
```

## 🧪 Examples

You can find **C** and **Node.js** examples in the `examples` directory of the repository.

### Running the Node.js Example

```bash
cd examples/nodejs
./test.sh
```

## ⚠️ Limitations

### macOS SIP (System Integrity Protection)

On macOS, System Integrity Protection (SIP) prevents system-level commands and applications (like `ping`, `curl`, `ssh`) from being hooked. This is a security feature of the OS to protect core system components.

Therefore, these protected commands will ignore `hosts-hook` and use the standard system DNS resolver.

However, **most third-party applications are not affected by SIP**. This includes:
- Development tools (Node.js, Python, Java applications)
- Database clients
- And many more.

These applications will correctly use the DNS overrides provided by `hosts-hook`.

## 📜 License

This project is distributed under the **MIT License**.

## 👥 Contributing

Issues and pull requests are welcome! 🙏
