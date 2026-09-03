# maintainer's guide

## releases

To trigger a release, create a release through the "draft releases" page of github.

Releases can happen independently per language, as long as the name of the tag matches `{language}-v*` where language is currently one of `python` and `js`. This will upload a version to the language-specific package index (PyPI for `python` and npmjs.org for `js`). npmjs.org packages are uploaded to the staging area and need to be manually confirmed through the website.

Be aware that tags can intentionally not be deleted (mutable tags are bad for supply-chain security), so be careful when creating one: there is no way to undo this.

After confirming a python release was successfully uploaded to PyPI, activate a version build on ReadTheDocs (click on "add version" in the versions tab) and rename the slug from `python-v{version}` to just `v{version}`. If anything went wrong with the build, it is fine to push another tag called `python-docs-v{version}` (or anything, really, it should just be different from the release pattern above). Be aware that in that case you may have to choose a different slug such as `v{version}post0`.

Usually a version update will be triggered on conda-forge within a day, but if not (or you need to be faster than that), use a bot command issue to trigger a version update. You'll find a list of bot commands by choosing the "bot commands" issue template. Also be aware that "when in doubt, rerender" (through another bot command) can resolve a lot of conda-forge build issues.
