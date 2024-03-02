# Contributing
All contributions that help improve Xosms are welcome but here are a few things you should know.

## Conventional Commits
Xosms is making use of conventional commits, please it these when making commits.

## Adding, Modifying, or Removing APIs
All platforms must have a stable and consistent JavaScript facing API. Currently Xosms cannot use traits to help enforce these rules and care needs to be taken that all platforms, even unsupported, are consistent with each other. 

### Tests
Tests are provided to ensure these APIs exist and to help guide how the API should function in given scenarios.

Tests should be created, modified, or removed if APIs are updated in any fashion.