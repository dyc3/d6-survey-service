<script lang="ts">
    function handleToggle(event: { detail: { text: string; }; }){
        if(event.detail.text == 'toggled'){
            alert('test')
        }
    }
	/**
	 * Whether the button group should show vertically or horizontally.
	 */
	export let orientation: 'horizontal' | 'vertical';

</script>

<div class = 'buttonGroup {orientation}'>
    <slot on:message={handleToggle}/>
</div>

<style lang="scss">
    @import 'main.scss';
    
    .buttonGroup{
        display: flex;
        justify-content: space-around;
        align-items: center;
    }

    .horizontal{
        flex-direction: row;
    }

    .vertical{
        flex-direction: column;
    }
</style>