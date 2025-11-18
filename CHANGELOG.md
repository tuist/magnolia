# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2025-11-18

### 🐛 Bug Fixes

- **(deps)** Update rust crate inquire to 0.9 ([549d4d3](549d4d37495c998970a434b4829be3af34f92dbe))

### 📚 Documentation

- Simplify CLAUDE.md to only reference PLAN.md ([68b0d89](68b0d89c9bdc76abb880af53bd03c7d653566c7e))

### 📦 Dependency Updates

- **(deps)** Update actions/checkout action to v5 ([c441408](c4414082fd275df95fcfb1da147a632b43bca92f))

### 🚀 Features

- Add matrix strategy support for GitHub Actions ([9053e22](9053e220ea683c8a27654db08443c322b7eb92cc))

## [0.4.0] - 2025-11-17

### 🎨 Styling

- Apply cargo fmt formatting ([ed282fd](ed282fd2eeed81205256567e5a0e8a19a560c763))
- Fix remaining formatting issues ([e2101db](e2101db38cfdf23e290ed2d83cac6257c72a28d5))

### 🐛 Bug Fixes

- Add GitHub Actions environment variables for action execution ([45f2c76](45f2c76b8f760c1099739424abcf2f3b74dc5506))
- Add support for action inputs (with: keyword) ([b6e73ab](b6e73ab368caaae93790886b529dac29f42a6199))
- Use non-empty dummy token for local execution ([26f57e3](26f57e3064e03e76d824d55bec7d0104b4463fa3))
- Resolve all clippy warnings ([0622872](062287224c4a38f40b7182205c42627254104f7f))

### 🚀 Features

- Add full GitHub Actions marketplace support ([60f5e35](60f5e3515be0da73dcbde763ccbbe18331bd90f2))
- Filter GitHub Actions workflow commands from output ([1fc2fe1](1fc2fe1b5c7ee36547dd391dc4729b0fec54f8bb))
- Use git credential helper for GitHub authentication ([c98f53f](c98f53f740796f23f8b10db7a240c6ff2e938ec1))
- Filter verbose [command] output from actions ([2a66843](2a6684370a8eac049b2343599f3f0d30788667bd))

## [0.3.0] - 2025-11-16

### 🚀 Features

- Add container execution for GitHub Actions and Forgejo Actions ([6377163](637716389272c90dd95f7ed7ab9c0e4c6eee5695))

## [0.2.0] - 2025-11-16

### 🚀 Features

- Add container-based execution with Podman/Docker support ([8b5dcd8](8b5dcd8b6159ff17ffca6a333c465e5ac8192036))

## [0.1.3] - 2025-11-16

### 📚 Documentation

- Add line breaks to Magnolia Manifesto ([3533bcd](3533bcdc5ec07f2c8279c0f99b76a33162c58f07))

## [0.1.2] - 2025-11-16

### 📚 Documentation

- Improve Magnolia Manifesto formatting ([7fe098d](7fe098d3110f89d8bb8769e489a0ab83a196e698))

## [0.1.1] - 2025-11-16

### 📚 Documentation

- Add Magnolia Manifesto to README ([c12f212](c12f212503eceb8f86111d6f71552aea4b2a1c1b))

## [0.1.0] - 2025-11-13

### 🎨 Styling

- Apply cargo fmt formatting ([949bb6c](949bb6c1a17b6fbcb8ea08ddc5c558df52bc5037))

### 📚 Documentation

- Add emojis to README for better visual appeal ([96bfd42](96bfd422b979849fff0f14248efb0785d85b77b3))

### 🚀 Features

- Initial release of magnolia CLI ([2a14a27](2a14a273321612726dcf026b93a061a6e8974ce6))
- Simplify CLI interface and add interactive selection ([db6698b](db6698b78b80570cf13e92ad1a23d335ecb13f76))
- Implement pipeline execution with confirmation prompts ([c606025](c60602576339c0af5af44e057ceb8aa16e3c3869))

### 🚜 Refactor

- Reorganize CI workflows and add magnolia.yml ([e580574](e580574cb79be6cf257224245e81237f064cd73f))


