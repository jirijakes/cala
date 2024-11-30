# Cala

Cala is a robust ledger system developed by Galoy, designed to handle complex financial transactions and accounting operations. It provides a flexible and scalable solution for managing financial records with strong consistency guarantees.

## Features

### Core Capabilities
- **Double-Entry Accounting**: Built-in support for double-entry bookkeeping principles ensuring accurate financial records
- **SQL-Compatible**: Engineered to work with SQL databases (PostgreSQL) for robust data persistence and querying
- **Strong Consistency**: Ensures accuracy and reliability of financial records
- **Real-time Processing**: Efficient transaction processing suitable for production financial systems

### API & Integration
- **GraphQL API**: Modern API interface with built-in playground for easy integration and testing
- **Extensible Architecture**: Modular design with support for custom extensions via the Node.js bindings
- **Transaction Templates**: Customizable transaction templates for common financial operations
- **Multi-Currency Support**: Handle transactions across different currencies

### Core Entities

#### Accounts
- Unique identification via UUID and optional external IDs
- Configurable normal balance type (debit/credit)
- Version-controlled with status tracking (active/locked)
- Rich metadata support with JSON fields
- Balance queries with currency and time-range filtering

#### Account Sets
- Hierarchical account grouping with nested set support
- Journal-scoped for organizational separation
- Consolidated balance reporting across member accounts
- Flexible member management (add/remove operations)

#### Journals
- Independent ledger contexts for transaction isolation
- Support for multiple concurrent journals
- Transaction correlation and external ID tracking

#### Transactions
- Template-based transaction creation
- Effective dating support
- Correlation ID for transaction linking
- Multi-layer support (settled/pending/encumbrance)
- Rich metadata and external ID tracking

#### Velocity Controls
- Configurable transaction limits
- Balance-based and time-based controls
- Multiple limit aggregation support
- Flexible control attachment to accounts/sets

### Technical Specifications

#### Query Interface
- Point lookups by UUID, external ID, and code
- Cursor-based pagination with configurable page sizes
- Balance queries with time range support
- Comprehensive transaction search

#### Type System
- Strong typing with custom scalars (UUID, Timestamp, Date)
- JSON support for flexible metadata
- Decimal type for precise financial calculations
- Expression-based template parameters

#### Implementation Details
- Consistent versioning across all entities
- Optimistic concurrency control
- Immutable audit fields (createdAt, modifiedAt)
- Rich error handling and validation

#### Developer Experience
- Interactive GraphQL playground at `http://localhost:2252/graphql`
- Built-in schema documentation
- Query validation and type checking

## Components

- `cala-ledger`: Core ledger implementation
- `cala-server`: Server implementation handling API requests
- `cala-nodejs`: Node.js bindings for integration with JavaScript/TypeScript applications
- `cala-cel-interpreter` & `cala-cel-parser`: Common Expression Language (CEL) support
- `cala-tracing`: Tracing and monitoring functionality

## Prerequisites

The following dependencies are required:
- Rust (see `rust-toolchain.toml` for version)
- Node.js (for Node.js bindings)
- Docker (for containerized deployment)
- Make

All dependencies can be automatically installed using Nix:
```bash
nix develop
```

This will set up a development environment with all required tools and dependencies.

## Development Setup

1. Clone the repository:
```bash
git clone https://github.com/GaloyMoney/cala.git
cd cala
```

2. Install dependencies:
```bash
make reset-deps
```

## Testing

Run unit tests with:
```bash
make reset-deps next-watch
```

Run end-to-end tests with:
```bash
make e2e
```

## Running the Server

To run the server:
```bash
make run-server
```

## Docker Support

Build the Docker image:
```bash
docker build -t cala .
```

For production builds:
```bash
docker build -f Dockerfile.release -t cala:release .
