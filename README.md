# markdown-toc

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Table-of-Contents (toc) generator, written in Rust. Inspired by [sebdah/markdown-toc](https://github.com/sebdah/markdown-toc).

## Table of Contents

1. [Table of Contents](#table-of-contents)
1. [Installation](#installation)
    1. [Run in Docker](#run-in-docker)
    1. [Install with cargo](#install-with-cargo)
    1. [Build yourself](#build-yourself)
1. [Usage](#usage)
    1. [Generating basic ToC](#generating-basic-toc)
    1. [Customizing bullets](#customizing-bullets)
    1. [Limiting the depth of headers](#limiting-the-depth-of-headers)
    1. [Excluding links](#excluding-links)
    1. [Customizing header of ToC](#customizing-header-of-toc)
        1. [Excluding header](#excluding-header)

## Installation

### ~~Run in Docker~~

Not published yet

```bash
docker run -v $PWD:/app -w /app --rm -it pbzweihander/markdown-toc README.md
```

### ~~Install with cargo~~

Not published yet

```bash
cargo install md-toc
```

### Build yourself

```bash
git clone https://github.com/pbzweihander/markdown-toc.git
cargo build --release
cargo install --path .
```

## Usage

```bash
$ md-toc -h
    Usage: md-toc FILE [options]

      FILE        The Markdown file to parse for table of contents,
                  or "-" to read from stdin

    Options:
        -h, --help          print this help message
            --bullet {str}  Custom bullet of the ToC list. (default: "1.")
            --indent {int}  Indentation of the ToC list. (default: 4)
            --max-depth {int}
                            Max depth of headers to include.
            --min-depth {int}
                            Min depth of headers to include. (default: 0)
            --header {str}  Custom header of the ToC. (default: "## Table of
                            Contents")
            --no-link       Exclude links in ToC
            --no-header     Exclude the header of ToC
```

### Generating basic ToC

```bash
md-toc README.md
```

Output:

```markdown

## Table of Contents

1. [markdown-toc](#markdown-toc)
    1. [Table of Contents](#table-of-contents)
    1. [Installation](#installation)
        1. [Run in Docker](#run-in-docker)
        1. [Install with cargo](#install-with-cargo)
        1. [Build yourself](#build-yourself)
    1. [Usage](#usage)
        1. [Generating basic ToC](#generating-basic-toc)
        1. [Customizing bullets](#customizing-bullets)
        1. [Limiting the depth of headers](#limiting-the-depth-of-headers)
        1. [Excluding links](#excluding-links)
        1. [Customizing header of ToC](#customizing-header-of-toc)
            1. [Excluding header](#excluding-header)

```

### Customizing bullets

```bash
md-toc README.md --bullet "-" --indent 2
```

Output:

```markdown

## Table of Contents

- [markdown-toc](#markdown-toc)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
    - [Run in Docker](#run-in-docker)
    - [Install with cargo](#install-with-cargo)
    - [Build yourself](#build-yourself)
  - [Usage](#usage)
    - [Generating basic ToC](#generating-basic-toc)
    - [Customizing bullets](#customizing-bullets)
    - [Limiting the depth of headers](#limiting-the-depth-of-headers)
    - [Excluding links](#excluding-links)
    - [Customizing header of ToC](#customizing-header-of-toc)
      - [Excluding header](#excluding-header)

```

### Limiting the depth of headers

```bash
md-toc README.md --min-depth 1 --max-depth 2
```

Output:

```markdown

## Table of Contents

1. [Table of Contents](#table-of-contents)
1. [Installation](#installation)
    1. [Run in Docker](#run-in-docker)
    1. [Install with cargo](#install-with-cargo)
    1. [Build yourself](#build-yourself)
1. [Usage](#usage)
    1. [Generating basic ToC](#generating-basic-toc)
    1. [Customizing bullets](#customizing-bullets)
    1. [Limiting the depth of headers](#limiting-the-depth-of-headers)
    1. [Excluding links](#excluding-links)
    1. [Customizing header of ToC](#customizing-header-of-toc)

```

### Excluding links

```bash
md-toc README.md --no-link
```

Output:

```markdown

## Table of Contents

1. markdown-toc
    1. Table of Contents
    1. Installation
        1. Run in Docker
        1. Install with cargo
        1. Build yourself
    1. Usage
        1. Generating basic ToC
        1. Customizing bullets
        1. Limiting the depth of headers
        1. Excluding links
        1. Customizing header of ToC
            1. Excluding header

```

### Customizing header of ToC

```bash
md-toc README.md --header "# ToC"
```

Output:

```markdown
# ToC

1. [markdown-toc](#markdown-toc)
    1. [Table of Contents](#table-of-contents)
    1. [Installation](#installation)
        1. [Run in Docker](#run-in-docker)
        1. [Install with cargo](#install-with-cargo)
        1. [Build yourself](#build-yourself)
    1. [Usage](#usage)
        1. [Generating basic ToC](#generating-basic-toc)
        1. [Customizing bullets](#customizing-bullets)
        1. [Limiting the depth of headers](#limiting-the-depth-of-headers)
        1. [Excluding links](#excluding-links)
        1. [Customizing header of ToC](#customizing-header-of-toc)
            1. [Excluding header](#excluding-header)


```

#### Excluding header

```bash
md-toc README.md --no-header
```

Output:

```markdown

1. [markdown-toc](#markdown-toc)
    1. [Table of Contents](#table-of-contents)
    1. [Installation](#installation)
        1. [Run in Docker](#run-in-docker)
        1. [Install with cargo](#install-with-cargo)
        1. [Build yourself](#build-yourself)
    1. [Usage](#usage)
        1. [Generating basic ToC](#generating-basic-toc)
        1. [Customizing bullets](#customizing-bullets)
        1. [Limiting the depth of headers](#limiting-the-depth-of-headers)
        1. [Excluding links](#excluding-links)
        1. [Customizing header of ToC](#customizing-header-of-toc)
            1. [Excluding header](#excluding-header)

```
