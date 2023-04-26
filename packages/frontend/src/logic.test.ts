import { arraySwap } from '../src/lib/functions/arraySwap';
import { describe, it, expect } from 'vitest';

describe('Ensuring array swap logic works', () => {
	it('should put the item 3 to position 0 and move the rest over by one', () => {
		const arr = [0, 1, 2, 3];
		expect(arraySwap(3, 0, arr)).toEqual([3, 0, 1, 2]);
	});

	it('should put the item 1 to position 7 (put below where dropped) and move the rest over by one', () => {
		const arr = [0, 1, 2, 3, 4, 5, 6];
		expect(arraySwap(1, 6, arr)).toEqual([0, 2, 3, 4, 5, 6, 1]);
	});
});
