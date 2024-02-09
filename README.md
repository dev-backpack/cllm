# cllm

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

`cllm` can be installed using `brew`!

```bash
brew tap dev-backpack/cllm
brew install cllm
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