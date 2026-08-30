# Specful documentation site

This is the Astro/Starlight source for <https://unkos-dev.github.io/specful/>. The published pages, the CLI reference,
and the ADR reference examples in `src/pages/reference/examples/` are generated from this project and from the live
artifacts under `../docs/adr/` at build time.

See `../plans/2026-08-30-doc-site-plan.md` for the delivery plan and content ownership boundary.

## Layout

- `src/content/docs/`: authored pages (adoption, authoring workflow, profiles, CLI reference).
- `src/pages/reference/examples/`: pages rendered at build time from `../docs/adr/`.
- `src/styles/`: vendored `tokens.css` and the `theme.css` overlay that maps `--sf-*` tokens onto Starlight's theme
  variables.
- `src/branding/NOTICE.md`: source, licence, and attribution for the vendored brand material.

## Commands

Run from this directory, through the repository's mise-pinned bun:

| Command | Action |
|---|---|
| `mise exec -- bun install` | Install dependencies from the committed lockfile. |
| `mise exec -- bun run dev` | Start the local authoring preview at `localhost:4321`. |
| `mise exec -- bun run build` | Build the production site to `./dist/`. |
| `mise exec -- bun run preview` | Preview the production build locally. |
