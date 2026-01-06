# Development Scripts

This directory contains automation scripts for development and CI workflows.

## Quick Start

```bash
# Run full CI pipeline locally
make ci

# Start infrastructure
make docker-up

# Run application
make run

# Stop infrastructure
make docker-down
```

## Available Scripts

### CI/CD Scripts

**`ci.sh`** - Full CI pipeline (format, lint, test, build)
```bash
./scripts/ci.sh
# or
make ci
```

**`check.sh`** - Fast compilation check
```bash
./scripts/check.sh
# or
make check
```

**`fmt.sh`** - Verify code formatting
```bash
./scripts/fmt.sh
# or
make fmt

# Fix formatting issues:
make fmt-fix
```

**`clippy.sh`** - Run linter
```bash
./scripts/clippy.sh
# or
make clippy
```

**`test.sh`** - Run test suite
```bash
./scripts/test.sh
# or
make test
```

**`build.sh`** - Build workspace
```bash
./scripts/build.sh
# or
make build
```

### Docker Infrastructure Scripts

**`docker-up.sh`** - Start all infrastructure services
```bash
./scripts/docker-up.sh
# or
make docker-up
```

Services started:
- Postgres/TimescaleDB: `localhost:5432`
- pgAdmin: `http://localhost:8081` (admin@example.com / admin)
- Grafana: `http://localhost:3000` (admin / admin)
- Loki: `http://localhost:3100`
- Vector: UDP port 9000 for logs

**`docker-down.sh`** - Stop all infrastructure
```bash
./scripts/docker-down.sh
# or
make docker-down
```

**`docker-logs.sh`** - View service logs
```bash
./scripts/docker-logs.sh postgres-timescale
# or
make docker-logs SERVICE=postgres-timescale
```

Available services:
- `postgres-timescale`
- `pgadmin`
- `grafana`
- `loki`
- `vector`

### Database Scripts

**`db-reset.sh`** - Reset database (destroys all data!)
```bash
./scripts/db-reset.sh
# or
make db-reset
```

This script:
1. Stops the postgres container
2. Removes the data volume
3. Starts a fresh postgres instance
4. Waits for readiness

Use this when:
- Schema changes require fresh migrations
- Test data needs cleanup
- Database is in corrupted state

### Quality Assurance Scripts

**`coverage.sh`** - Generate code coverage report
```bash
./scripts/coverage.sh
# or
make coverage
```

Opens HTML report: `target/llvm-cov/html/index.html`

Requirements:
- Installs `cargo-llvm-cov` if not present

**`bench.sh`** - Run performance benchmarks
```bash
./scripts/bench.sh
# or
make bench
```

Results saved to: `target/criterion/`

## Makefile Targets

The `Makefile` provides convenient aliases for all scripts:

```bash
make help          # Show all available targets
make ci            # Run full CI pipeline
make check         # Run cargo check
make fmt           # Check formatting
make fmt-fix       # Fix formatting
make clippy        # Run linter
make test          # Run tests
make build         # Build workspace
make coverage      # Generate coverage report
make bench         # Run benchmarks
make docker-up     # Start infrastructure
make docker-down   # Stop infrastructure
make docker-logs   # View service logs (specify SERVICE=name)
make db-reset      # Reset database
make run           # Start infrastructure + run app
make clean         # Remove build artifacts
```

## Pre-Commit Workflow

Before committing code:

```bash
# Format code
make fmt-fix

# Run CI checks
make ci
```

If CI passes locally, it should pass in GitHub Actions.

## GitHub Actions Integration

The CI workflow (`.github/workflows/ci.yml`) mirrors local scripts:

1. **Format Check** - Runs `cargo fmt --check`
2. **Clippy Lints** - Runs `cargo clippy -- -D warnings`
3. **Test Suite** - Spins up Postgres service, runs tests
4. **Build** - Builds release binaries, uploads artifacts
5. **Coverage** - Generates coverage report, uploads to Codecov

## Troubleshooting

**Script permission denied:**
```bash
chmod +x scripts/*.sh
```

**Docker services not starting:**
```bash
# Check Docker daemon
docker ps

# View logs
make docker-logs SERVICE=postgres-timescale

# Reset everything
make docker-down
docker system prune -f
make docker-up
```

**Database connection refused:**
```bash
# Reset database
make db-reset

# Verify connection
psql postgresql://postgres:password@localhost:5432/main_db -c '\dt'
```

**Tests failing locally but pass in CI:**
```bash
# Ensure fresh database
make db-reset

# Clear cargo cache
cargo clean
make test
```

## Environment Variables

Scripts respect standard Rust environment variables:

- `CARGO_TERM_COLOR=always` - Colored output
- `RUST_BACKTRACE=1` - Stack traces on panic
- `DATABASE_URL` - Postgres connection string (for tests)

Set in `.env` file for local development:

```bash
# .env
DATABASE_URL=postgres://postgres:password@localhost:5432/main_db
APP_ENVIRONMENT=local
```

## Script Conventions

All scripts follow these conventions:

1. **Bash strict mode:** `set -euo pipefail`
   - Exit on error (`-e`)
   - Treat unset variables as errors (`-u`)
   - Fail on pipe errors (`-o pipefail`)

2. **Verbose output:** Echo operations for transparency

3. **Idempotent:** Safe to run multiple times

4. **Self-documenting:** Clear messages about what's happening

## Adding New Scripts

When adding a new script:

1. Create in `scripts/` directory
2. Add shebang: `#!/usr/bin/env bash`
3. Add strict mode: `set -euo pipefail`
4. Make executable: `chmod +x scripts/your-script.sh`
5. Add Makefile target in `Makefile`
6. Document in this README

Template:
```bash
#!/usr/bin/env bash
set -euo pipefail

echo "==> Running your operation..."
# Commands here

echo "==> Operation complete!"
```
