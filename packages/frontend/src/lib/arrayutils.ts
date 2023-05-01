export function arrayMove(array: any[], oldIndex: number, newIndex: number) {
	if (oldIndex < newIndex) {
		return [
			...array.slice(0, oldIndex),
			...array.slice(oldIndex + 1, newIndex + 1),
			array[oldIndex],
			...array.slice(newIndex + 1)
		];
	} else {
		return [
			...array.slice(0, newIndex),
			array[oldIndex],
			...array.slice(newIndex, oldIndex),
			...array.slice(oldIndex + 1)
		];
	}
}
