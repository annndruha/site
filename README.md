[![Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue)](https://www.apache.org/licenses/LICENSE-2.0)
[![Discord](https://img.shields.io/discord/1296917489615110174?label=discord&logo=discord&logoColor=#5865F2)](https://discord.gg/9HFMDMt95z)

# The Twelve-Factor App Website

This repository contains the source code for the [12factor.net](https://12factor.net) website, the canonical home for the Twelve-Factor App methodology.

## About This Site

The goal of this site is to provide a clear, modern, and multilingual resource for the twelve factors. It serves as the definitive reference for developers and operations engineers building modern cloud-native applications.

This project also serves as a place to document the ongoing evolution of the twelve factors and explore how they apply to new technologies and best practices in software development and delivery.

## Running Locally

The site is built and served by a small Rust application. To run the site locally for development, you will need to have the Rust toolchain installed.

To start the development server, run the helper script:

```shell
./dev.sh
```

This will build the Rust server, start it on `http://localhost:12012`, and open the site in your default browser.

The script provides several commands to manage the local server:

-   `./dev.sh start`: Start the server (default action).
-   `./dev.sh stop`: Stop the server.
-   `./dev.sh restart`: Restart the server.
-   `./dev.sh rebuild`: Rebuild the Rust application and restart the server.
-   `./dev.sh logs`: View the most recent server logs.

## Contributing

Contributions to this site are welcome, especially translations to new languages. The content for the twelve factors is located in the `content/` directory as Markdown files, and UI strings are in the `locales/` directory.

Please see `CONTRIBUTING.md` for more detailed guidelines on how to contribute.

## Governance

This project follows the main [twelve-factor governance](https://github.com/twelve-factor/twelve-factor/blob/next/GOVERNANCE.md), including the code of conduct defined there.