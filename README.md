<div align="center">

## Teenygrad-thesis
#### A thesis project to explore the utility and usability of the Rust programming language in a machine learning framework.

[![Rust](https://img.shields.io/badge/Rust-f74c00.svg?style=for-the-badge&logoColor=white&logo=rust)]()
[![Teenygrad](https://img.shields.io/badge/Teenygrad-FFFFFF.svg?style=for-the-badge&logoColor=black&logo=tinygrad)](https://github.com/tinygrad/teenygrad)

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/teeny-kth/teenygrad/test.yml?branch=main&style=for-the-badge&logo=github&logoColor=white&label=Test&labelColor=black)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/teeny-kth/teenygrad/lines.yml?branch=main&style=for-the-badge&logo=github&logoColor=white&label=Line%20Counter&labelColor=black)

</div>

## ⇁  Welcome
This is a thesis project by [Alexander Berg](https://github.com/21st-centuryman) and [Jabez Otieno](https://github.com/Jakunot) to rewrite the [Teenygrad](https://github.com/tinygrad/teenygrad) framework by Tiny corp to Rust. Teenygrad is a below 1000 lines implementation of [Tinygrad](https://github.com/tinygrad/tinygrad). The purpose of our thesis is to discuss the properties of the Rust programming language and if they are suitable or necessary for large language models and at which stages they might or might not be applicable. This thesis is planned to be presented in late fall of 2024, until then we plan to work on this whenever school and life allow us. We plan to structure the development of this project similarly to how we have done it professionally in the past, so feel free to check for issues, pull requests and the like. In the meantime below is some information on the stages and structure for this project. If you want current TODO check the current issues.

## ⇁  Repository stages
We have 3 stages to this project.
1. Personal research and practice. 
   * Teenygrad is sub 1000 lines and we want to at least stay within the spirit of that. Therefore we decided that we will focus on tasks involving rust to get better at using it functionally as well as focusing on finishing our current school work. 
   * Now - June
2. Implementation.
   * This is the fun part, basically Implementing the project. Making sure it finishes and is within a reasonable line count.
   * June - July
3. Thesis writing and research
   * Writing the thesis, doing research on the project and presenting our findings.
   * August - September


## ⇁  Repository structure
The project will be structured as shown:
```
├── extra
│   └── datasets
│       └── mnist/ <- Collection of mnist data
├── src
│   ├── lib.rs
│   ├── example
│   │   └── mnist.rs <- mnist implementation
│   ├── tests
│   │   └── sz.py <- Line counter and tokenizer
│   └── teenygrad
│       ├── function.rs
│       ├── helpers.rs
│       ├── lazy.rs
│       ├── ops.rs
│       ├── realize.rs
│       ├── tensor.rs
│       ├── nn
│       │   └── optim.rs
│       └── shape
│           └── symbolic.rs
└── README.md
```
