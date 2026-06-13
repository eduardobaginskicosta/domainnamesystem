[devns]: https://github.com/eduardobaginskicosta/devns

[tokio_repo]: https://github.com/tokio-rs/tokio
[server_tom]: ./server.toml
[citation]: https://docs.github.com/github/creating-cloning-and-archiving-repositories/creating-a-repository-on-github/about-citation-files

[social_insta]: https://www.instagram.com/eduardobaginskicosta/
[social_yt]: https://www.youtube.com/@baginskistudio
[social_in]: https://www.linkedin.com/in/eduardobaginskicosta/

# Domain Name System

> [!WARNING]
>
> This repository has been archived and is preserved primarily for educational,
> historical, and reference purposes.
>
> Development of this project has been discontinued in favor of
> [**DevNS (`eduardobaginskicosta/devns`)**][devns], a complete rewrite that
> addresses several architectural limitations, protocol compliance issues,
> packet parsing inconsistencies, and stability problems identified in
> this implementation.
>
> While this repository remains useful for studying DNS concepts and the
> evolution of the codebase, users seeking an actively maintained and reliable
> DNS server should use [**DevNS**][devns] instead.
>
> [DevNS][devns] provides improved RFC 1034 and RFC 1035 compliance, enhanced
> packet validation, better interoperability with modern DNS clients and
> resolvers, improved scalability, and a significantly more robust architecture.
>
> New deployments, testing environments, and future development efforts should
> therefore target the [DevNS][devns] repository rather than this
> archived project.

This project implements a **DNS server in Rust** that partially follows the
specifications outlined in **RFC 1034** and **RFC 1035**. It was originally
inspired by the repository [**dnsguide**](https://github.com/EmilHernvall/dnsguide)
by [**EmilHernvall**](https://github.com/EmilHernvall), which provides a
step-by-step guide for building a DNS server from scratch in Rust.

Compared to the original reference implementation, this version introduces a
more modular structure, improved separation of concerns, and additional runtime
capabilities such as asynchronous processing and dynamic scalability.

The server is capable of resolving both **IPv4** and **IPv6** addresses and can
operate either as a forwarding resolver using upstream DNS providers or as a
locally managed resolver using custom domain configurations.

> [!IMPORTANT]
>
> This project is intended for learning, experimentation, and small-scale
> or internal usage scenarios. While it includes performance and scalability
> improvements, it is not designed to be a production-ready DNS server.

<!-- = = = -->

## ✨ Features

### Modular Codebase
The original single-file implementation has been refactored into a modular
architecture. This improves readability, maintainability, and long-term
extensibility.

### Dynamic Scalability
The server supports dynamic worker scaling, allowing the number of worker
threads to be adjusted based on system load and configuration.

### Asynchronous Processing
Using [`tokio`][tokio_repo], the server handles DNS queries concurrently without
blocking execution, improving throughput and responsiveness under load.

### Flexible Configuration
All major server settings can be controlled via the [`server.toml`][server_tom]
configuration file, including:

- Nameservers
- Worker limits
- Message queue size
- Debugging options

### IPv4 and IPv6 Support
The server resolves both **A (IPv4)** and **AAAA (IPv6)** DNS records.

### RFC-Oriented Implementation
Core DNS behavior follows the principles defined in **RFC 1034** and **RFC 1035**,
providing a practical implementation of the DNS protocol.

### Improved Debugging and Error Handling
Enhanced error handling and optional debug output help simplify troubleshooting
and development.

<!-- = = = -->

## 🤝 Support the Project and Follow My Work

If DevNS has ben usedul to you, your team, or your organization, please consider
supporting its continued development.

You can also follow my work through the following platforms:
- [**Instagram**][social_insta] -- Personal updates, behind-the-scenes content, and
  ongoing projects.
- [**YouTube**][social_yt] -- Development-related content, technical projects, and
  demonstrations.
- [**LinkedIn**][social_in] -- Professional updates, technical articles, and
  industry-related discussions.

Your support, feedback, and engagement help keep projeccts like DevNS actively
maintained and continuously envolving.

<!-- = = = -->

## 👨‍🏫 Citation

If you use DevNS in academic work, research projects, technical reports, or
publications, please consider citing the project appropriately.

For additional information regarding citation files on GitHub, see:
[About CITATION Files][citation].

### APA Format
```APA
Baginski Costa, E. (2005). Domain Name System (Version 0.1.0) [Computer software]. https://github.com/eduardobaginskicosta/domainnamesystem
```

### BibTeX Format
```BibTeX
@software{Baginski_Costa_Domain_Name_System_2005,
author = {Baginski Costa, Eduardo},
month = oct,
title = {{Domain Name System}},
url = {https://github.com/eduardobaginskicosta/domainnamesystem},
version = {0.1.0},
year = {2005}
}
```
