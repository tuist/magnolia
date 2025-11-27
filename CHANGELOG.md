# Changelog

All notable changes to this project will be documented in this file.

## [0.6.2] - 2025-11-27

### 🐛 Bug Fixes

- **(deps)** Update rust crate colored to v3 ([3dc256a](3dc256a76f6fc73ea583c19dc1a95220e629a651))

### 📦 Dependency Updates

- **(deps)** Update dependency node to v24 ([e77a59b](e77a59bcabc619254e39f74458af317298948f85))
- **(deps)** Update github artifact actions ([aa09b0d](aa09b0da4d4f5e271e700c9dc4e9dd68aa273670))

## [0.6.1] - 2025-11-27

### 📦 Dependency Updates

- **(deps)** Pin node.js ([47077b8](47077b81d39052a3e0cf5fccc77f28271e2ff4ce))
- **(deps)** Update actions/checkout action to v6 ([eff3bec](eff3bec7100c8e676e24400282922ed320124d9f))
- **(deps)** Update node.js to v24 ([a7894b6](a7894b6973d4a677902f7d8d25b5c25b4345277d))
- **(deps)** Update dependency node to v7 ([29b08b9](29b08b9d7f1c0a71d40d5010192a6ffb922f9b2e))
- **(deps)** Update rust crate clap to v4.5.53 ([ecf76ff](ecf76ff282fd8d12a14b653428ba7ae4dc9c1324))

## [0.6.0] - 2025-11-18

### 🎨 Styling

- Fix formatting ([891c217](891c2170503cf887e7b9c4b39bb1c7e93ad965c4))

### 🐛 Bug Fixes

- Use correct Claude CLI interface for agent execution ([1e70849](1e708496db4b738ea50aa43f9c988ae3aefd8531))
- Resolve clippy warnings and format code ([35becae](35becaec66e8c1ea358e457365f1afbe28348fd3))

### 📚 Documentation

- Add migration testing guide to fixtures directory ([76babdb](76babdb653a051ee7bc96629fc30a591a9ccf637))
- Add quick test guide for migration feature ([2858184](28581849c73656e4cdddc45de72435cd5eb51dea))
- Add comprehensive usage examples to README ([fc35949](fc3594941e4e95ab8705125ca19e57841b861c51))

### 🚀 Features

- Add intelligent CI pipeline migration command ([39d62e5](39d62e55e85530842687c33c8d19dfe7b3cea85d))
- Improve agent CLI error message and add test directory ([c5d3090](c5d3090c9b71648f2dfffb24e6c2d9f31b48cef4))
- Add support for AppCircle and Buildkite CI providers ([c1d5416](c1d5416ee593222e38638ce2337540f398ace0cf))
- Add progress indicators during migration ([c9a753d](c9a753dac591e857217e28f7a6fb63eb9aab6f56))

## [0.5.1] - 2025-11-18

### 📦 Dependency Updates

- **(deps)** Update rust crate clap to v4.5.52 ([f4c6d45](f4c6d452a32538f3abeb05d95958c3fa3fb62ee7))
- **(deps)** Update actions/setup-node action to v6 ([3913301](39133014430521163a41cf997f38e58d679b545b))

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


