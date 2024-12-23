---
layout: doc
sidebar: true
---

# Get Lead Lang

Lead lang is distributed using **leadman**. The following command can install leadman on **Windows** and **most UNIX-like** systems. [Read more about supported operating systems](#supported-os-with-architectures)

## Install

:::tabs
== Windows (Powershell)

```sh
irm https://ahq-softwares.github.io/lead/install.ps1 | iex
```

== Linux / macOS / FreeBSD

```sh
curl -fsSL https://ahq-softwares.github.io/lead/install.sh | bash
```

:::

## Supported OS with Architectures

| OS      | Architecture   | Supported | Notes                                |
| ------- | :------------- | :-------: | :----------------------------------- |
| Windows | x64            |    ✅     | Windows 10 or above                  |
|         | arm64          |    ✅     | Windows 11                           |
|         | i686 (32-bit)  |    ✅     | Windows 10 32 Bit                    |
| macOS   | x64            |    ✅     |                                      |
|         | arm64          |    ✅     |                                      |
| Linux⭐ | x64            |    ✅     | Ubuntu 20.04 or above and equivalent |
|         | i686 (32-bit)  |    🟨     |                                      |
|         | arm64          |    🟨     |                                      |
|         | armv7 (32-bit) |    🟨     |                                      |
| FreeBSD | x64            |    🟨     |                                      |
|         | i686 (32-bit)  |    🟨     |                                      |
|         | arm64          |    🟨     |                                      |
|         | armv7 (32-bit) |    ❌     |                                      |
| NetBSD  | x64            |    🟨     |                                      |
|         | i686 (32-bit)  |    ❌     |                                      |
|         | arm64          |    ❌     |                                      |
|         | armv7 (32-bit) |    ❌     |                                      |

✅: Fully Supported

🟨: Lead Docs not supported

❌: Not Supported, Not Planned either

⭐: See Below

::: details **Note for linux users**
You must have the following installed for **lead docs** desktop application

| Package       | Version      |
| ------------- | ------------ |
| webkitgtk-4.1 | 2.20 to 2.36 |

You might follow [this](https://tauri.app/start/prerequisites/#linux)

:::
