export function arrayMove<T>(array: T[], oldIndex: number, newIndex: number): T[] {
	const item = array.splice(oldIndex, 1)[0];
	array.splice(newIndex, 0, item);
	return array;
}
