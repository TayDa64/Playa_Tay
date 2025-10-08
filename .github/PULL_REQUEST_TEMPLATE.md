<!--
Before submitting a PR, please read https://github.com/tauri-apps/tauri/blob/dev/.github/CONTRIBUTING.md#pull-request-guidelines

1. Give the PR a descriptive title.

  Examples of good title:
    - fix(windows): fix race condition in event loop
    - docs: update example for `App::show`
    - feat: add `Window::set_fullscreen`

  Examples of bad title:
    - fix #7123
    - update docs
    - fix bugs

2. If there is a related issue, reference it in the PR text, e.g. closes #123.
3. If this change requires a new version, then add a change file in `.changes` directory with the appropriate bump, see https://github.com/tauri-apps/tauri/blob/dev/.changes/README.md
4. Ensure that all your commits are signed https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits
5. Ensure `cargo test` and `cargo clippy` passes.
6. Propose your changes as a draft PR if your work is still in progress.
-->

## Linked Issue
<!-- 
Required: Reference the issue this PR addresses. Use "Closes #123" or "Fixes #456" 
If implementing a task from Phase tickets, reference the corresponding issue number.
-->

Closes #

## Model Used
<!-- 
Optional but recommended for YOLO-labeled PRs: Specify the AI model used for this work.
Examples: openai/gpt-4.1, openai/o3, anthropic/claude-3.5-sonnet, github/copilot
Leave blank if not applicable.
-->

Model: 

## Test Plan
<!--
Describe how you tested this change. Include:
- Unit tests added/modified
- Integration tests run
- Manual testing steps
- Any edge cases verified

For backend services: `cargo test -p <crate>`, manual API testing
For frontend: UI screenshots, browser testing
For CI/workflows: workflow run links
-->

## Checklist
- [ ] Code follows repository conventions (see `.github/copilot-instructions.md`)
- [ ] Tests added/updated and passing (`cargo test`, `pnpm test`)
- [ ] Linting passes (`cargo clippy`, `pnpm lint`)
- [ ] Documentation updated (if applicable)
- [ ] Commits are signed
- [ ] PR is labeled appropriately (add `yolo` if using YOLO Supervisor)

<!--
For Phase 2.4–2.6 implementations:
- [ ] Followed Tauri-first architecture
- [ ] Used Rust for backend services (no Electron runtime dependencies)
- [ ] Avoided `any` in TypeScript (use `unknown` + type guards)
- [ ] Did not use `sleep` in terminal commands
-->
