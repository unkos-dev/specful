// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import sitemap from '@astrojs/sitemap';

const site = 'https://unkos-dev.github.io';
const base = '/specful';

// https://astro.build/config
export default defineConfig({
  site,
  base,
  integrations: [
    starlight({
      title: 'Specful',
      description: 'Repository-native requirements, design, and decisions for people and coding agents.',
      logo: {
        light: './src/assets/branding/specful-lockup-colour.svg',
        dark: './src/assets/branding/specful-lockup-colour-reversed.svg',
        replacesTitle: true,
      },
      customCss: ['./src/styles/theme.css'],
      social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/unkos-dev/specful' }],
      editLink: {
        baseUrl: 'https://github.com/unkos-dev/specful/edit/main/site/',
      },
      head: [
        {
          tag: 'meta',
          attrs: { property: 'og:image', content: `${site}${base}/social-preview-light.png` },
        },
        {
          tag: 'meta',
          attrs: { name: 'twitter:card', content: 'summary_large_image' },
        },
      ],
      sidebar: [
        { label: 'Adoption', slug: 'adoption' },
        { label: 'Authoring workflow', slug: 'authoring-workflow' },
        {
          label: 'Profiles',
          items: [
            { label: 'Requirement', slug: 'profiles/requirement' },
            { label: 'Design', slug: 'profiles/design' },
            { label: 'ADR', slug: 'profiles/adr' },
          ],
        },
        {
          label: 'Reference',
          items: [
            { label: 'CLI', slug: 'reference/cli' },
            { label: 'Validation integration', slug: 'reference/validation-integration' },
          ],
        },
      ],
    }),
    sitemap(),
  ],
});
