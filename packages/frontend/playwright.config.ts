import type { PlaywrightTestConfig } from '@playwright/test';

const config: PlaywrightTestConfig = {
	webServer: [
		{
			command: 'yarn run build && yarn run preview',
			port: 4173
		},
		{
			command: 'cargo run',
			port: 5347
		}
	],
	testDir: 'tests',
	use: {
		baseURL: 'http://localhost:4173/'
	}
};

export default config;
