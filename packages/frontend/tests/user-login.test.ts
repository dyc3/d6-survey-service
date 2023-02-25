import { expect, test } from '@playwright/test';
import crypto from 'crypto';

test.afterEach(async ({ page }) => {
	await page.evaluate(() => localStorage.clear());
});

test('should register user or log in user and go to mysurveys', async ({ page }, testInfo) => {
	const username = `testuser-${crypto.randomUUID()}`;
	const password = 'pass';

	await page.goto('/login');
	await page.getByRole("tab", { name: "Register" }).click();
	await page.fill('input[name="username"]', username);
	await page.fill('input[name="password"]', password);
	await page.getByRole("button", { name: "Submit" }).click();
	await page.getByRole('heading', { name: 'My Surveys' }).waitFor({ state: "visible" });
	// await page.screenshot({ path: `screenshots/${testInfo.title}.png` });
	let token = await page.evaluate(() => localStorage.getItem('token'));
	expect(token).toBeTruthy();

	await page.evaluate(() => localStorage.clear());

	await page.goto('/login');
	await page.getByRole("tab", { name: "Log In" }).click();
	await page.fill('input[name="username"]', username);
	await page.fill('input[name="password"]', password);
	await page.getByRole("button", { name: "Submit" }).click();
	await page.getByRole('heading', { name: 'My Surveys' }).waitFor({ state: "visible" });
	token = await page.evaluate(() => localStorage.getItem('token'));
	expect(token).toBeTruthy();
});

test('should not register user and should not log in', async ({ page }, testInfo) => {
	await page.goto('/login');
	await page.getByRole("tab", { name: "Register" }).click();
	await page.getByRole("button", { name: "Submit" }).click();
	await page.getByText("InvalidCredentials").waitFor({ state: "visible" });
	// await page.screenshot({ path: `screenshots/${testInfo.title}.png` });

	await page.getByRole("tab", { name: "Log In" }).click();
	await page.getByRole("button", { name: "Submit" }).click();
	await page.getByText("InvalidCredentials").waitFor({ state: "visible" });
	// await page.screenshot({ path: `screenshots/${testInfo.title}.png` });
});
