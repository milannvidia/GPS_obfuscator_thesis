import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
export default {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			pages: '../docs',
			assets: '../docs',
			fallback:'index.html',
			precompress: false,
			strict: true
		}),
		paths: {
            base: process.env.NODE_ENV === 'production' ? '/GPS_obfuscator_thesis' : '',
        }
	}
};

