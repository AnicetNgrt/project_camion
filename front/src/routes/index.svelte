<script>
import Page from "$components/molecules/Page.svelte";
import Separator from "$components/atoms/Separator.svelte";
</script>

<Page>
    <div id="title-container">
        <div id="title" class="row center-y center-x">
            <div>
                <img id="laugh" src="images/laugh_emoji.svg" alt="laugh emoji"/>
                <Separator size="xl"/>
            </div>
            <div class="col center-x">
                <h4 id="title-top">Les blagues fameuses de</h4>
                <h1 id="title-bottom">Félicie</h1>
                <h5 id="subtitle">Contenu garanti 100% troublant</h5>
            </div>
            <div>
                <Separator size="xl"/>
                <img id="laugh" src="images/sceptic_emoji.svg" alt="laugh emoji"/>
            </div>
        </div>
    </div>
    <Separator size="lg"/>
    <div class="joke-container">
        <div class="joke-stamp"></div>
        <h3 class="joke-title">🚚 Le camion</h3>
        <Separator size="mi"/>
        <p class="joke-body">
            <i>Félicie :</i> Demande moi si je suis un camion.
            <br/>
            <i>Audience :</i> Est-ce que tu es un camion ?
            <br/>
            <i>Félicie :</i> Non.
        </p>
    </div>
    <div class="joke-container">
        <h3 class="joke-title">👱‍♀️ Une blague sur les blondes</h3>
        <Separator size="mi"/>
        <p class="joke-body">
            <i>Félicie :</i> Sais-tu comment fait-on pour troubler une blonde ?
            <br/>
            <i>Audience :</i> Je ne pense pas...
            <br/>
            <i>Félicie :</i> En se peignant en violet et en lui lançant des fourchettes.
        </p>
    </div>
    <div class="joke-container">
        <h3 class="joke-title">🐶 Celle-ci a du chien</h3>
        <Separator size="mi"/>
        <p class="joke-body">
            <i>Félicie :</i> Y a-t-il une différence entre un caniche et un séquoïa ?
            <br/>
            <i>Audience :</i> Très certainement...
            <br/>
            <i>Félicie :</i> En fait non. Tous les deux sont des caniches. Sauf le séquoïa.
        </p>
    </div>
    <div class="joke-container">
        <h3 class="joke-title">🤯 Franchement retournant</h3>
        <Separator size="mi"/>
        <p class="joke-body">
            <i>Félicie :</i> Demande moi si je suis un liquide de refroidissement.
            <br/>
            <i>Audience :</i> Est-ce que tu es un liquide de refroidissement ?
            <br/>
            <i>Félicie :</i> Oui !
            <br/>
            <i>Audience :</i> Quoi ? Vraiment ?!
            <br/>
            <i>Félicie :</i> Non, j'ai menti :)
        </p>
    </div>

    <Separator size="xl"/>
</Page>

<style>
    .joke-container {
        max-width: 100%;
        width: 65ch;
        padding: var(--sm) var(--md);
        padding-bottom: var(--lg);
        margin-top: var(--md);
        background-color: rgb(255, 255, 255);
        border: solid var(--bord) var(--ccontrast);
        border-radius: var(--brad);
        box-shadow: var(--shad-offx) var(--shad-offy) var(--ssmth) var(--ccontrast-shadow);
        transform: translate(calc(var(--shad-offx) / -2), calc(var(--shad-offy) / -2)) rotate(-0.5deg);
    }

    .joke-body, .joke-title {
        max-width: 100%;
    }

    .joke-title {
        font-family: Lora;
    }

    #title-container {
        max-width: 100%;
        width: 65ch;
        padding: var(--xl) var(--md);
        margin-top: var(--md);
        background-color: rgb(255, 255, 255);
        border: solid var(--bord) var(--ccontrast);
        border-radius: var(--brad);
        box-shadow: var(--shad-offx) var(--shad-offy) var(--ssmth) var(--ccontrast-shadow);
        transform: translate(calc(var(--shad-offx) / -2), calc(var(--shad-offy) / -2)) rotate(1deg);
    }

    #title {
        transform: rotate(-5deg);
    }

    #title-top, #title-bottom, #subtitle {
        font-weight: 400;
    }

    #title-bottom {
        margin-top: -0.1em;
        margin-bottom: 0em;
        font-family: Lora;
        font-size: var(--giga);
    }

    #subtitle {
        transform: rotate(3deg);
    }

    #laugh {
        width: var(--giga);
    }
</style>