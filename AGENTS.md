<!-- GSD:project-start source:PROJECT.md -->
## Project

**Claw Proxy**

Claw Proxy is a cross-platform developer tool that unifies multiple AI providers behind a single local proxy server. It exposes both OpenAI-compatible and Anthropic-compatible APIs so tools like Claude Code can route through multiple providers and accounts without changing their normal workflow.

The product is CLI-first, with a Tauri menu bar or system tray companion app for visibility and control. The core engine must remain independently runnable so the desktop UI is an optional control surface, not a dependency.

**Core Value:** Developers can point their existing AI tooling at one local endpoint and transparently get reliable multi-provider, multi-account routing without changing how they work.

### Constraints

- **Tech stack**: Rust core with Axum/Tokio/Reqwest plus Tauri 2.x, React 18, TypeScript, TailwindCSS, shadcn/ui, and Zustand — the approved architecture is already chosen.
- **Architecture**: Core engine and UI must be fully separated — the proxy must run without the desktop app, and the UI must not own business logic.
- **Compatibility**: MVP must work on macOS and Windows — tray behavior and sidecar control need to respect both environments.
- **API surface**: `/v1/chat/completions` and `/v1/messages` must both be first-class entrypoints — Claude Code support depends on the Anthropic-compatible path.
- **Security**: API keys live in plaintext YAML for MVP — acceptable for now, but this is a known limitation to isolate and later replace.
- **Extensibility**: The router cannot contain provider-specific behavior — all provider variance belongs in adapters implementing the shared trait.
<!-- GSD:project-end -->

<!-- GSD:stack-start source:STACK.md -->
## Technology Stack

Technology stack not yet documented. Will populate after codebase mapping or first phase.
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

Conventions not yet established. Will populate as patterns emerge during development.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

Architecture not yet mapped. Follow existing patterns found in the codebase.
<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->
## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, or `.github/skills/` with a `SKILL.md` index file.
<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
