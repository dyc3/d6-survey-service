import { expect, test } from '@playwright/test';
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
});

test('create a survey and navigate away to see if onbeforeunload listener fires', async ({
	page
}) => {
	//create a survey like prior tests in mysurveys.test.ts
	await page.goto('/mysurveys');
	await page.getByRole('heading', { name: 'My Surveys' }).waitFor({ state: 'visible' });

	await page.getByRole('button', { name: 'Create Survey' }).click();
	await page.getByPlaceholder('Survey Title').click();
	await page.getByPlaceholder('Survey Title').fill('foo');

	await page.getByRole('combobox').selectOption('Text');
	await page.getByRole('button', { name: '+ Add Question' }).click();
	await page.getByPlaceholder('Prompt').click();
	await page.getByPlaceholder('Prompt').fill('q1');

	//prepare dialog handling
	let dialogAppeared = false;
	page.on('dialog', async (dialog) => {
		console.log('dialog appeared');
		dialogAppeared = true;
		await dialog.accept();
	});

	//copy page url
	let url = page.url();
	url = url.replace('edit', 'respond');

	//publish survey, wait to ensure it passes, then navigate to survey response
	await page.getByRole('button', { name: 'Publish Survey' }).click();
	await page.waitForTimeout(100);
	await page.goto(url);

	//make sure survey response page is loaded
	await page.getByRole('heading', { name: 'foo' }).waitFor({ state: 'visible' });

	//fill textbox
	await page.getByRole('textbox').fill('bar');

	//reload page to see if onbeforeunload listener fires
	await page.reload();

	expect(dialogAppeared).toBe(true);
});
