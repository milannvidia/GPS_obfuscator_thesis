<script lang="ts">
	import { page } from '$app/state';
	import { base } from '$app/paths';
	let activeUrl = $derived(page.url.pathname);
	import {
		Sidebar,
		SidebarGroup,
		SidebarWrapper,
		SidebarItem
	} from 'flowbite-svelte';
	import '../app.css';
	import { HomeSolid, ImageSolid } from 'flowbite-svelte-icons';
	let { children } = $props();
</script>

<Sidebar {activeUrl} class="fixed top-0 left-0 z-40 h-screen">
	<SidebarWrapper>
		<SidebarGroup class="h-screen">
			<img src="{base}/logo.png" alt="" />
			<SidebarItem label="Obfuscation algor" href="{base}/"></SidebarItem>
			<SidebarItem label="Android coarse comparison" href="{base}/android-coarse" />
			<SidebarItem label="Android parameter" href="{base}/android-priority-coarse" />
		</SidebarGroup>
	</SidebarWrapper>
</Sidebar>
<div class="ml-64 h-full">
	{@render children()}
</div>
