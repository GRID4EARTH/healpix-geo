# Contributing to healpix-geo

Thank you for your interest in contributing! This project is licensed under the [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0), and all contributions are under this license.

## How to Contribute

- Report bugs or request features via [GitHub Issues](https://github.com/GRID4EARTH/healpix-geo/issues).
- For all non-trivial contributions, please start a discussion with the maintainers by opening an [issue](https://github.com/GRID4EARTH/healpix-geo/issues/new) to ensure alignment with project goals and future plans.

### Discussion Before Submitting

- **Mandatory Pre-Submission Discussion**: For all non-trivial contributions, open an issue or start a discussion with maintainers about your proposed changes. This allows maintainers to provide feedback, suggest directions, and ensure your efforts align with the project's roadmap.
- **Collaboration**: Engage constructively with maintainers during this phase to refine and improve your proposal.
- Fork the repository and create feature branches for your changes.
- Add tests and documentation for your contributions.
- Submit pull requests with clear titles and descriptions, referencing related issues.

## Pull Request Guidelines

- Keep commits focused and descriptive.
- Ensure all tests pass and maintain code style.
- Maintain open communication with maintainers during the review process.
- By contributing, you agree to license your changes under Apache 2.0.

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

All contributors should respect the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).
