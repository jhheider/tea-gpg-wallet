# Tea GPG Wallet Constitution

## Preamble

This constitution establishes the fundamental principles, standards, and guidelines that govern the development, maintenance, and evolution of the tea-gpg-wallet project. All contributors, maintainers, and users are expected to uphold these principles to ensure the project's continued success, security, and usability.

## Core Principles

### 1. Security First
- **Cryptographic Security**: All cryptographic operations must use proven, well-audited libraries and algorithms
- **Key Management**: GPG key handling must never expose private keys or sensitive data
- **Input Validation**: All user inputs must be validated and sanitized before processing
- **Error Handling**: Security-sensitive errors must not leak sensitive information
- **Dependency Management**: Keep dependencies minimal and regularly audit for vulnerabilities

### 2. User Experience Excellence
- **Clarity**: All commands, error messages, and documentation must be clear and unambiguous
- **Consistency**: Command-line interface, error formats, and output styling must be consistent across all features
- **Accessibility**: Support multiple authentication methods (GPG, BPB, direct key ID) for different user preferences
- **Feedback**: Provide meaningful progress indicators and status updates for long-running operations
- **Graceful Degradation**: Handle missing dependencies or tools with helpful error messages and suggestions

## Code Quality Standards

### 3. Code Organization
- **Modularity**: Separate concerns into distinct modules (deployer, wallet, CLI, utils)
- **Abstraction**: Use appropriate abstractions to hide implementation details
- **Single Responsibility**: Each function, module, and struct should have a single, well-defined purpose
- **Dependency Direction**: High-level modules should not depend on low-level modules

### 4. Rust Best Practices
- **Error Handling**: Use `anyhow::Result<T>` for all fallible operations with descriptive error messages
- **Ownership**: Prefer borrowing over ownership where possible, use `String` over `&str` only when necessary
- **Type Safety**: Leverage Rust's type system to prevent runtime errors
- **Async/Await**: Use async/await consistently for I/O operations
- **Documentation**: Document all public APIs with rustdoc comments including examples

### 5. Code Style
- **Formatting**: Use `rustfmt` with default settings
- **Linting**: Pass `clippy` with no warnings
- **Naming**: Use descriptive names following Rust conventions (snake_case for functions/variables, PascalCase for types)
- **Comments**: Write comments for complex logic, not obvious code
- **Constants**: Use const for compile-time constants, avoid magic numbers

## Testing Standards

### 6. Test Coverage Requirements
- **Unit Tests**: Minimum 80% line coverage for all library code
- **Integration Tests**: Test all CLI commands with real GPG keys (in CI with test keys)
- **Error Path Testing**: Test all error conditions and edge cases
- **Security Testing**: Test input validation, key handling, and error message security

### 7. Test Organization
- **Test Structure**: Follow the pattern of one test module per source module
- **Test Data**: Use consistent test GPG keys and addresses across all tests
- **Mocking**: Mock external dependencies (GPG, BPB) in unit tests
- **Environment**: Use environment variables for test configuration, never hardcode secrets

### 8. Test Quality
- **Descriptive Names**: Test names should clearly describe what is being tested
- **Independence**: Tests must be independent and runnable in any order
- **Deterministic**: Tests must produce consistent results
- **Fast**: Unit tests should complete in under 1 second, integration tests under 10 seconds

## User Experience Consistency

### 9. Command-Line Interface Standards
- **Command Structure**: Follow consistent patterns for subcommands and arguments
- **Help Text**: Provide comprehensive help text with examples for all commands
- **Argument Validation**: Validate arguments early with clear error messages
- **Exit Codes**: Use standard Unix exit codes (0 for success, 1 for general errors, 2 for usage errors)

### 10. Output Formatting
- **Colored Output**: Use consistent color schemes (blue for labels, green for success, red for errors)
- **Progress Indicators**: Use spinners for operations >1 second, progress bars for operations with known duration
- **Address Formatting**: Always display addresses with 0x prefix and consistent formatting
- **Amount Formatting**: Use consistent decimal formatting for TEA amounts (auto-remove trailing zeros)

### 11. Error Handling
- **Error Messages**: Provide actionable error messages with suggestions for resolution
- **Error Context**: Include relevant context (key ID, address, amount) in error messages
- **Graceful Failures**: Handle missing tools (GPG, BPB) with helpful installation instructions
- **Logging**: Use appropriate log levels for debugging without cluttering user output

## Performance Requirements

### 12. Response Time Standards
- **Wallet Operations**: Deploy, send, and sweep operations must complete within 30 seconds
- **Query Operations**: Balance checks and address predictions must complete within 5 seconds
- **CLI Startup**: Application startup must complete within 1 second
- **Help/Config**: Configuration display must be instantaneous

### 13. Resource Usage
- **Memory**: Keep memory usage under 50MB for typical operations
- **CPU**: Minimize CPU usage during idle periods
- **Network**: Batch network requests where possible, implement connection pooling
- **Disk**: Minimize temporary file usage, clean up after operations

### 14. Scalability Considerations
- **Concurrent Operations**: Support multiple concurrent wallet operations
- **Large Amounts**: Handle large TEA amounts without precision loss
- **Multiple Keys**: Efficiently handle operations across multiple GPG keys
- **Network Resilience**: Implement retry logic for network failures

## Security Requirements

### 15. Key Handling
- **No Key Storage**: Never store private keys or sensitive data persistently
- **Secure Transmission**: Use secure channels for all network communication
- **Key Validation**: Validate GPG keys before use, check for revocation
- **Signature Verification**: Verify all signatures before accepting them

### 16. Input Security
- **Sanitization**: Sanitize all user inputs before processing
- **Validation**: Validate addresses, amounts, and key IDs with strict patterns
- **Injection Prevention**: Prevent command injection in GPG/BPB subprocess calls
- **Buffer Management**: Use safe buffer operations to prevent overflow

## Documentation Standards

### 17. Code Documentation
- **API Documentation**: Document all public APIs with rustdoc comments
- **Examples**: Include usage examples in all public function documentation
- **README**: Maintain comprehensive README with installation, usage, and troubleshooting
- **Architecture**: Document system architecture and design decisions

### 18. User Documentation
- **Command Reference**: Maintain complete command reference with examples
- **Troubleshooting**: Provide common problem solutions and debugging steps
- **Security Guide**: Document security considerations and best practices
- **Migration Guide**: Provide upgrade instructions for breaking changes

## Compliance and Enforcement

### 19. Code Review Requirements
- **All Changes**: All code changes must be reviewed by at least one maintainer
- **Security Review**: Security-sensitive changes require review by security-focused maintainer
- **Testing**: All new features must include appropriate tests
- **Documentation**: All public APIs must be documented before merge

### 20. Release Standards
- **Semantic Versioning**: Follow semantic versioning for all releases
- **Changelog**: Maintain detailed changelog for all releases
- **Testing**: All releases must pass full test suite
- **Security Audit**: Security-sensitive releases must undergo security review

### 21. Continuous Integration
- **Automated Testing**: All tests must pass in CI before merge
- **Code Quality**: Code must pass formatting and linting checks
- **Security Scanning**: Regular dependency vulnerability scanning
- **Performance Monitoring**: Monitor performance regressions

## Amendment Process

This constitution may be amended through the following process:
1. Propose amendment with clear rationale and impact analysis
2. Open discussion period (minimum 7 days)
3. Consensus building among maintainers
4. Implementation of approved amendments
5. Update documentation and communication to community

---

*This constitution is a living document that evolves with the project. All contributors are encouraged to suggest improvements while maintaining the core principles of security, quality, and user experience.*
