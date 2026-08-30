# Vendored Specful brand material

This directory and `../styles/tokens.css` vendor material from the `specful-branding` repository, at commit
`ee33678168e75355e6a2f4ab30421f60e63ef9d0`.

## Source and licence

- `../styles/tokens.css` is copied verbatim from `tokens.css` in `specful-branding`, licensed under Creative Commons
  Attribution 4.0 International (CC BY 4.0): <https://creativecommons.org/licenses/by/4.0/legalcode>.
- `src/assets/branding/specful-lockup-colour.svg` and `specful-lockup-colour-reversed.svg` (header logo, light and
  dark), `public/favicon.svg` (from `assets/github/specful-icon-mark.svg`), and `public/social-preview-light.png` /
  `social-preview-dark.png` are copied verbatim from `assets/github/` and `assets/lockup/` in `specful-branding`. These
  are reserved-rights brand assets: they identify the Specful project and may be used to refer to or link to Specful,
  but not to identify another product or service, or to suggest sponsorship or endorsement. See `specful-branding`'s own
  `LICENSE` file for the complete terms.

This material is distinct from this repository's own Apache-2.0 (implementation) and CC0-1.0 (templates, schemas,
examples) grants; it does not fall under either.

## Adaptations

`tokens.css` is vendored unmodified. The mapping from `--sf-*` tokens onto Starlight's `--sl-*` theme variables lives in
`../styles/theme.css`, a separate file, so the vendored source stays byte-identical to upstream and every adaptation is
visible as a diff against this notice rather than an edit to the vendored file itself.
