# Tea GPG Wallet Constitution

## Core Principles

### I. Security First
All cryptographic operations MUST use proven, well-audited libraries and algorithms. GPG key handling MUST never expose private keys or sensitive data. All user inputs MUST be validated and sanitized before processing. Security-sensitive errors MUST not leak sensitive information. Dependencies MUST be kept minimal and regularly audited for vulnerabilities.

### II. User Experience Excellence
All commands, error messages, and documentation MUST be clear and unambiguous. Command-line interface, error formats, and output styling MUST be consistent across all features. Support multiple authentication methods (GPG, BPB, direct key ID) for different user preferences. Provide meaningful progress indicators and status updates for long-running operations. Handle missing dependencies or tools with helpful error messages and suggestions.

### III. Test-First (NON-NEGOTIABLE)
TDD mandatory: Tests written → User approved → Tests fail → Then implement. Red-Green-Refactor cycle strictly enforced. Minimum 80% line coverage for all library code. Test all CLI commands with real GPG keys. Test all error conditions and edge cases. Test input validation, key handling, and error message security.

### IV. Code Quality Standards
Separate concerns into distinct modules (deployer, wallet, CLI, utils). Use appropriate abstractions to hide implementation details. Each function, module, and struct MUST have a single, well-defined purpose. High-level modules MUST not depend on low-level modules. Use `anyhow::Result<T>` for all fallible operations with descriptive error messages. Use async/await consistently for I/O operations.

### V. Performance & Observability
Wallet operations MUST complete within 30 seconds. Query operations MUST complete within 5 seconds. CLI startup MUST complete within 1 second. Memory usage MUST stay under 50MB. Provide meaningful progress indicators and status updates. Use structured logging for debugging without cluttering user output.

## Additional Constraints

### Technology Stack Requirements
- Rust 1.89+ with Alloy for Ethereum interactions
- Clap v4 for CLI interface
- Tokio for async runtime
- anyhow for error handling
- indicatif for progress indicators
- colored for output formatting

### Security Requirements
- Never store private keys persistently
- Use secure communication channels (HTTPS)
- Prevent command injection in subprocess calls
- Validate all inputs before processing
- Implement proper key verification

### Performance Standards
- 30s wallet operations, 5s queries, 1s startup
- <50MB memory usage
- Support concurrent operations
- Implement retry logic with exponential backoff

## Development Workflow

### Code Review Requirements
All code changes MUST be reviewed by at least one maintainer. Security-sensitive changes require review by security-focused maintainer. All new features MUST include appropriate tests. All public APIs MUST be documented before merge.

### Release Standards
Follow semantic versioning for all releases. Maintain detailed changelog for all releases. All releases MUST pass full test suite. Security-sensitive releases MUST undergo security review.

### Continuous Integration
All tests MUST pass in CI before merge. Code MUST pass formatting and linting checks. Regular dependency vulnerability scanning. Monitor performance regressions.

## Governance

Constitution supersedes all other practices. Amendments require documentation, approval, migration plan. All PRs/reviews MUST verify compliance. Complexity MUST be justified. Use CONSTITUTION.md for runtime development guidance.

**Version**: 1.0.0 | **Ratified**: 2024-12-19 | **Last Amended**: 2024-12-19