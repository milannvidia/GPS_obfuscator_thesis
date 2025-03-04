<script lang="ts">
	import { page } from '$app/state';
	import { base } from '$app/paths';
	let activeUrl = $derived(page.url.pathname);
	import {
		Sidebar,
		SidebarGroup,
		SidebarWrapper,
		SidebarItem,
		SidebarDropdownWrapper,
		SidebarDropdownItem
	} from 'flowbite-svelte';
	import '../app.css';
	import { HomeSolid, ImageSolid } from 'flowbite-svelte-icons';
	let { children } = $props();
</script>

<Sidebar {activeUrl} class="fixed top-0 left-0 z-40 h-screen">
	<SidebarWrapper>
		<SidebarGroup class="h-screen">
			<SidebarItem label="Home" href="{base}/">
				{#snippet icon()}
					<HomeSolid />
				{/snippet}
			</SidebarItem>
			<SidebarDropdownWrapper label="Visualisatie">
				{#snippet icon()}
					<ImageSolid />
				{/snippet}
				<SidebarDropdownItem label="Obfuscate" href="{base}/obfuscation" />
				<SidebarDropdownItem label="Android coarse comparison" href="{base}/android-coarse" />
			</SidebarDropdownWrapper>
		</SidebarGroup>
	</SidebarWrapper>
</Sidebar>
<div class="ml-64 h-full">
	{@render children()}
</div>
