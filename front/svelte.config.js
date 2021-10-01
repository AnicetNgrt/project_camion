import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';
import { resolve } from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: preprocess(),

	kit: {
		adapter: adapter({
			// default options are shown
			pages: 'build',
			assets: 'build',
			fallback: null
		}),
		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte',
		vite: {
            resolve: {
                alias: {
                    $components: resolve('./src/components'),
                    $logic: resolve('./src/logic'),
                    $styles: resolve('./src/styles'),
                    $types: resolve('./src/types.ts'),
                }
            }
        }
	}
};

export default config;
