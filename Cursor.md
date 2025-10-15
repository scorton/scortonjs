This file contains context about the project for the Gemini AI model.
This document defines the GEMINI rules for our software stack. These guidelines establish a shared vocabulary and enforce consistency across our services, APIs, and codebases.

Table of Contents

Purpose & Scope

GEMINI Principles

G — Granular Services

E — Explicit Contracts

M — Modular Design

I — Idempotency & Immutability

N — Naming Conventions

I — Integration & Testing

Rule Definitions

Enforcement & Compliance

Versioning & Updates

Purpose & Scope

The GEMINI rules provide a lightweight framework to:

Ensure clear boundaries between services

Standardize API contracts and data exchange

Promote modular, reusable code

Guarantee safe operations through idempotent patterns

Apply consistent naming across code and infrastructure

Embed integration testing early in the development lifecycle

These guidelines apply to all backend services (e.g., Rails, Django, Flask), frontend codebases, and infra-as-code in our organization.

GEMINI Principles

G — Granular Services

Single Responsibility: Each service should own a single business domain.

API Boundary: Communicate only through well-defined REST/gRPC interfaces.

Failure Isolation: Design for independent deploys and graceful degradation.

E — Explicit Contracts

Schema First: Define request/response schemas using OpenAPI or Protocol Buffers.

Versioned APIs: Use semantic versioning for all public endpoints.

Backward Compatibility: Maintain two versions concurrently during major upgrades.

M — Modular Design

Component-Based: Break functionality into modules or gems/packages.

Loose Coupling: Depend on interfaces, not implementations.

Clear Imports: Explicitly list external dependencies.

I — Idempotency & Immutability

Idempotent Endpoints: Ensure safe retries without side effects (e.g., idempotency keys).

Immutable Data: Write-once records; use version stamps for updates.

Event Sourcing: Prefer event logs for auditability where applicable.

N — Naming Conventions

Consistent Patterns: snake_case for Ruby/Python, camelCase for JS/TS.

Resource Naming: REST endpoints should use plural nouns (/users, /orders).

Semantic Branches: Git branches prefixed by type (e.g., feat/, fix/, chore/).

I — Integration & Testing

Contract Tests: Consumer-driven contract tests for cross-service calls.

CI/CD Gate: Mandatory integration tests before merge.

Monitoring Hooks: Health-check endpoints and automated alerting.

Rule Definitions

Service Boundaries: No direct database access across services. Always use APIs.

API Documentation: Every public endpoint must have an OpenAPI spec.

Dependency Control: Use dependency audit tools; avoid transitive high-risk libs.

Idempotency Keys: Generate and log keys for state-changing operations.

Error Handling: Return structured error objects with codes and messages.

Branch Policies: Enforce pull-request reviews and status checks.

Enforcement & Compliance

Automated Linters: Run RuboCop, ESLint, and API schema validators in CI.

Contract Test Suite: Shared test repo for all service contracts.

Periodic Audits: Quarterly architecture and dependency reviews.

Versioning & Updates

This document is versioned under semantic versioning. Changes must be:

Proposed via pull request against the docs/ directory.

Reviewed by at least two architects.

Released alongside corresponding CI/CD pipeline updates.
