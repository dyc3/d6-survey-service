import { arrayMove } from './arrayutils';
import { describe, it, expect } from 'vitest';

describe('arrayutils.move', () => {
	it('should put the item 3 to position 0 and move the rest over by one', () => {
		const arr = [0, 1, 2, 3];
		expect(arrayMove(arr, 3, 0)).toEqual([3, 0, 1, 2]);
	});

	it('should put the item 1 to position 7 (put below where dropped) and move the rest over by one', () => {
		const arr = [0, 1, 2, 3, 4, 5, 6];
		expect(arrayMove(arr, 1, 6)).toEqual([0, 2, 3, 4, 5, 6, 1]);
	});
});
