# Development workflow

This document describes the lightweight development workflow used by `healpix-geo`. It is
intended to support both regular contributions and coordinated work across GRID4EARTH
repositories without introducing a complex branching model.

GRID4EARTH development is primarily supported by ESA-funded activities, and external
contributions are welcome.

## Branches

- `main` is stable and release-ready.
- `integration` is a shared branch for combining and validating unreleased changes before they
  are promoted to `main`. It is used primarily for integration and cross-repository validation,
  not as the default branch for day-to-day feature development.
- Feature branches and pull requests contain individual developments. They should be focused
  and linked to the relevant issue where possible. Start them from `main` by default.
- Releases are tagged from `main`.

The usual path remains a focused feature pull request to `main`. Use `integration` only when a
change or compatible group of changes needs combined or downstream validation before promotion:

```text
feature branch ────────────────────────────────> PR to main ──> main ──> release tag
       \
        └─ when integration validation is needed ─> integration
                                                       ├─> cross-feature validation
                                                       ├─> cross-repository validation
                                                       └─> PR: integration -> main
```

## Cross-repository development

Some changes need to be tested with `healpix-analyse` or another GRID4EARTH repository before
they are ready for a `healpix-geo` release. In that situation:

1. Develop the change on a feature branch and submit it through the normal pull request
   process.
2. When the unreleased change is needed for development or testing in at least one other
   repository in the GRID4EARTH organization, identify the downstream issue or pull request that
   requires it. A maintainer may integrate the change into `integration` as soon as its feature
   branch or pull request passes the relevant checks.
3. Point the downstream development branch to `healpix-geo`'s `integration` branch temporarily
   and document that unreleased dependency in the downstream pull request.
4. After the combined and downstream checks pass, promote the compatible, release-ready changes
   to `main` through an `integration`-to-`main` pull request and the normal review process. Keep
   unrelated or incomplete changes out of that promotion.
5. Tag the release from `main`, then update downstream repositories to use the released version
   instead of the branch dependency.

The `integration` branch is a validation and promotion point. It is not a replacement for
focused feature branches, pull-request review, or releases, and feature branches should not use
it as their base unless the feature specifically depends on unreleased integrated work.

## Authorship and credit

Contributions should retain accurate authorship through commits and pull requests. Project
documentation and release records should acknowledge contributors where appropriate.

The creators used for Zenodo release records are maintained in [`.zenodo.json`](.zenodo.json).
Before tagging a release, review this metadata against the Git history, including
`Co-authored-by` trailers. Add ORCID identifiers only after verifying them against a source
controlled by the contributor. The release tag supplies the version to Zenodo, so the version is
not stored in `.zenodo.json`.

Core developers are listed first in the agreed contribution order. Other creators are listed
alphabetically by family name. Changes to the core developer list or creator order require
maintainer agreement; do not reorder the list mechanically by commit count or lines changed.

Creator status reflects either ongoing core-development responsibility or a substantial
contribution to the software's design, implementation, or documentation. More limited operational
or maintenance contributions should still be credited in the Zenodo `contributors` list with an
appropriate role.
