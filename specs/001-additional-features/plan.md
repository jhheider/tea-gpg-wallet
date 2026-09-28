# Implementation Plan: TEA GPG Wallet CLI Tool

**Branch**: `tea-gpg-wallet` | **Date**: 2024-12-19 | **Spec**: `/specs/tea-gpg-wallet/spec.md`
**Input**: Feature specification from `/specs/tea-gpg-wallet/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from file system structure or context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, `GEMINI.md` for Gemini CLI, `QWEN.md` for Qwen Code or `AGENTS.md` for opencode).
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
A Rust CLI tool for interacting with TEA's on-chain GPG rewards wallets, providing secure wallet management using GPG key authentication instead of traditional private keys. The tool supports multiple authentication methods (direct key ID, BPB, GPG email lookup) and enables wallet deployment, TEA transfers, and transaction execution through GPG signatures.

## Technical Context
**Language/Version**: Rust 1.89+ (as specified in Cargo.toml)  
**Primary Dependencies**: Alloy (Ethereum interactions), Clap (CLI), Tokio (async), anyhow (error handling)  
**Storage**: N/A (stateless CLI tool)  
**Testing**: cargo test with 80% coverage target  
**Target Platform**: Cross-platform CLI (Linux, macOS, Windows)  
**Project Type**: single (CLI tool with library components)  
**Performance Goals**: 30s wallet operations, 5s queries, 1s startup, <50MB memory  
**Constraints**: No persistent key storage, secure subprocess handling, retry logic with exponential backoff  
**Scale/Scope**: Single-user CLI tool, concurrent operations support, large TEA amount precision  

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Security First**: ✅ All cryptographic operations use proven libraries (Alloy), GPG key handling never exposes private keys, all inputs validated, secure subprocess handling specified
**User Experience Excellence**: ✅ Clear CLI interface with colored output, progress indicators, multiple authentication methods, comprehensive error messages
**Test-First (NON-NEGOTIABLE)**: ✅ TDD approach specified, 80% coverage target, CLI command testing with real GPG keys, error condition testing
**Code Quality Standards**: ✅ Modular architecture (deployer, wallet, CLI, utils), single responsibility principle, async/await usage, anyhow error handling
**Performance & Observability**: ✅ Measurable performance targets (30s/5s/1s), memory limits (50MB), progress indicators, structured logging

**Status**: PASS - All constitutional principles satisfied

## Project Structure

### Documentation (this feature)
```
specs/tea-gpg-wallet/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
cli/
├── src/
│   ├── main.rs          # CLI entry point and command parsing
│   ├── bpb.rs           # BPB authentication integration
│   ├── gpg.rs           # GPG authentication integration
│   ├── monitoring.rs    # Performance monitoring
│   ├── output.rs        # Colored output and metrics formatting
│   └── utils.rs         # CLI utilities (formatting, validation)

lib/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── deployer.rs      # Wallet deployment and prediction
│   ├── wallet.rs        # Wallet operations and transaction signing
│   └── utils.rs         # Common utilities (conversion, validation)

tests/
├── integration/         # End-to-end CLI tests
├── unit/               # Unit tests for library components
└── fixtures/           # Test GPG keys and data
```

**Structure Decision**: Single project with CLI and library separation, following Rust workspace conventions

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - All technical choices are resolved from specification
   - Dependencies are well-defined (Alloy, Clap, Tokio, anyhow)
   - Integration patterns are specified (GPG, BPB, blockchain RPC)

2. **Generate and dispatch research agents**:
   ```
   For blockchain integration:
     Task: "Research Alloy best practices for GPG wallet contract interaction"
   For CLI UX:
     Task: "Research progress indicators and colored output patterns in Rust CLI tools"
   For authentication:
     Task: "Research secure subprocess handling for GPG/BPB integration"
   For error handling:
     Task: "Research retry patterns with exponential backoff in Rust"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - AddressPrediction, SigningData, WalletStatus, SigningResult
   - Validation rules from requirements (16-char hex keys, checksum addresses)
   - State transitions (not deployed → deployed, balance changes)

2. **Generate API contracts** from functional requirements:
   - CLI commands as "endpoints" with input/output specifications
   - Library API contracts for deployer, wallet, utils modules
   - Output interface specifications to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per module/command
   - Assert input validation and output formatting
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each CLI command → integration test scenario
   - Quickstart test = complete workflow validation

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/bash/update-agent-context.sh cursor`
     **IMPORTANT**: Execute it exactly as specified above. Do not add or remove any arguments.
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each contract → contract test task [P]
- Each entity → model creation task [P] 
- Each CLI command → integration test task
- Implementation tasks to make tests pass

**Ordering Strategy**:
- TDD order: Tests before implementation 
- Dependency order: Utils → Library modules → CLI commands
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 39 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

No violations - all constitutional principles satisfied.

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [x] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented

---
*Based on Constitution v1.0.0 - See `/memory/constitution.md`*