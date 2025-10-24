# DNS Server in Rust
This project implements a **DNS server in Rust** that partially follows the
specifications outlined in **RFC 1034** and **RFC 1035**. It was inspired by
the repository [**dnsguide**](https://github.com/EmilHernvall/dnsguide) by
[**EmilHernvall**](https://github.com/EmilHernvall), which provides a
step-by-step guide for building DNS server from scratch in Rust. However,
this version improves upon the original by breaking the code into multiple
modules, refactoring key sections, and introducing additional features such
as **dynamic scalability** and **asynchronous processing**.

<!-- * >>> * -->

## ✨ Features ✨
- **Modular Code:** The original single-file implemention has been refactored
  into a more maintainable modular structure.
- **Dynamic Scalability:** The server supports dynamic thread scaling, allowing
  you to adjust the number of worker threads based on demand.
- **Asynchronous Processing:** Using [`tokio`](https://github.com/tokio-rs/tokio),
  the server can handle DNS queries concurrently without blocking, improving
  performance and scalability.
- **Flexible Configuration:** The server’s settings (e.g., nameservers, message
  queue size, worker count) can be easily configured via the
  [`server.toml`](./server.toml) configuration file.
- **Partial RFC 1034 & 1035 Support:** Implements DNS resolution features as per
  **RFC 1034** and **RFC 1035**, allowing you to query and resolve both **IPv4**
  and **IPv6** addresses.
- **Improved Error Handling & Debugging:** Enhanced error handling and optional
  debugging output to help you troubleshoot issues.

<!-- * >>> * -->

## 📁 Project Structure 📁
This project is organized into a modular structure with multiples **folders** and **crates**
for better maintainability and flexibility.

- **Root Folder:**
  Contains the main configuration files, including the [`server.toml`](./server.toml)
  configuration file.

- **[`dns_core (crates/core)`](./crates/core):**
  The main library crate, internally named **`dns_core`**. This crate contains the
  **core logic** for DNS resolution, handling requests, processing queries, etc.

- **[`dns (crates/bin)`](./crates/bin):**
  The binary crate, internally named **`dns`**. This crate contains the **entry point**
  for the server, which sets up the server and run the DNS service.

### Directory Structure
```plain
domainnamesystem/  # Root folder
├── crates/        # Crates folder
│   ├── bin/       # Server binary (dns)
│   └── core/      # Core logic for DNS resolution (dns_core)
├── Cargo.toml     # Cargo configuration
└── server.toml    # Configuration file
```

<!-- * >>> * -->

## 📦 Compile and Run 📦
To get started with this DNS server, follow these steps:

### 1. **Clone the Repository:**
```bash
git clone https://github.com/eduardobaginskicosta/domainnamesystem
cd domainnamesystem
```

### 2. Configure the Server
Edit the [`server.toml`](./server.toml) file to configure the DNS server. You can
which **nameservers** to use, the maximum number of **worker threads**, and
other settings.

Example `server.toml`:
```toml
[server]
nameservers = ["1.1.1.1", "1.0.0.1"]
max_messages = 20
max_workers = 10
debug = false

[domains]
[[domains.single]]
name = "host.local"
ipv4 = ["127.0.0.1"]
ipv6 = ["::1"]

[[domains.multiple]]
name = ["example.com", "example.local"]
ipv4 = ["0.0.0.0"] # blocking feature (read comments)
ipv6 = ["::0"] # blocking feature (read comments)
```

### 3. **Build and Run the Project**
The project is designed to be flexible with **two main versions** of the server:
the **`experimental` (new)** implementation and the **`legacy` (older)** version.
By default, the server will run with the **`experimental`** version.

To run **`experimental` (new; recommended)** implementation:
```bash
cargo run --release
```
To run the **`legacy` (old)** implementation:
```bash
cargo run --release --features legacy
```

### 4. Test The Server
After starting the server, you can test it using tools like `nslookup` or `dig`:
(replace `DNS_IP` with your machine's actual address. Ex: `192.168.1.10`)
```bash
nslookup host.local DNS_IP # for windows
dig @DNS_IP host.local A # for macos and linux
```
<!-- * >>> * -->

## ⚙️ Configuration ⚙️
The server is configurated via the [`server.toml`](./server.toml) file. All necessary
configuration options are defined within the file itself in the form of comments, which
include **descriptions**, **examples** and **tips** to help you understand and just the
settings according to your needs.

### Server Configuration (`[server]`)
```toml
# REQUIRED. Used for server configurations.
[server]

# Description: Define the lookup servers. These are the DNS servers your server will
#              use to resolve queries.
# Example: nameservers = ["1.1.1.1", "0.0.0.0"]
nameservers = ["1.1.1.1", "1.0.0.1"]

# Description: Maximum number of messages that can be queued for processing.
# Example: max_messages = 100
max_messages = 20

# Description: Maximum number of worker threads (parallel processing) that can
#              handle requests.
# Example: max_workers = 20
max_workers = 10

# Description: Enable or disable debug messages. Useful for troubleshooting.
#              | Displating characters in the console can directly affect server |
#              | performance as resources are diverted to displaying characters. |
#              | For better performance in production, leave it disabled.        |
# Example: debug = false
debug = false
```

### Domains Configuration
```toml
# REQUIRED. Used for domain configurations.
# Mus be set even if this feature is not used.
[domains]
```

#### ➡️ Single Domain Configuration (`[[domains.single]]`)
Multiple `[[domains.single]]` sections can be defined in the same file to specific
IPs for specific domain. Simply replicate the code and modify the values.
```toml
# OPTIONAL. Used to configure a single domain.
# ( There can be multiple sections )
[[domains.single]]

# Description: Defines a single domain to resolve.
# Example: name = "mycomputer"
name = "host.local"

# Description: List of IPV4 addresses that domain will resolve to.
# Example: ipv4 = ["127.0.0.1"]
#
# Tips: If the address "0.0.0.0" is set, all associated domains will be blocked due
#       the blocking feature. If the address "0.0.0.0" is set, all others in the list
#       will be ignored.
ipv4 = ["127.0.0.1"]

# Description: List of IPV6 addresses that domain will resolve to.
# Example: ipv6 = ["::1"]
#
# Tips: If the address "::0" is set, all associated domains will be blocked due the
#       blocking feature. If the address "::0" is set, all others in the list will
#       be ignored.
ipv6 = ["::1"]
```

#### ➡️ Multiple Domains Configuration (`[[domains.multiple]]`)
Multiple `[[domains.multiple]]` sections can be defined in the same file to specific
IPs for specific domains. Simply replicate the code and modify the values.
```bash
# OPTIONAL. Used to configure multiple domains.
# ( There can be multiple sections )
[[domains.multiple]]

# Description: Defines multiple domains that share the same resolution (i.e., they
#              will resolve to the seme IPs).
# Example: name = ["example.com", "example.local"]
name = ["example.com", "example.local"]

# Description: List of IPv4 addresses for the domains.
# Example: ipv4 = ["127.0.0.1"]
#
# Tips: If the address "0.0.0.0" is set, all associated domains will be blocked due
#       the blocking feature. If the address "0.0.0.0" is set, all others in the list
#       will be ignored.
ipv4 = ["0.0.0.0"]

# Description: List of IPv6 addresses for the domains.
# Example: ipv6 = ["::1"]
#
# Tips: If the address "::0" is set, all associated domains will be blocked due the
#       blocking feature. If the address "::0" is set, all others in the list will
#       be ignored.
ipv6 = ["::0"]
```

<!-- * >>> * -->

## 📊 Scalability and Performance 📊
This DNS server is build with scalability in mind. It uses [`tokio`](https://github.com/tokio-rs/tokio)
for asynchronous processing, which allows the server to handle multiples DNS queries
concurrently without blocking the main thread. The number of worker threads can be
dynamically adjusted based on demand, ensuring optimal performance.

By using a message queue system and distributing requests across multiple worker
threads heavy traffic efficiently. The queue size and worker count are configurable
via the [`server.toml`](./server.toml) file.

<!-- * >>> * -->

## 🧑‍💻 Contributing 🧑‍💻
Feel free to contribute to this project by forking the repository, submitting pull requests,
or reporting issues. If you'd like to add new features or improving the existing code, open
a issue or PR, and i'd be happy to review it!

## 📜 License 📜
This project is licensed under the MIT License - see [**LICENSE**](./LICENSE) file dor details.
