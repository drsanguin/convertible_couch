---
name: "CI"
about: "Changes to our CI configuration files and scripts (examples: Github Actions, Dependabot)"
title: "ci(<scope>): <subject>"
labels: "build"
assignees: drsanguin

---
<!-- The header is mandatory and must conform to the Commit Message Header format.
build(<scope>): <short summary>
          │             │
          │             └─⫸ Summary in present tense. Not capitalized. No period at the end.
          │
          └─⫸ Commit Scope: deps|displays|speakers|common

The <summary> field is mandatory, the (<scope>) field is optional. -->
build(<scope>): <short summary>

<!-- The body is mandatory for all commits except for those of type "docs". When the body is present it must be at least 20 characters long and must conform to the Commit Message Body format.
Just as in the summary, use the imperative, present tense: "fix" not "fixed" nor "fixes".

Explain the motivation for the change in the commit message body. This commit message should explain why you are making the change. You can include a comparison of the previous behavior with the new behavior in order to illustrate the impact of the change.-->
<body>

<!-- The footer is optional. The Commit Message Footer format describes what the footer is used for and the structure it must have.
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
Closes #<pr number>-->
<footer>

<!-- Acknowledgment : https://github.com/angular/angular/blob/16fa9839890f9862bbe86e465add0e2a99c214e9/contributing-docs/commit-message-guidelines.md -->