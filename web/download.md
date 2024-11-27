---
layout: doc
sidebar: true
---

# Get Lead Lang

Lead lang is distributed by **leadman**. This article shares the script to install leadman.

## Supported OS with Architectures

| OS      |  Architecture  | Supported | Notes                           |
| ------- | :------------: | :-------: | :------------------------------ |
| Windows |      x64       |    ✅     |                                 |
|         |     arm64      |    ✅     |                                 |
|         | i686 (32-bit)  |    ❌     | 32 bit is not longer widespread |
| macOS   |      x64       |    ✅     |                                 |
|         |     arm64      |    ✅     |                                 |
| Linux⭐ |      x64       |    ✅     |                                 |
|         | i686 (32-bit)  |    ❌     | 32 bit is not longer widespread |
|         |     arm64      |    🟨     | CI failed with lead_docs        |
|         | armv7 (32-bit) |    ❌     | 32 bit is not longer widespread |
| FreeBSD |      x64       |    🟨     | Cannot be built with lead docs  |
|         | i686 (32-bit)  |    ❌     | 32 bit is not longer widespread |
|         |     arm64      |    ❌     |                                 |
|         | armv7 (32-bit) |    ❌     | 32 bit is not longer widespread |
| NetBSD  |      x64       |    ❌     | Verified to not work            |
|         | i686 (32-bit)  |    ❌     | 32 bit is not longer widespread |
|         |     arm64      |    ❌     |                                 |
|         | armv7 (32-bit) |    ❌     | 32 bit is not longer widespread |

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

## Install

:::tabs
== Linux / macOS

```sh
curl -fsSL https://ahq-softwares.github.io/lead/install.sh | bash
```

== Windows (Powershell)

```sh
irm https://ahq-softwares.github.io/lead/install.ps1 | iex
```

:::
