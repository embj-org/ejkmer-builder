# EJ Kmer Builder

EJ Kmer Builder is a demonstration tool that showcases how to integrate the [EJ Builder SDK](https://crates.io/crates/ej-builder-sdk) for automated embedded testing.
This tool manages the build and deployment process for the [k-mer algorithm benchmark project](https://github.com/embj-org/kmer) across different embedded hardware platforms.

It was built as part of the EJ Guide series that can be found [here](https://embj-org.github.io/ej/).

## What It Does

This builder application demonstrates what is possible with EJ and the EJ Builder SDK features including:

- **Cross-compilation management**: Automatically builds k-mer applications for ARM64 targets
- **Remote deployment**: Handles secure file transfer to embedded devices via SCP
- **Job cancellation**: Properly cleans up remote processes when jobs are cancelled

## Installation

### From Git

```bash
cargo install --git https://github.com/embj-org/ejkmer-builder
```

### From Source

```bash
git clone https://github.com/embj-org/ejkmer-builder.git
cd ejkmer-builder
cargo install --path .
```

## Prerequisites

Before using this builder, ensure you have:

- **Rust toolchain**: Latest stable version
- **AArch64 cross-compiler**: For ARM64 target support
- **SSH access**: To your target embedded devices
- **CMake**: For building the k-mer applications
- **EJ framework**: Either EJB standalone or EJD dispatcher setup

## Usage

The builder is typically invoked by EJ framework components (EJB), but can be used directly:

```bash
# When called by EJ, receives these arguments:
# ejkmer-builder <action> <config_path> <board_name> <board_config_name> <socket_path>

# The application automatically determines whether to build or run based on the action parameter
```

## Integration with EJ Guides

This project serves as the primary example in [EJ Guide 02 - Builder SDK](https://embj-org.github.io/ej/02-BuilderSDK.html), demonstrating:

1. **Migration from shell scripts**: Shows how to convert basic deployment scripts to robust SDK-based tools
2. **Job cancellation handling**: Demonstrates proper cleanup when tests are interrupted
3. **Production deployment patterns**: Illustrates best practices for embedded test automation

## Comparison with Shell Scripts

This Rust-based builder provides several advantages over simple shell scripts:

- Cleans up remote processes when jobs are cancelled  
- Clear error messages and proper exit codes  
- You get to write code in Rust

And some incoveniences:

- Requires compilation step
- Need familiarity with Rust ecosystem
- You get to write code in Rust

## Related Projects

- [K-mer Benchmark](https://github.com/embj-org/kmer) - The application this builder deploys
- [EJ Framework](https://github.com/embj-org/ej) - The testing framework this integrates with
- [EJ Kmer Dispatcher](https://github.com/embj-org/ejkmer-dispatcher) - Companion dispatcher SDK example
- [EJ Builder SDK](https://crates.io/crates/ej-builder-sdk) - The SDK this project demonstrates

## Contributing

Contributions are welcome! This project serves as a reference implementation for the EJ Builder SDK, so improvements that demonstrate best practices or additional SDK features are particularly valuable.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For questions about this builder or the EJ framework:

- Check the [EJ Documentation](https://embj-org.github.io/ej/)
- Visit the [EJ GitHub Repository](https://github.com/embj-org/ej)
- Review the [Builder SDK Documentation](https://crates.io/crates/ej-builder-sdk)
