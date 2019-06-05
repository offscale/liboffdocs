liboffdocs
==========
[![Build Status](https://travis-ci.org/offscale/liboffdocs.svg?branch=master)](https://travis-ci.org/offscale/liboffdocs)

Liboffdocs is the library for the offdocs CLI.

## Problem
Docs are distributed, in wiki(s), codebase(s), blog(s) and more. These cross languages, programming and natural.

## Solution
Create a static documentation generator, that works cleanly with Angular.

Bring in documentation from multiple repositories, including RFCs, wikis, and code.

Use a simple YAML file to control ETL.

### Config file
```yaml
base_angular_repo: '.' # this repo
code:
- url: https://github.com/offscale/golang-proj.git
  type: {$ref: "#/_types/sphinx"} # shown as an example of a custom type overriding
  target: '/docs/go'
- url: https://github.com/offscale/rust-proj.git
  target: '/docs/rust'
- url: https://github.com/offscale/java-proj.git
  target: '/docs/java'
blog:
- # TBD
rfc:
- url: https://github.com/offscale/rfcs.git
  target: '/rfcs'
wiki:
- url: https://github.com/offscale/some-proj.wiki.git
  target: '/wiki0'
- url: https://github.com/offscale/some-proj.wiki.git
  target: '/wiki1'
```


#### Implementation details
These are all implementation details, and can be completely rethought. Consider using [liboffsetup](https://github.com/offscale/liboffsetup) here.
```yaml
# Consider making these a default list than can be extended or overridden
_types:
  code:
    dependencies: {$ref: "#/_dependencies/sphinx"}
    after: python -m comment_extraction antlr.py
  rfc:
    dependencies: {$ref: "#/_dependencies/rfc"}
    after:
      - ng-git-gen -p "$PWD" -l -b 'make html_body' -e '.html' -i "import { NgxPageScrollModule } from 'ngx-page-scroll';" -f '.replace(/href="#/g, `pageScroll href="#`)' -r rfc -s './xml2rfc.css' -vv
      - ng-md-components -d "$PWD"
      - cp /usr/local/lib/python3.*/dist-packages/xml2rfc/data/{xml2rfc.js,xml2rfc.css} src/app/rfc/generated/
  wiki:
    description: Github wiki
    dependencies: {$ref: "#/_dependencies/wiki"}
    after:
      - ng-git-gen -p "$PWD" -l -o '.md' -vv

# Consider making these a default list than can be extended or overridden
# Consider switching this syntax to that of `liboffsetup`, and using its internal functions
_dependencies:
  rfc:
    - os: linux
      arch: amd64
      distribution: Ubuntu
      version: '>16.04'
      apt:
        - python3-lxml
        - python3-setuptools
        - python3-pip
        - python3-html5lib
        - html-xml-utils
        - curl
      bash:
        - curl -L https://git.io/n-install | bash -s -- -y lts
        - npm i -g npm
        - npm i -g typescript tslint @angular/cli angular-cli-ghpages ng-git-gen ng-md-components
        - curl -L https://github.com/mmarkdown/mmark/releases/download/v2.0.46/mmark_2.0.46_linux_amd64.tgz -o mmark.tgz
        - tar xf mmark.tgz
        - sudo ln -s "$PWD/mmark" /usr/local/bin/mmark
        - sudo -H pip3 install xml2rfc
        - sudo -H pip3 install -U six
  sphinx:
    - os: linux
      arch: amd64
      distribution: Ubuntu
      version: '>16.04'
      env:
      - COMMENT_EXTRACTION=comment-extraction-master
      - EXTRACTED_DIR=docs/extracted-comments
      - INPUT_DIR=src
      apt:
        - python3-dev
        - python3-lxml
      bash:
        - pip install https://github.com/dev10/comment-extraction/archive/master.tar.gz#egg=comment-extraction
  wiki:
    - os: linux
      arch: amd64
      distribution: Ubuntu
      version: '>16.04'
      apt: {$ref: "#/_dependencies/rfc/apt"}
      bash: {$ref: "#/_dependencies/rfc/bash"}  
```

## RFCs

Of interest is its RFC: https://github.com/offscale/offscale-rfcs/blob/master/%5Boffdocs%5D%20Concepts.md

## Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### Step-by-step guide

```bash
# Install Rust (nightly)
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Clone this repo
$ git clone https://github.com/offscale/liboffdocs && cd liboffdocs
# Run tests
$ cargo test
# Format, build and test
$ cargo make
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
