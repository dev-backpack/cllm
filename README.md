# cllm
[![Discord](https://dcbadge.vercel.app/api/server/sy9BhhdbJu?style=flat&compact=true)](https://discord.gg/sy9BhhdbJu)
[![GitHub Release](https://img.shields.io/github/v/release/dev-backpack/cllm)](https://github.com/dev-backpack/cllm/releases)
[![CLLM Docs](https://img.shields.io/badge/Docs-CLLM-blue)](https://dev-backpack.github.io/cllm/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![GitHub Issues](https://img.shields.io/github/issues/dev-backpack/cllm)](https://github.com/dev-backpack/cllm/issues)

Empower your CLI experience with a command search tool driven by LLM magic!

```bash
$ cllm search "Show all pods in k8s cluster"
  kubectl get pods -A

$ cllm search "Find all files recursively within the current directory that contain 'a' in their filenames."
  find . -type f -name '*a*' -print

$ cllm search "Provide the command to build and push a Docker image from the current directory."
  docker build -t myapp:latest . --push
```

## Installation

### Homebrew (macOS and Linux)

```bash
brew tap dev-backpack/cllm
brew install cllm
```

### Install Script (macOS and Linux)

```bash
curl https://raw.githubusercontent.com/dev-backpack/cllm/main/install.sh | sh
```

## Commands

#### Register OpenAI API Key

```bash
cllm set key [API_KEY]
```

#### Search a command using LLM

```bash
cllm search [QUERY]
```

#### Show the history of commands searched

```bash
cllm history
```

#### Show the description of a command

```bash
cllm describe [COMMAND]
```


## License
```cllm``` is licensed under the [MIT License](https://opensource.org/licenses/MIT).

## Connect with us
If you have any questions or suggestions, feel free to open an issue or join our [Discord server](https://discord.gg/sy9BhhdbJu).