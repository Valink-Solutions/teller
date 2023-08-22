/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	daisyui: {
		themes: [
			{
				neubrutalism: {
					primary: '#4ade80',
					secondary: '#dc7efc',
					accent: '#f2d5ae',
					neutral: '#1B1D2C',
					'base-100': '#f3f4f6',
					info: '#44B1E4',
					success: '#19A38C',
					warning: '#AB7D07',
					error: '#E4255E',
					'--rounded-box': '0.25rem',
					'--rounded-btn': '0.25rem',
					'--rounded-badge': '0rem',
					'--rounded-progress': '0rem',
					'--animation-btn': '0.25s',
					'--animation-input': '0.2s',
					'--btn-text-case': 'uppercase',
					'--btn-focus-scale': '1',
					'--border-btn': '1px',
					'--tab-border': '1px',
					'--tab-radius': '0rem',
					'--shadow-btn': '3px 3px 0px 0px rgba(0,0,0,1)',
					'--shadow-default': '5px 5px 0px 0px rgba(0,0,0,1)'
				},
				'neubrutalism-dark': {
					primary: '#4ade80',
					secondary: '#dc7efc',
					accent: '#f2d5ae',
					neutral: '#1B1D2C',
					'base-100': '#111827',
					info: '#44B1E4',
					success: '#19A38C',
					warning: '#AB7D07',
					error: '#E4255E',
					'--rounded-box': '0.5rem',
					'--rounded-btn': '0.2rem',
					'--rounded-badge': '0rem',
					'--rounded-progress': '0rem',
					'--animation-btn': '0.25s',
					'--animation-input': '0.2s',
					'--btn-text-case': 'uppercase',
					'--btn-focus-scale': '1',
					'--border-btn': '1px',
					'--tab-border': '1px',
					'--tab-radius': '0rem',
					'--shadow-btn': '3px 3px 0px 0px rgba(255,255,255,1)',
					'--shadow-default': '5px 5px 0px 0px rgba(255,255,255,1)'
				}
			},
			'dark',
			'cupcake'
		],
		darkTheme: 'neubrutalism'
	},
	theme: {
		extend: {
			backgroundColor: ['selection'],
			textColor: ['selection'],
			dropShadow: {
				neu: '3px 3px 0px 0px rgba(0, 0, 0, 1)',
				'neu-light': '3px 3px 0 rgba(61, 66, 77, 1)',
				'neu-pressed': '2px 2px 0 rgba(0, 0, 0, 1)',
				'neu-card': '3px 3px 0 rgba(0, 0, 0, 1)'
			},
			boxShadow: {
				neu: '3px 3px 0px 0px rgba(0, 0, 0, 1)',
				'neu-light': '3px 3px 0 rgba(61, 66, 77, 1)',
				'neu-pressed': '2px 2px 0 rgba(0, 0, 0, 1)',
				'neu-card': '3px 3px 0 rgba(0, 0, 0, 1)'
			},
			borderColor: {
				dark: 'rgba(61, 66, 77, 1)'
			},
			width: {
				120: '30rem',
				144: '36rem'
			}
		}
	},
	plugins: [require('daisyui')]
};
