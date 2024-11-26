---
layout: doc
sidebar: true
---

# Get Lead Lang

Lead lang is distributed by **leadman**. This article shares the script to install leadman.

## Supported OS with Architectures

| OS      |  Architecture  | Supported |
| ------- | :------------: | --------: |
| Windows |      x64       |        ✅ |
|         |     arm64      |        ✅ |
|         | i686 (32-bit)  |        ❌ |
| macOS   |      x64       |        ✅ |
|         |     arm64      |        ✅ |
| Linux⭐ |      x64       |        ✅ |
|         | i686 (32-bit)  |        ❌ |
|         |     arm64      |        ⏲️ |
|         | armv7 (32-bit) |        ❌ |
| FreeBSD |      x64       |        ⏲️ |
|         | i686 (32-bit)  |        ❌ |
|         |     arm64      |        ⏲️ |
|         | armv7 (32-bit) |        ❌ |

✅: Currently Supported

🟨: Lead Docs not supported

⏲️: Not Supported, Currently being worked on

❌: Not Supported, Not Planned either

⭐: See Below
::: details **Note for linux users**
You must have the following installed for **lead docs** desktop application

| Package   | Version |
| --------- | ------- |
| webkitgtk | >= 4.1  |

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
