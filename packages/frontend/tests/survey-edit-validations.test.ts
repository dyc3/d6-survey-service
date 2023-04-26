import { expect, test, type Page } from '@playwright/test';
import crypto from 'crypto';

const username = `testuser-${crypto.randomUUID()}`;
const password = 'pass';

test.beforeAll(async ({ browser }) => {
	const page = await browser.newPage();

	await page.goto('/login');
	await page.getByRole('tab', { name: 'Register' }).click();
	await page.fill('input[name="username"]', username);
	await page.fill('input[name="password"]', password);
	await page.getByRole('button', { name: 'Submit' }).click();
	await page.getByRole('heading', { name: 'My Surveys' }).waitFor({ state: 'visible' });
	const token = (await page.evaluate(() => localStorage.getItem('token'))) ?? '';

	if (token === '') {
		throw new Error('Token is empty');
	}
});

test.beforeEach(async ({ page }) => {
	await page.goto('/login');
	await page.getByRole('tab', { name: 'Log In' }).click();
	await page.fill('input[name="username"]', username);
	await page.fill('input[name="password"]', password);
	await page.getByRole('button', { name: 'Submit' }).click();
	await page.getByRole('heading', { name: 'My Surveys' }).waitFor({ state: 'visible' });

	// Create a survey to work with
	await page.getByRole('button', { name: 'Create Survey' }).click();
	await page.getByRole('heading', { name: 'Editing' }).waitFor({ state: 'visible' });
});

async function waitForSave(page: Page, success = true) {
	await page.getByText('Saving...').waitFor({ state: 'visible' });
	if (success) {
		await page.getByText('Changes saved').waitFor({ state: 'visible' });
	} else {
		await page.getByText('Changes not saved').waitFor({ state: 'visible' });
	}
}

test('survey title validation', async ({ page }) => {
	await page.getByPlaceholder('Survey Title').fill('');
	await waitForSave(page, false);
	await expect(page.getByText('title is required')).toBeVisible();
	await page.getByPlaceholder('Survey Title').fill('foo bar');
	await expect(page.getByText('title is required')).not.toBeVisible();
	await expect(page.getByRole('heading', { name: 'foo bar' })).toBeVisible();
});
