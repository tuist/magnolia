# Migration Architecture

This document describes the architecture of Magnolia's CI pipeline migration feature.

## Overview

The migration feature enables intelligent conversion of CI pipelines from external providers (Bitrise, Codemagic, CircleCI) to Git forge native CI systems (GitHub Actions, GitLab CI, Forgejo Actions).

## Architecture

### Components

```
┌─────────────────────────────────────────────────────────────┐
│                        User Interface                        │
│                    (magnolia migrate)                        │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   Migration Orchestrator                     │
│                    (src/migrate.rs)                          │
│  - Source detection (Bitrise, Codemagic, CircleCI)         │
│  - Target detection (from git origin)                       │
│  - User interaction (selection, confirmation)               │
│  - File I/O (read source, write target)                    │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                    Agent Client Protocol                     │
│                     (src/agent.rs)                           │
│  - CLI detection (claude/codex)                             │
│  - MCP-based communication                                  │
│  - Task delegation with context                             │
└──────────────────────────┬──────────────────────────────────┘
                           │
          ┌────────────────┼────────────────┐
          ▼                ▼                ▼
┌─────────────────┐ ┌──────────────┐ ┌─────────────────┐
│  Main Agent     │ │   Doc Agent  │ │ Validator Agent │
│  (Orchestrate)  │ │  (Research)  │ │  (Test & Fix)   │
└─────────────────┘ └──────────────┘ └─────────────────┘
```

### Workflow

1. **Detection Phase**
   - Scan repository for source CI configs at conventional paths
   - Parse git remote origin to determine target CI system
   - Present options to user if multiple sources found

2. **Analysis Phase**
   - Read source configuration file
   - Collect git context (remote URL, branch)
   - Prepare context for agent delegation

3. **Migration Phase**
   - Main agent analyzes source config
   - Documentation agent researches both CI systems
   - Main agent generates target configuration
   - Apply transformations and mappings

4. **Verification Phase** (unless `--no-verify`)
   - Write generated config to temporary location
   - Use existing `magnolia` execution engine to run pipeline
   - Validator agent fixes issues if verification fails
   - Iterate until validation succeeds

5. **Finalization Phase**
   - Write final configuration to target location
   - Create necessary directories
   - Report success to user

## Agent Delegation Model

### Main Migration Agent
- **Input**: Source config, target CI, git context
- **Role**: Orchestrates overall migration process
- **Output**: Generated target configuration

### Documentation Agent
- **Input**: CI system name, feature query
- **Role**: Researches documentation for feature equivalencies
- **Output**: Feature mappings and recommendations

### Validator Agent
- **Input**: Generated config, error messages
- **Role**: Analyzes failures and suggests fixes
- **Output**: Corrected configuration

## Design Decisions

### Why Agentic Protocol?

CI migration is inherently complex because:
1. Different CI systems use different paradigms and terminology
2. Feature equivalencies aren't always 1:1
3. Documentation for both systems must be consulted
4. Edge cases require semantic understanding, not just syntax translation

The agentic approach allows:
- Leverage LLM knowledge of CI systems
- Research up-to-date documentation dynamically
- Handle edge cases intelligently
- Iterate on failures until success

### Why Hybrid Agent Model?

Instead of a single monolithic agent or many specialized agents, we use a hybrid:
- **Main agent** has full context and makes final decisions
- **Specialized agents** handle discrete sub-tasks (research, validation)
- Reduces complexity while maintaining flexibility
- Allows for parallel sub-task execution in future

### Auto-detection Strategy

Both source and target are auto-detected to minimize user friction:
- Source: Scan conventional file paths (industry standard locations)
- Target: Parse git remote (where the code will be pushed)
- Override flags available for edge cases

## Testing Strategy

### Unit Tests (`src/migrate.rs`, `src/agent.rs`)
- CI system name mapping
- Target path generation
- Source config detection
- Agent task serialization

### Integration Tests (`tests/migrate_test.rs`)
- Detection of all source CI configs in fixtures
- Multi-source scenario handling
- Target path correctness

### Fixtures (`fixtures/`)
- Real-world CI configurations for all supported systems
- Used for both testing and development

## Future Enhancements

1. **Caching**: Cache agent responses for identical source configs
2. **Rollback**: Implement rollback mechanism if verification fails
3. **Partial Migration**: Support migrating specific jobs/workflows
4. **Custom Mappings**: Allow users to define custom feature mappings
5. **Batch Migration**: Migrate multiple pipelines in one command
6. **Migration Reports**: Generate detailed migration reports with warnings
