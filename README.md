<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
<div align="left">

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]

</div>

<a href="https://github.com/Kaweees/rustyChip8">
  <source media="(prefers-color-scheme: dark)" srcset="assets/img/rust-logo-light-mode.png">
  <img alt="Text changing depending on mode. Light: 'Rust Logo Light Mode' Dark: 'Rust Logo Dark Mode'" src="assets/img/rust-logo-dark-mode.png" align="right" width="150">
</a>

<div align="left">
  <h1><em><a href="https://github.com/Kaweees/rustyChip8">~rustyChip8</a></em></h1>
</div>

<!-- ABOUT THE PROJECT -->
An emulator and implementation of the CHIP-8 processor written in Rust

### Built With

[![Rust][Rust-shield]][rust-url]
[![WebAssembly][WebAssembly-shield]][webassembly-url]
[![NixOS][NixOS-shield]][NixOS-url]
[![GitHub Actions][github-actions-shield]][github-actions-url]

<!-- PROJECT PREVIEW -->
## Preview

<p align="center">
  <img src="assets/img/demo.png"
  width = "80%"
  alt = "Space Invaders running on the emulator."
  />
</p>

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

Before attempting to build this project, make sure you have [Nix](https://nixos.org/download.html) installed on your machine.

### Installation

To get a local copy of the project up and running on your machine, follow these simple steps:

1. Clone the project repository

   ```sh
   git clone https://github.com/Kaweees/rustyChip8.git
   cd rustyChip8
   ```

2. Install the project dependencies

   ```sh
   nix-shell --max-jobs $(nproc) # Linux / Windows (WSL)
   nix-shell --max-jobs $(sysctl -n hw.ncpu) # macOS
   ```

3. Build and run the project

   ```sh
   just run -- --rom ./rom/space-invaders.ch8
   ```

<!-- PROJECT FILE STRUCTURE -->
## Project Structure

```sh
rustyChip8/
├── .github/                       - GitHub Actions CI/CD workflows
├── src/                           - project source files
├── Cargo.toml                     - Rust package manager configuration
├── Cargo.lock                     - Rust package manager lock file
├── LICENSE                        - project license
├── README.md                      - you are here
└── justfile                       - Just runner configuration
```

## License

The source code for [Kaweees/rustyChip8](https://github.com/Kaweees/rustyChip8) is distributed under the terms of the GNU General Public License v3.0, as I firmly believe that collaborating on free and open-source software fosters innovations that mutually and equitably beneficial to both collaborators and users alike. See [`LICENSE`](./LICENSE) for details and more information.

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/Kaweees/rustyChip8.svg?style=for-the-badge
[contributors-url]: https://github.com/Kaweees/rustyChip8/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Kaweees/rustyChip8.svg?style=for-the-badge
[forks-url]: https://github.com/Kaweees/rustyChip8/network/members
[stars-shield]: https://img.shields.io/github/stars/Kaweees/rustyChip8.svg?style=for-the-badge
[stars-url]: https://github.com/Kaweees/rustyChip8/stargazers

<!-- MARKDOWN SHIELD BADGES & LINKS -->
<!-- https://github.com/Ileriayo/markdown-badges -->
[Rust-shield]: https://img.shields.io/badge/Rust-20232A?style=for-the-badge&logo=Rust&logoColor=CE422B&labelColor=222222&color=CE422B
[Rust-url]: https://rust-lang.org/
[WebAssembly-shield]: https://img.shields.io/badge/WebAssembly-20232A?style=for-the-badge&logo=WebAssembly&logoColor=654FF0&labelColor=222222&color=654FF0
[webassembly-url]: https://webassembly.org/
[NixOS-shield]: https://img.shields.io/badge/NIX-%23008080.svg?style=for-the-badge&logo=NixOS&logoColor=5277C3&labelColor=222222&color=5277C3
[NixOS-url]: https://nixos.org/
[github-actions-shield]: https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=2671E5&labelColor=222222&color=2671E5
[github-actions-url]: https://github.com/features/actions
