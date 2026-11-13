# CKB IDL Ecosystem Release Plan

**Status:** Confirmed by maintainer OWK50GA following review.

This is the adopted M0 ecosystem release plan. Component versions and calendar
dates remain unassigned where explicitly stated below.

## Purpose and scope

This plan coordinates delivery across the CKB IDL specification, Rust crates,
clients, CLI, Script Registry, examples, and planned Molecule tooling. Its home
in `script-registry` does not make every deliverable a registry release.

[Project proposal](https://talk.nervos.org/t/dis-ckb-idl-interface-description-language-and-discoverability-for-ckb-scripts/11031)
| [Public delivery board](https://github.com/orgs/ckb-idl/projects/1)
| [Existing tooling and release walkthrough](https://talk.nervos.org/t/ls-idl-a-lock-script-interface-description-language-for-ckb-derive-validate-commit/10596)

Milestones measure ecosystem outcomes. Package versions, repository tags,
service deployments, and milestone completion are recorded separately.

## Existing baseline

The Rust toolchain already has published `0.1.0` releases. They are inputs to
this delivery, not new M0 deliverables.

| Component | Existing baseline | Repository |
| --- | --- | --- |
| IDL specification | Documented IDL 0.1.0 model, canonicalization, encoding, commitments, and vectors | [ckb-idl-spec](https://github.com/ckb-idl/ckb-idl-spec) |
| Rust derive, runtime types, and exporter | Published `ckb-idl-derive`, `ckb-idl-types`, and `ckb-idl-export` crates at 0.1.0 | [ckb-idl-derive](https://github.com/ckb-idl/ckb-idl-derive) |
| Rust client | Published `ckb-idl-client` 0.1.0 | [ckb-idl-client](https://github.com/ckb-idl/ckb-idl-client) |
| Binding CLI | Published `ckb-idl-cli` 0.1.0 | [ckb-idl-cli](https://github.com/ckb-idl/ckb-idl-cli) |
| Reference scripts and examples | Existing implementations and walkthroughs; record exact revisions for release demonstrations | [ckb_sudt_script](https://github.com/ckb-idl/ckb_sudt_script), [idl-examples](https://github.com/ckb-idl/idl-examples) |
| TypeScript tooling | Existing implementation; compatibility upgrade remains planned | [ckb-idl-derive-ts](https://github.com/ckb-idl/ckb-idl-derive-ts) |
| Script Registry | Development scaffold; manifest version 0.1.0 is not a published registry release | [script-registry](https://github.com/ckb-idl/script-registry) |
| Molecule-to-IDL tooling | Planned; repository and package names are not selected | To be established |

The released crates can be verified on crates.io:
[derive](https://crates.io/crates/ckb-idl-derive),
[types](https://crates.io/crates/ckb-idl-types),
[export](https://crates.io/crates/ckb-idl-export),
[client](https://crates.io/crates/ckb-idl-client), and
[CLI](https://crates.io/crates/ckb-idl-cli).

## Milestone delivery checkpoints

Target windows are relative to initial funding and the confirmed start date.
No calendar deadline or new package version is assigned by this plan.

| Milestone | Target window | Delivery outcome | Affected components |
| --- | --- | --- | --- |
| M0 — Foundation | Week 0 | Public roadmap/board, confirmed plan, registry scaffold, CI/local Docker, contribution/security documentation | Registry and ecosystem coordination |
| M1 — Publication core | Weeks 1–3 | Authenticated namespace/package publication, immutable artifacts and releases, and public reads; scripts without IDL can participate | Registry |
| M2 — Testnet verification | Weeks 4–7 | Deployment, confirmation, and liveness evidence; interface binding checks and legacy attestations; a demonstrable CLI/client workflow | Registry, Rust client, CLI, examples |
| M3 — Discovery and interface | Weeks 8–11 | Public TypeScript UI, exact/full-text search, explicit evidence states, integration snippets, and bounded candidate discovery/revalidation | Registry API/worker/frontend and client integrations |
| M4 — Interoperability and hardening | Weeks 12–15 | IDL 0.1.1 empty interfaces, supported Molecule generation, TypeScript compatibility, cross-toolchain examples, and documentation | Specification, derive/types/export, clients, CLI, Molecule tooling, registry, examples |

Every milestone closes with public implementation/documentation links, passing
checks, reproducible demonstrations where applicable, and a published progress
report. Completing a milestone does not require republishing unchanged crates.

## Component versions and compatibility

- Keep the IDL specification version separate from Rust crate versions, npm
  package versions, registry API versions, and service deployment identifiers.
- IDL 0.1.1 is the planned specification target. It does not automatically set
  every implementing crate or package to version 0.1.1.
- Select each next package version after reviewing its actual changes and
  compatibility impact. Record required companion-crate versions and supported
  IDL/encoding profiles before publication.
- Preserve published artifacts and their documented behavior. Make any breaking
  change explicit, including migration guidance and unsupported-version errors.
- Test continued consumption of the released IDL 0.1.0 baseline where support is
  claimed. Test empty 0.1.1 interfaces separately from missing IDL artifacts.
- Publish changed packages in dependency order derived from their manifests.
  Do not make releases depend on unpublished local paths or floating revisions.

Before a coordinated release, add a tested compatibility record:

| Component | Exact version/tag or commit | Supported IDL/profile versions | Required companion versions | Validation evidence |
| --- | --- | --- | --- | --- |
| One row per changed component and required consumer | To be selected | To be tested | To be recorded | CI, vectors, and demonstration links |

A single shared version number is not required. Do not claim a combination is
compatible until its evidence is recorded.

## Cross-repository sequencing

1. M1 builds on the existing toolchain and supports publication without IDL.
2. M2 connects publication to chain evidence and existing commitment verification.
3. M3 consumes those APIs and evidence states for discovery and integration.
4. M4 coordinates specification and implementation changes: define the 0.1.1
   rules/vectors, update producers and consumers, then demonstrate interoperability.

These are delivery dependencies, not a ban on parallel work. Specification and
compatibility design can begin before M4; completed M1–M3 work should not depend
on an unreleased implementation being presented as available.

The Molecule generator's repository, package boundary, supported subset, and
initial release version must be selected before its implementation release.
Its output must preserve the supported Molecule wire format. The final layout
of the TypeScript client/tooling should likewise be recorded before packaging.

## Issue ownership and tracking

The organization project is the shared roadmap across repositories. Repository
milestones and issue numbers are local tracking aids, not global release IDs.

- Keep implementation issues in the repository that owns the change. Transfer
  provisional registry issues when their destination repositories are ready.
- For work spanning repositories, retain an umbrella issue and link the component
  issues/PRs. Split compound work rather than moving the same task repeatedly.
- Preserve project membership and milestone context when transferring issues;
  verify links and update dependencies after transfer.
- Track planned repositories provisionally without pretending they already exist.

The current registry issues are the starting breakdown, not a requirement to
implement the entire ecosystem inside `script-registry`.

## Branch promotion and release procedure

```text
feature/* or fix/* -> dev -> staging -> master
```

For this registry, implement changes on separate feature/fix branches based on
`dev`. Review and merge them into `dev`; promote a batch into `staging` for
combined validation, then into `master`. Do not write features directly on `dev`.
The maintainer has authorized the remaining M0 foundation/documentation updates
to be committed directly to `dev`; new feature and fix work uses separate branches.
Apply the equivalent reviewed promotion process in participating repositories,
recording any repository-specific branch conventions.

For each release batch:

1. Identify included issues, changed repositories/packages, exact candidate
   revisions, dependency order, and acceptance evidence.
2. Validate the batch on staging: formatting/build/tests, relevant conformance
   vectors, and affected integration or testnet workflows. Registry CI currently
   targets `dev` and `master`; staging validation must be performed explicitly
   until staging triggers are configured.
3. Review configuration, database/storage migrations, backup/recovery steps,
   and rollback limitations for any service deployment. Mark these not applicable
   when a release does not change them.
4. Promote the reviewed candidate to master, verify the resulting release commit,
   and tag/publish only the components being released. Record the exact commits
   and package artifacts, not merely a branch name.
5. Smoke-test published artifacts and deployments. Publish release notes with
   compatibility, known limitations, and migration or recovery instructions.
6. Update the board and milestone report with the resulting release evidence.

M0 is primarily a foundation checkpoint; it does not require inventing a new
release of already-published crates or publishing the registry binary to crates.io.

## Acceptance and scope boundaries

A milestone or component is complete only when its own acceptance criteria are
met. A published package alone does not prove the ecosystem workflow works.
Use shared canonicalization/commitment/encoding vectors and reference-script
workflows to demonstrate claimed interoperability.

Deployment evidence, IDL binding, and publisher attestation remain distinct.
Clients retain independent verification. Registry publication without an IDL
must remain possible throughout the rollout.

This delivery is testnet-first. Mainnet operation, formal third-party audits,
arbitrary source builds or uploaded-code execution, on-chain registry contracts,
and general package dependency resolution are outside its initial scope.

## Confirmation and maintenance

The maintainer has reviewed and approved the baseline, component ownership,
milestone outcomes, compatibility policy, and promotion procedure in this plan.
The publication commit identifies the adopted revision and can be linked from
the M0 release-plan issue. Dates and component versions remain explicitly
unassigned where the inputs are not yet known; select them before the relevant
release is executed.

Update this document when scope, ownership, compatibility, sequencing, or release
commitments change. Record actual releases as they occur and link the corresponding
milestone reports. Existing releases must remain visible as the baseline rather
than being relabelled as newly completed grant work.
