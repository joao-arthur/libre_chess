<script lang="ts">
    import { onMount } from "svelte";
    import initWASM, {
        engineAddOnChangeListener,
        engineGetSettings,
        EngineInfo,
        engineInit,
        engineGetBoardColorPresets,
        engineGetBoardSetPresets,
        engineSetBoardColor,
        engineSetBoardSet,
        engineSetDimension,
    } from "chess_engine";
    import Select from "$lib/components/Select.svelte";

    let initiated = $state(false);
    let boardColorPresets = $state([]);
    let boardSetPresets = $state([]);
    let model = $state({
        board_color: "",
        board_set: "block",
    } as EngineInfo);

    let innerWidth = $state(0);
    let innerHeight = $state(0);

    let canvas: HTMLCanvasElement;

    onMount(() => {
        if (!initiated) {
            initWASM().then(() => {
                initiated = true;
                let context = canvas.getContext("2d");
                if (!context) {
                    return;
                }
                engineAddOnChangeListener(() => {
                    model = engineGetSettings();
                });
                engineInit(canvas);
                boardColorPresets = engineGetBoardColorPresets();
                boardSetPresets = engineGetBoardSetPresets();
                engineSetDimension(Math.min(innerWidth, innerHeight));
            });
        }
    });

    function handleSetBoardColor(preset: string) {
            engineSetBoardColor(preset);
    }

    function handleSetBoardSet(preset: string) {
            engineSetBoardSet(preset);
    }
</script>

<svelte:window bind:innerWidth bind:innerHeight />

<style>
    main {
        width: 100vw;
        height: 100vh;
        display: flex;
    }

    canvas {
        margin: auto;
    }

    .form {
        display: flex;
        flex-direction: column;
        row-gap: 1rem;
    }

    .field-container {
        display: flex;
        flex-direction: column;
    }
</style>

<main>
    <canvas
        bind:this={canvas}
        width={Math.min(innerHeight, innerWidth)}
        height={Math.min(innerHeight, innerWidth)}
        style={`width: ${Math.min(innerHeight, innerWidth)}px; height: ${
            Math.min(innerHeight, innerWidth)
        }px;`}
    >
    </canvas>
    <div class="form">
        <div class="field-container">
            <label for="boardColor">Board Color</label>
            <Select
                id="boardColor"
                options={boardColorPresets.map((item: any) => ({
                            label: item.name,
                            value: item.id,
                        }))
                    }
                value={model?.board_color || ""}
                onChange={handleSetBoardColor}
            />
        </div>
        <div class="field-container">
            <label for="boardSet">Board Set</label>
            <Select
                id="boardColor"
                options={boardSetPresets.map((item: any) => ({
                    label: item.name,
                    value: item.id,
                }))}
                value={model?.board_set || ""}
                onChange={handleSetBoardSet}
            />
        </div>
    </div>
</main>
