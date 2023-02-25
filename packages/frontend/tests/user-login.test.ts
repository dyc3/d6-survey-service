import { expect, test } from '@playwright/test';
import crypto from 'crypto';

test('should register user and go to mysurveys', async ({ page }, testInfo) => {
	await page.goto('/login');
	await page.getByRole("tab", { name: "Register" }).click();
	await page.fill('input[name="username"]', `testuser-${crypto.randomUUID()}`);
	await page.fill('input[name="password"]', 'pass');
	await page.getByRole("button", { name: "Submit" }).click();
	await page.getByRole('heading', { name: 'My Surveys' }).waitFor({ state: "visible" });
	// await page.screenshot({ path: `screenshots/${testInfo.title}.png` });
});

test('should log in existing user and go to mysurveys', async ({ page, request }, testInfo) => {
	const username = `testuser-${crypto.randomUUID()}`;
	const password = 'pass';
	const register = await request.post('/api/user/register', {
		data: { username, password }
	});
	expect(register.ok()).toBe(true);

	await page.goto('/login');
	await page.getByRole("tab", { name: "Log In" }).click();
	await page.fill('input[name="username"]', username);
	await page.fill('input[name="password"]', password);
	await page.getByRole("button", { name: "Submit" }).click();
	await page.getByRole('heading', { name: 'My Surveys' }).waitFor({ state: "visible" });
	// await page.screenshot({ path: `screenshots/${testInfo.title}.png` });
});
