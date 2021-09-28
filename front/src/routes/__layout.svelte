<script lang="ts">
    import '$styles/theme-engine.css';
    import '$styles/themes.css';
    import '$styles/global.css';
    import '$styles/flex-helpers.css';

    import { onMount } from 'svelte';

    let unmounted = true;
    onMount(() => {
        unmounted = false;
    });

    const description = "";
    const title = "Camion";
    const application_name = "Camion";
    const keywords = ""
    const thumbnail = "";
    const thumbnail_height = "0";
    const thumbnail_width = "0";
    const url = "https://camion.anicetnougaret.fr";
</script>

<svelte:head>
    <meta name="application-name" content={application_name}/>
    <meta name="keywords" content={keywords}/>
    <title>{title}</title>
    <meta name="description" content={description}/>
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:site" content="@AniC_dev" />
    <meta name="twitter:creator" content="@AniC_dev" />
    <meta property="og:url" content="{url}" />
    <meta property="og:title" content={title} />
    <meta property="og:description" content={description} />
    <meta property="og:image" content="http://{thumbnail}" />
    <meta property="og:image:secure_url" content="https://{thumbnail}" /> 
    <meta property="og:image:type" content="image/jpeg" /> 
    <meta property="og:image:width" content="{thumbnail_width}" /> 
    <meta property="og:image:height" content="{thumbnail_height}" />
</svelte:head>

<main class="theme theme-light theme-compute" class:unmounted>
    <slot/>
</main>

<style>
    main {
        height: 100vh;
        overflow-x: hidden;
    }

    .unmounted {
        opacity: 0;
    }
</style>