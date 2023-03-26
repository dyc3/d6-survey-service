<script lang="ts">
	import type { ValidationError } from './common';

	export let error: ValidationError;

	function buildMessage(error: ValidationError): string {
		switch (error.type) {
			case 'BadValue':
				return `Invalid value for ${error.data.field}: ${error.data.message}`;
			case 'MismatchedTypes':
				return `Mismatched types for ${error.data.uuid}`;
			case 'NotFound':
				return `Not found: ${error.data.uuid}`;
			case 'NotInRange':
				return `${error.data.field} must be in range ${error.data.min} to ${error.data.max}, but was ${error.data.value}`;
			case 'NotUnique':
				return `${error.data.field} must be unique, but was ${error.data.value}`;
			case 'Required':
				return `${error.data.field} is required`;
			default:
				return `Unable to render error: ${error.type}`;
		}
	}

	$: message = buildMessage(error);
</script>

<span>{message}</span>

<style lang="scss">
	@import './ui/variables';

	span {
		color: $color-danger;
	}
</style>
