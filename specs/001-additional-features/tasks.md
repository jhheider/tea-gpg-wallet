# Tasks: TEA GPG Wallet CLI Tool

**Input**: Design documents from `/specs/tea-gpg-wallet/`
**Prerequisites**: plan.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓

## Execution Flow (main)
```
1. Load plan.md from feature directory ✓
   → Tech stack: Rust 1.89+, Alloy, Clap v4, Tokio, anyhow, indicatif, colored
   → Structure: CLI tool with library components (deployer, wallet, utils)
2. Load design documents ✓
   → data-model.md: 4 core entities (AddressPrediction, SigningData, WalletStatus, SigningResult)
   → contracts/: 2 contract files (cli-interface.md, library-api.md)
   → research.md: Technology decisions and patterns
3. Generate tasks by category ✓
   → Setup: Rust project, dependencies, linting
   → Tests: Contract tests, integration tests, unit tests
   → Core: Data models, library modules, CLI commands
   → Integration: Error handling, retry logic, configuration
   → Polish: Documentation, performance, final testing
4. Apply task rules ✓
   → Different files = [P] for parallel
   → Tests before implementation (TDD)
   → Dependencies respected
5. Number tasks sequentially (T001-T039) ✓
6. Generate dependency graph ✓
7. Create parallel execution examples ✓
8. Validate task completeness ✓
   → All contracts have tests ✓
   → All entities have models ✓
   → All CLI commands implemented ✓
9. Return: SUCCESS (tasks ready for execution) ✓
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `cli/src/`, `lib/src/`, `tests/` at repository root
- Paths based on existing Rust workspace structure

## Phase 3.1: Setup
- [x] T001 Create project structure per implementation plan
- [x] T002 Initialize Rust workspace with dependencies (Alloy, Clap v4, Tokio, anyhow, indicatif, colored)
- [x] T003 [P] Configure rustfmt and clippy linting

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [x] T004 [P] Contract test CLI interface in tests/contract/test_cli_interface.rs
- [x] T005 [P] Contract test library API in tests/contract/test_library_api.rs
- [x] T006 [P] Integration test complete workflow in tests/integration/test_complete_workflow.rs
- [x] T007 [P] Integration test authentication methods in tests/integration/test_auth_methods.rs
- [x] T008 [P] Unit test data model validation in tests/unit/test_data_models.rs
- [x] T009 [P] Unit test utility functions in tests/unit/test_utils.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [x] T010 [P] AddressPrediction model in lib/src/models.rs
- [x] T011 [P] SigningData model in lib/src/models.rs
- [x] T012 [P] WalletStatus model in lib/src/models.rs
- [x] T013 [P] SigningResult model in lib/src/models.rs
- [x] T014 [P] Utils module in lib/src/utils.rs (decimal conversion, validation, formatting)
- [x] T015 [P] Deployer module in lib/src/deployer.rs (predict, deploy, send, balance)
- [x] T016 [P] Wallet module in lib/src/wallet.rs (signing, execute, sweep, status)
- [x] T017 [P] BPB authentication in cli/src/bpb.rs
- [x] T018 [P] GPG authentication in cli/src/gpg.rs
- [x] T019 [P] CLI utilities in cli/src/utils.rs (formatting, colors, validation)
- [x] T020 CLI main command structure in cli/src/main.rs
- [x] T021 CLI config command implementation
- [x] T022 CLI find command implementation
- [x] T023 CLI deploy command implementation
- [x] T024 CLI send command implementation
- [x] T025 CLI execute command implementation
- [x] T026 CLI sweep command implementation
- [x] T027 CLI status command implementation

## Phase 3.4: Integration
- [ ] T028 Error handling and retry logic with exponential backoff
- [ ] T029 Configuration management (environment variables, config file, CLI flags)
- [ ] T030 Progress indicators and colored output formatting
- [ ] T031 [P] Performance monitoring implementation (operation timing, memory usage, gas costs, network latency, retry counts)
- [ ] T032 [P] Verbosity levels and metrics output formatting

## Phase 3.5: Polish
- [ ] T033 [P] Comprehensive unit test coverage (80% target)
- [ ] T034 Performance testing (30s operations, 5s queries, 1s startup)
- [ ] T035 [P] Update README.md with installation and usage
- [ ] T036 [P] Generate CLI help documentation
- [ ] T037 Security audit and input validation testing
- [ ] T038 Final integration testing with real GPG keys
- [ ] T039 Remove code duplication and optimize

## Dependencies
- Tests (T004-T009) before implementation (T010-T027)
- Models (T010-T013) before modules (T015-T016)
- Utils (T014) before other modules
- Modules (T015-T016) before CLI commands (T021-T027)
- CLI structure (T020) before individual commands
- Core implementation before integration (T028-T032)
- Integration before polish (T033-T039)

## Parallel Execution Examples

### Launch T004-T009 together (Contract and Integration Tests):
```
Task: "Contract test CLI interface in tests/contract/test_cli_interface.rs"
Task: "Contract test library API in tests/contract/test_library_api.rs"
Task: "Integration test complete workflow in tests/integration/test_complete_workflow.rs"
Task: "Integration test authentication methods in tests/integration/test_auth_methods.rs"
Task: "Unit test data model validation in tests/unit/test_data_models.rs"
Task: "Unit test utility functions in tests/unit/test_utils.rs"
```

### Launch T010-T014 together (Data Models and Utils):
```
Task: "AddressPrediction model in lib/src/models.rs"
Task: "SigningData model in lib/src/models.rs"
Task: "WalletStatus model in lib/src/models.rs"
Task: "SigningResult model in lib/src/models.rs"
Task: "Utils module in lib/src/utils.rs"
```

### Launch T015-T019 together (Core Modules and Auth):
```
Task: "Deployer module in lib/src/deployer.rs"
Task: "Wallet module in lib/src/wallet.rs"
Task: "BPB authentication in cli/src/bpb.rs"
Task: "GPG authentication in cli/src/gpg.rs"
Task: "CLI utilities in cli/src/utils.rs"
```

### Launch T031-T032 together (Performance Monitoring):
```
Task: "Performance monitoring implementation (operation timing, memory usage, gas costs, network latency, retry counts)"
Task: "Verbosity levels and metrics output formatting"
```

### Launch T033, T035-T036 together (Polish Documentation):
```
Task: "Comprehensive unit test coverage in tests/unit/"
Task: "Update README.md with installation and usage"
Task: "Generate CLI help documentation"
```

## Task Details

### T004: Contract Test CLI Interface
**File**: `tests/contract/test_cli_interface.rs`
**Purpose**: Test all CLI commands according to cli-interface.md contract
**Test Cases**:
- config command output format (RPC URL, deployer address, network info, config source)
- find command with all authentication methods (direct key ID, --bpb, --gpg email)
- deploy command with validation (PRIVATE_KEY requirement, deployment confirmation, transaction hash)
- send command with amount validation (decimal amounts, before/after balances, auto-deployment)
- execute command with optional data (hex validation, 1KB max length, signature verification)
- sweep command with zero balance handling (direct key ID support, graceful zero balance)
- status command with comprehensive output (address, status, balance, nonce, GPG verifier, transaction count)
- Error message format consistency (brief description, details, suggestions, help reference)
- Exit code validation (0=success, 1=system error, 2=user error, 3=security error)
- Global options testing (--help, --version, --verbose, --json, --config, --rpc-url, --retry-*)

### T005: Contract Test Library API
**File**: `tests/contract/test_library_api.rs`
**Purpose**: Test library API according to library-api.md contract
**Test Cases**:
- deployer::predict_address() (key ID validation, address prediction, deployment status check)
- deployer::ensure_deployed() (private key validation, deployment transaction, error handling)
- deployer::send_to_gpg_key() (amount validation, balance checking, transaction submission)
- wallet::get_signable_hash() (SigningData::Withdraw variant, deadline validation, hash generation)
- wallet::get_execute_signable_hash() (SigningData::Execute variant, amount/data validation, deadline)
- wallet::sweep_gpg_key() (signature verification, balance transfer, zero balance handling)
- wallet::execute_gpg_transaction() (parameter validation, signature verification, transaction execution)
- wallet::get_wallet_status() (complete status retrieval, field validation, error handling)
- utils::decimal_to_wei_precise() (decimal format validation, precision handling, overflow protection)
- utils::wei_to_eth_auto() (formatting accuracy, large/small amount handling)
- utils::key_id_to_bytes() (hex validation, length checking, byte conversion)
- utils::validate_address() (checksum validation, format checking, error cases)
- Error handling and retry logic (WalletError variants, exponential backoff, network failures)

### T006: Integration Test Complete Workflow
**File**: `tests/integration/test_complete_workflow.rs`
**Purpose**: Test complete user workflow from quickstart.md
**Test Scenarios**:
- Setup and configuration check
- Wallet address prediction
- Wallet deployment
- TEA token sending
- Balance verification
- Transaction execution with GPG signatures
- Fund sweeping
- Status checking

### T007: Integration Test Authentication Methods
**File**: `tests/integration/test_auth_methods.rs`
**Purpose**: Test all authentication methods (direct key, BPB, GPG email)
**Test Scenarios**:
- Direct GPG key ID usage
- BPB integration (mocked)
- GPG email lookup
- Error handling for missing tools
- Fallback behavior

### T008: Unit Test Data Model Validation
**File**: `tests/unit/test_data_models.rs`
**Purpose**: Test data model validation rules
**Test Cases**:
- AddressPrediction validation
- SigningData deadline validation
- WalletStatus field validation
- SigningResult format validation
- Error type coverage
- State transition validation

### T009: Unit Test Utility Functions
**File**: `tests/unit/test_utils.rs`
**Purpose**: Test utility functions with edge cases
**Test Cases**:
- decimal_to_wei_precise() with various formats
- wei_to_eth_auto() with large/small amounts
- key_id_to_bytes() validation
- address validation
- Error handling for invalid inputs

### T031: Performance Monitoring Implementation
**File**: `lib/src/monitoring.rs`
**Purpose**: Implement performance monitoring for operations
**Requirements**:
- Operation timing measurement (start/end timestamps)
- Memory usage tracking (peak, current, allocated)
- Gas cost calculation and display
- Network latency measurement (RPC call timing)
- Retry count tracking and reporting
- Integration with CLI verbosity levels

### T032: Verbosity Levels and Metrics Output Formatting
**File**: `cli/src/output.rs`
**Purpose**: Implement verbosity levels and metrics formatting
**Requirements**:
- Verbosity level parsing (--verbose, -V flags)
- Metrics output formatting (human-readable and JSON)
- Performance metrics display (operation duration, memory, gas, latency, retries)
- Conditional output based on verbosity level
- Integration with all CLI commands

## Notes
- [P] tasks = different files, no dependencies
- Verify tests fail before implementing
- Commit after each task
- Avoid: vague tasks, same file conflicts
- All tasks must be executable by LLM without additional context

## Validation Checklist
✓ All contracts have corresponding tests (T004-T005)
✓ All entities have model tasks (T010-T013)
✓ All tests come before implementation (T004-T009 before T010-T027)
✓ Parallel tasks truly independent
✓ Each task specifies exact file path
✓ No task modifies same file as another [P] task
✓ CLI commands cover all functionality from specification
✓ Integration tests cover complete user workflows
✓ Performance and security requirements addressed
✓ Performance monitoring tasks added (T031-T032)
✓ Contract tests have specific test cases
✓ SigningData enum variants reflected in tasks