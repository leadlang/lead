---
layout: doc
sidebar: true
---

# Get Lead Lang

Lead lang is distributed by **leadman**. This article shares the script to install leadman.

## Install

:::tabs
== Linux / macOS / FreeBSD

```sh
curl -fsSL https://ahq-softwares.github.io/lead/install.sh | bash
```

== Windows (Powershell)

```sh
irm https://ahq-softwares.github.io/lead/install.ps1 | iex
```

:::

## Supported OS with Architectures

| OS      | Architecture         | Supported | Notes                                |
| ------- | :------------------- | :-------: | :----------------------------------- |
| Windows | x64                  |    ✅     | Windows 10 or above                  |
|         | arm64                |    ✅     | Windows 11                           |
|         | i686 (32-bit)        |    ✅     | Windows 10                           |
| macOS   | x64                  |    ✅     | Ubuntu 20.04 or above and equivalent |
|         | arm64                |    ✅     |                                      |
| Linux⭐ | x64                  |    ✅     |                                      |
|         | i686, armv7 (32-bit) |    ❌     |                                      |
|         | arm64                |    🟨     | CI failed with lead_docs             |
| FreeBSD | x64                  |    🟨     | Cannot be built with lead docs       |
|         | i686, armv7 (32-bit) |    ❌     |                                      |
|         | arm64                |    ❌     |                                      |
| NetBSD  | x64                  |    ❌     | Verified to not work                 |
|         | i686, armv7 (32-bit) |    ❌     |                                      |
|         | arm64                |    ❌     |                                      |

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
