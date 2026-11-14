# Contributing to healpix-geo

Thank you for your interest in contributing. GRID4EARTH development is primarily supported by
ESA-funded activities, and external contributions are welcome.

## How to contribute

- Report bugs or request features through
  [GitHub Issues](https://github.com/GRID4EARTH/healpix-geo/issues).
- For non-trivial changes, open an
  [issue](https://github.com/GRID4EARTH/healpix-geo/issues/new) before starting work so the
  approach can be discussed with the maintainers.
- Fork the repository and create a feature branch for each change.
- Add or update tests and documentation where appropriate.
- Open a pull request with a clear title and description, and link the related issue.

See [`DEVELOPMENT.md`](DEVELOPMENT.md) for the branch model and the workflow for changes that
span multiple GRID4EARTH repositories.

## Pull request guidelines

- Keep commits focused and descriptive.
- Run the relevant tests and formatting checks before submitting.
- Respond to review feedback and keep the pull request focused on its stated purpose.

## Continuous Integration

To save compute time, CI is triggered only when the relevant language was impacted by a PR. More concretely:

- rust CI will only run when `$project_root/Cargo.*` or `$project_root/healpix-geo/*` was modified
- python CI will run when the rust CI runs, and additionally when `$project_root/healpix-geo-python/*` was modified
- ReadTheDocs runs will be skipped except on `main` or on tags, or when the python CI runs
- js CI will run when the rust CI runs, and additionally when `$project_root/healpix-geo-js/*` was modified

This behavior can be changed by adding labels or commit message tags (text enclosed in brackets in the first line of the commit message):

- adding a `[run-rtd]` commit message tag will always run RTD on that commit
- adding a `[skip-rtd]` commit message tag will skip RTD on that commit (`[run-rtd]` is prioritized)
- adding a `[skip-ci]` commit message will skip all CI on that commit
- adding a `skip-ci` label to the PR will skip all github CI for the entire PR (maintainers only)

## Code of Conduct

All contributors should respect the
[Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).

## License

The project is licensed under the [Apache License 2.0](LICENSE). By contributing, you agree that
your contributions will be licensed under the same terms.
