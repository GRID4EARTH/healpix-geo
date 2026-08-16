# Development workflow

This document describes the lightweight development workflow used by `healpix-geo`. It is
intended to support both regular contributions and coordinated work across GRID4EARTH
repositories without introducing a complex branching model.

GRID4EARTH development is primarily supported by ESA-funded activities, and external
contributions are welcome.

## Branches

- `main` is stable and release-ready.
- `dev` is a shared integration branch for unreleased work needed by other GRID4EARTH
  repositories. It is used when cross-repository integration is needed, rather than for every
  change.
- Feature branches and pull requests contain individual developments. They should be focused
  and linked to the relevant issue where possible.
- Releases are tagged from `main`.

## Cross-repository development

Some changes need to be tested with `healpix-analyse` or another GRID4EARTH repository before
they are ready for a `healpix-geo` release. In that situation:

1. Develop the change on a feature branch and submit it through the normal pull request
   process.
2. Integrate the unreleased change into `dev` when a shared branch is needed for downstream
   testing.
3. Point the downstream development branch to `healpix-geo`'s `dev` branch temporarily and
   document that unreleased dependency in the downstream pull request.
4. Merge release-ready changes into `main` through the normal review process.
5. Tag the release from `main`, then update downstream repositories to use the released version
   instead of the branch dependency.

The `dev` branch is an integration point, not a replacement for focused feature branches,
review, or releases.

## Authorship and credit

Contributions should retain accurate authorship through commits and pull requests. Project
documentation and release records should acknowledge contributors where appropriate.

The creators used for Zenodo release records are maintained in [`.zenodo.json`](.zenodo.json).
Before tagging a release, review this metadata against the Git history, including
`Co-authored-by` trailers. Add ORCID identifiers only after verifying them against a source
controlled by the contributor. The release tag supplies the version to Zenodo, so the version is
not stored in `.zenodo.json`.
