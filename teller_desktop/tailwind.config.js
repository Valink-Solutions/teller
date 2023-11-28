import colors from 'tailwindcss/colors';

const accent = {
	100: '#f3e8d5',
	300: '#f2d5ae',
	500: '#d6af7b',
	700: '#b38853',
	900: '#8c5e29'
};

const gray = {
	100: '#f3f4f6',
	300: '#e1e4e8',
	500: '#c0c7cf',
	700: '#9fa4ad',
	900: '#151319'
};

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	daisyui: {
		themes: [
			{
				light: {
					primary: '#4ade80',
					secondary: '#dc7efc',
					accent: '#f2d5ae',
					neutral: '#1B1D2C',
					'base-100': '#f3f4f6',
					info: '#44B1E4',
					success: '#19A38C',
					warning: '#AB7D07',
					error: '#E4255E',
					'--rounded-box': '0rem',
					'--rounded-btn': '0rem',
					'--rounded-badge': '0rem',
					'--animation-btn': '0.25s',
					'--animation-input': '0.2s',
					'--btn-text-case': 'uppercase',
					'--btn-focus-scale': '1',
					'--border-btn': '1px',
					'--tab-border': '1px',
					'--tab-radius': '0rem',
					'--shadow-btn': '0px 4px 0px 0px rgba(0,0,0,1)',
					'--shadow-default': '0px 4px 0px 0px rgba(0,0,0,1)'
				},
				dark: {
					primary: '#4ade80',
					secondary: '#dc7efc',
					accent: '#f2d5ae',
					neutral: '#F7F5F8',
					'base-100': '#151319',
					info: '#44B1E4',
					success: '#19A38C',
					warning: '#AB7D07',
					error: '#E4255E',
					'--rounded-box': '0rem',
					'--rounded-btn': '0rem',
					'--rounded-badge': '0rem',
					'--rounded-progress': '0rem',
					'--animation-btn': '0.25s',
					'--animation-input': '0.2s',
					'--btn-text-case': 'uppercase',
					'--btn-focus-scale': '1',
					'--border-btn': '1px',
					'--tab-border': '1px',
					'--tab-radius': '0rem',
					'--shadow-btn': '0px 4px 0px 0px rgba(255,255,255,1)',
					'--shadow-default': '0px 4px 0px 0px rgba(255,255,255,1)'
				}
			},
			'cupcake'
		],
		darkTheme: 'dark',
		darkMode: 'class'
	},
	theme: {
		extend: {
			backgroundColor: ['selection'],
			textColor: ['selection'],
			dropShadow: {
				neu: '0px 4px 0px 0px black;',
				'neu-light': '0px 4px 0 #3d3a44',
				'neu-pressed': '0px 2px 0 black',
				'neu-pressed-dark': '0px 2px 0 #3D3A44',
				'neu-card': '0px 4px 0 black'
			},
			boxShadow: {
				neu: '0px 4px 0px 0px rgba(0, 0, 0, 1);',
				'neu-light': '0px 4px 0 #3d3a44',
				'neu-pressed': '0px 2px 0 rgba(0, 0, 0, 1)',
				'neu-card': '0px 4px 0 black'
			},
			borderColor: {
				dark: '#3D3A44'
			},
			width: {
				120: '30rem',
				144: '36rem'
			},
			colors: {
				// Your preferred accent color. Indigo is closest to Starlight’s defaults.
				accent: accent,
				// Your preferred gray scale. Zinc is closest to Starlight’s defaults.
				gray: gray
			},
			fontFamily: {
				// Your preferred text font. Starlight uses a system font stack by default.
				sans: ['"Atkinson Hyperlegible"'],
				// Your preferred code font. Starlight uses system monospace fonts by default.
				mono: ['"IBM Plex Mono"']
			}
		}
	},
	plugins: [require('daisyui')]
};
