<!-- The header is mandatory and must conform to the Commit Message Header format.
<type>(<scope>): <short summary>
  │       │             │
  │       │             └─⫸ Summary in present tense. Not capitalized. No period at the end.
  │       │
  │       └─⫸ Commit Scope: deps|templates|github-actions|displays|speakers
  │
  └─⫸ Commit Type: build|ci|docs|feat|fix|perf|refactor|test

The <summary> field is mandatory, the (<scope>) field is optional. -->
build(scope): lorem ipsum

<!-- The body is mandatory for all commits except for those of type "docs". When the body is present it must be at least 20 characters long and must conform to the Commit Message Body format.
Just as in the summary, use the imperative, present tense: "fix" not "fixed" nor "fixes".

Explain the motivation for the change in the commit message body. This commit message should explain why you are making the change. You can include a comparison of the previous behavior with the new behavior in order to illustrate the impact of the change.-->
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris fermentum eget neque dignissim feugiat. Suspendisse potenti. Proin nulla ipsum, interdum ullamcorper augue sed, sodales dictum sem. Integer quis mauris sit amet leo condimentum fermentum quis et sapien. 

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
BREAKING CHANGE: lorem ipsum

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris fermentum eget neque dignissim feugiat. Suspendisse potenti. Proin nulla ipsum, interdum ullamcorper augue sed, sodales dictum sem. Integer quis mauris sit amet leo condimentum fermentum quis et sapien. 

Fixes #42

<!-- Acknowledgment : https://github.com/angular/angular/blob/16fa9839890f9862bbe86e465add0e2a99c214e9/contributing-docs/commit-message-guidelines.md -->