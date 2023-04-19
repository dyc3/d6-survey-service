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

test('create and delete survey', async ({ page }) => {
	await page.goto('/mysurveys');
	await page.getByRole('heading', { name: 'My Surveys' }).waitFor({ state: 'visible' });

	// create survey
	await page.getByRole('button', { name: 'Create Survey' }).click();
	await page.getByRole('heading', { name: 'Editing' }).waitFor({ state: 'visible' });

	// set the title of the survey
	await page.getByPlaceholder('Survey Title').click();
	await page.getByPlaceholder('Survey Title').fill('foo bar');
	await page.getByRole('heading', { name: 'foo bar' }).waitFor({ state: 'visible' });

	// assert that it saves
	await page.getByText('Saving...').waitFor({ state: 'visible' });
	await page.getByText('Changes saved').waitFor({ state: 'visible' });

	// back to mysurveys page
	await page.goto('/mysurveys');

	// assert that survey with title is present in list
	await page.getByRole('cell', { name: 'foo bar' }).waitFor({ state: 'visible' });

	// delete survey
	await page.getByRole('button', { name: 'Delete' }).click();

	// assert that survey is gone
	expect(await page.locator('cell', { hasText: 'foo bar' }).count()).toEqual(0);

	// reload
	await page.reload();

	// assert that survey is still gone
	expect(await page.locator('cell', { hasText: 'foo bar' }).count()).toEqual(0);
});

test('create a survey with a few questions', async ({ page }) => {
	await page.goto('/mysurveys');
	await page.getByRole('heading', { name: 'My Surveys' }).waitFor({ state: 'visible' });

	await page.getByRole('button', { name: 'Create Survey' }).click();
	await page.getByPlaceholder('Survey Title').click();
	await page.getByPlaceholder('Survey Title').fill('foo');

	await page.getByRole('combobox').selectOption('Text');
	await page.getByRole('button', { name: '+ Add Question' }).click();
	await page.getByPlaceholder('Prompt').click();
	await page.getByPlaceholder('Prompt').fill('q1');

	await page.getByRole('combobox').selectOption('MultipleChoice');
	await page.getByRole('button', { name: '+ Add Question' }).click();
	await page.getByRole('button', { name: '+', exact: true }).click();
	await page.getByPlaceholder('Enter text...').first().click();
	await page.getByPlaceholder('Enter text...').first().fill('c1');
	await page.getByRole('button', { name: '+', exact: true }).click();
	await page.getByPlaceholder('Enter text...').nth(1).click();
	await page.getByPlaceholder('Enter text...').nth(1).fill('c2');

	await page.getByRole('combobox').selectOption('Rating');
	await page.getByRole('button', { name: '+ Add Question' }).click();
	await page.getByPlaceholder('Enter prompt...').click();
	await page.getByPlaceholder('Enter prompt...').fill('q2');
	await page.getByPlaceholder('Insert prompt...').click();
	await page.getByPlaceholder('Insert prompt...').fill('q3');

	await page.getByText('Saving...').waitFor({ state: 'visible' });
	await page.getByText('Changes saved').waitFor({ state: 'visible' });
});

//left off here for test stuff
test('create a survey and navigate away to see if onbeforeunload listener fires', async ({ page }) => {
	await page.goto('/mysurveys');
	await page.getByRole('heading', { name: 'My Surveys' }).waitFor({ state: 'visible' });

	await page.getByRole('button', { name: 'Create Survey' }).click();
	await page.getByPlaceholder('Survey Title').click();
	await page.getByPlaceholder('Survey Title').fill('foo');

	await page.getByRole('combobox').selectOption('Text');
	await page.getByRole('button', { name: '+ Add Question' }).click();
	await page.getByPlaceholder('Prompt').click();
	await page.getByPlaceholder('Prompt').fill('q1');

	let dialogAppeared = false;
	page.on('dialog', async (dialog) => {
		console.log('dialog appeared')
		dialogAppeared = true;
		await dialog.accept();
	});

	await page.evaluate(() => window.location.reload());

	await page.waitForTimeout(1000);

	expect(dialogAppeared).toBe(true);
});
