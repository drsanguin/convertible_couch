<!--
Commit Message Header :

The header is mandatory and must conform to the Commit Message Header format.
<type>(<scope>): <short summary>
  │       │             │
  │       │             └─⫸ Summary in present tense. Not capitalized. No period at the end.
  │       │
  │       └─⫸ Commit Scope: deps|templates|github-actions|displays_settings|speakers_settings
  │
  └─⫸ Commit Type: build|ci|docs|feat|fix|perf|refactor|test

The <type> and <summary> fields are mandatory, the (<scope>) field is optional.

Type :
Must be one of the following:
| Type     | Description                                                                                        |
| -------- | -------------------------------------------------------------------------------------------------- |
| build    | Changes that affect the build system or external dependencies (example scopes: deps, build-script) |
| ci       | Changes to our CI configuration files and scripts (examples: Github Actions, Dependabot)           |
| docs     | Documentation only changes                                                                         |
| feat     | A new feature                                                                                      |
| fix      | A bug fix                                                                                          |
| perf     | A code change that improves performance                                                            |
| refactor | A code change that neither fixes a bug nor adds a feature                                          |
| test     | Adding missing tests or correcting existing tests                                                  |

Scope :
The scope should be the name of the module affected (as perceived by the person reading the changelog generated from commit messages).

The following is the list of supported scopes:

There are currently a few exceptions to the "use module name" rule:
  - changelog: used for updating the release notes in CHANGELOG.md
  - none/empty string: useful for test and refactor changes that are done across all packages (e.g. test: add missing unit tests) and for docs changes that are not related to a specific package (e.g. docs: fix typo in template).

Summary :
Use the summary field to provide a succinct description of the change:
  - use the imperative, present tense: "change" not "changed" nor "changes"
  - don't capitalize the first letter
  - no dot (.) at the end
-->
build(lorem-ipsum): lorem ipsum

<!--
Commit Message Body :

The body is mandatory for all commits except for those of type "docs". When the body is present it must be at least 20 characters long and must conform to the Commit Message Body format.
Just as in the summary, use the imperative, present tense: "fix" not "fixed" nor "fixes".

Explain the motivation for the change in the commit message body. This commit message should explain why you are making the change. You can include a comparison of the previous behavior with the new behavior in order to illustrate the impact of the change.

Just as in the summary, use the imperative, present tense: "fix" not "fixed" nor "fixes".

Explain the motivation for the change in the commit message body. This commit message should explain why you are making the change. You can include a comparison of the previous behavior with the new behavior in order to illustrate the impact of the change.
-->
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin sed justo sodales, placerat sem non, elementum ligula. Praesent est quam, faucibus eget mauris vitae, sollicitudin placerat sem. Donec dui lacus, efficitur a vehicula vitae, interdum ut est. Aenean vitae nibh sed ligula eleifend bibendum sit amet eu nisl. Quisque cursus diam at justo porta convallis. Morbi eu mauris at sapien convallis lacinia pharetra et lacus. Nunc nec mauris lacinia lorem posuere posuere. Phasellus molestie aliquet lectus, id rhoncus urna blandit sit amet. Nulla scelerisque dolor nec arcu volutpat, ut ornare lacus consequat. Morbi tincidunt arcu vel ex fermentum tristique. Donec id metus a metus blandit efficitur eu eget nulla. Sed nec mi tincidunt, maximus justo vitae, egestas ante.

<!--
Commit Message Footer :

The footer is optional. The Commit Message Footer format describes what the footer is used for and the structure it must have.
The footer can contain information about breaking changes and deprecations and is also the place to reference GitHub issues and other PRs that this commit closes or is related to.

For example:
BREAKING CHANGE: <breaking change summary>
<BLANK LINE>
<breaking change description + migration instructions>
<BLANK LINE>
<BLANK LINE>
Fixes #<issue number> 

or

DEPRECATED: <what is deprecated>
<BLANK LINE>
<deprecation description + recommended update path>
<BLANK LINE>
<BLANK LINE>
Closes #<pr number>

Breaking Change section should start with the phrase BREAKING CHANGE: followed by a brief summary of the breaking change, a blank line, and a detailed description of the breaking change that also includes migration instructions.

Similarly, a Deprecation section should start with DEPRECATED: followed by a short description of what is deprecated, a blank line, and a detailed description of the deprecation that also mentions the recommended update path.
-->
BREAKING CHANGE: lorem ipsum

Aenean tempor lorem nulla, id pharetra diam lobortis nec. Nullam tellus nulla, gravida sed malesuada vel, condimentum at ipsum. Proin vel ipsum sed mi egestas finibus. Donec sit amet odio interdum, tempor ex a, mattis sapien. Sed pulvinar consectetur faucibus. Integer gravida enim nisi. Nullam fermentum est eu aliquam hendrerit. Suspendisse diam odio, elementum ut malesuada vel, molestie id dolor. Quisque et nisl rutrum, lobortis dui id, tempus eros. Nunc luctus nisi ante, ut semper augue aliquet eget. Curabitur hendrerit mattis nisl, non rhoncus libero pellentesque vel. Praesent et tellus ipsum. Phasellus tempor tincidunt lacinia.


Fixes #42

<!--
Revert commits

If the commit reverts a previous commit, it should begin with revert: , followed by the header of the reverted commit.

The content of the commit message body should contain:

    information about the SHA of the commit being reverted in the following format: This reverts commit <SHA>,
    a clear description of the reason for reverting the commit message.
-->

<!-- Acknowledgment : https://github.com/angular/angular/blob/16fa9839890f9862bbe86e465add0e2a99c214e9/contributing-docs/commit-message-guidelines.md -->