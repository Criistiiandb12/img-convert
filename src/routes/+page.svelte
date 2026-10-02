<script lang="ts">
    let selectedFile = $state<File | null>(null);
    let convertedFile = $state<Blob | null>(null);
    let previewUrl = $state<string | null>(null);
    let isDragging = $state(false);
    let isConverting = $state(false);
    let errorMessage = $state('');
    let statusMessage = $state('Listo para convertir');
    let outputFormat = $state('png');

    const acceptedExtensions = ['.jpg', '.jpeg', '.png', '.tif', '.tiff', '.webp', '.bmp'];
    const acceptedMimeTypes = ['image/jpeg', 'image/png', 'image/tiff', 'image/webp', 'image/bmp'];
    const outputFormats = [
        { value: 'png', label: 'PNG', mime: 'image/png', extension: 'png' },
        { value: 'jpg', label: 'JPG', mime: 'image/jpeg', extension: 'jpg' },
        { value: 'webp', label: 'WEBP', mime: 'image/webp', extension: 'webp' },
        { value: 'tiff', label: 'TIFF', mime: 'image/tiff', extension: 'tiff' },
        { value: 'bmp', label: 'BMP', mime: 'image/bmp', extension: 'bmp' }
    ];

    const selectedOutput = $derived(outputFormats.find((format) => format.value === outputFormat) ?? outputFormats[0]);

    function formatBytes(bytes: number) {
        if (bytes < 1024) return `${bytes} B`;
        if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
        return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    }

    function selectFile(file: File | undefined) {
        if (!file) return;
        const extension = `.${file.name.split('.').pop()?.toLowerCase() ?? ''}`;
        errorMessage = '';
        convertedFile = null;
        if (!acceptedMimeTypes.includes(file.type) && !acceptedExtensions.includes(extension)) {
            selectedFile = null;
            previewUrl = null;
            errorMessage = 'Ese formato no está soportado. Usa JPG, PNG, TIFF, WEBP o BMP.';
            statusMessage = 'Formato no compatible';
            return;
        }
        if (previewUrl) URL.revokeObjectURL(previewUrl);
        selectedFile = file;
        previewUrl = URL.createObjectURL(file);
        statusMessage = 'Imagen lista para convertir';
    }

    function handleFile(event: Event) {
        const input = event.currentTarget as HTMLInputElement;
        selectFile(input.files?.[0]);
        input.value = '';
    }

    function handleDrop(event: DragEvent) {
        event.preventDefault();
        isDragging = false;
        selectFile(event.dataTransfer?.files?.[0]);
    }

    function handleDragOver(event: DragEvent) {
        event.preventDefault();
        isDragging = true;
    }

    function handleDragLeave(event: DragEvent) {
        const target = event.currentTarget as HTMLElement;
        if (!target.contains(event.relatedTarget as Node)) isDragging = false;
    }

    function changeOutputFormat() {
        convertedFile = null;
        statusMessage = selectedFile ? 'Imagen lista para convertir' : 'Listo para convertir';
    }

    async function convertImage() {
        if (!selectedFile || isConverting) return;
        isConverting = true;
        errorMessage = '';
        statusMessage = 'Procesando imagen…';
        try {
            const { default: init, convert_to_format } = await import('#lib/wasm/image_converter.js');
            await init();
            const result = convert_to_format(new Uint8Array(await selectedFile.arrayBuffer()), outputFormat);
            const pngBytes = new Uint8Array(result.byteLength);
            pngBytes.set(result);
            convertedFile = new Blob([pngBytes.buffer], { type: selectedOutput.mime });
            statusMessage = 'Conversión completada';
        } catch (error) {
            console.error('Image conversion failed:', error);
            convertedFile = null;
            statusMessage = 'No se pudo convertir la imagen';
            errorMessage = 'No pudimos leer esta imagen. Prueba con otro archivo o formato.';
        } finally {
            isConverting = false;
        }
    }

    function downloadImage() {
        if (!convertedFile || !selectedFile) return;
        const url = URL.createObjectURL(convertedFile);
        const baseName = selectedFile.name.replace(/\.[^/.]+$/, '') || 'imagen';
        const link = document.createElement('a');
        link.href = url;
        link.download = `${baseName}.${selectedOutput.extension}`;
        document.body.appendChild(link);
        link.click();
        link.remove();
        setTimeout(() => URL.revokeObjectURL(url), 1000);
    }

    function reset() {
        if (previewUrl) URL.revokeObjectURL(previewUrl);
        selectedFile = null;
        convertedFile = null;
        previewUrl = null;
        errorMessage = '';
        statusMessage = 'Listo para convertir';
    }
</script>

<svelte:head>
    <title>Convertly — Conversor de imágenes</title>
    <meta name="description" content="Convierte imágenes entre JPG, PNG, WEBP, TIFF y BMP directamente en tu navegador." />
</svelte:head>

<main class="page-shell">
    <nav class="topbar" aria-label="Navegación principal">
        <a class="brand" href="/" aria-label="Convertly inicio"><span class="brand-mark">✦</span> convertly</a>
        <span class="privacy-note"><span class="status-dot"></span> Procesamiento 100% local</span>
    </nav>

    <section class="hero" aria-labelledby="page-title">
        <div class="eyebrow"><span></span> rápido · privado · gratuito</div>
        <h1 id="page-title">Tus imágenes,<br /><em>en el formato correcto.</em></h1>
        <p class="intro">Convierte imágenes a PNG en segundos. Todo ocurre en tu navegador: tus archivos nunca salen de tu dispositivo.</p>
    </section>

    <section class="converter-card" aria-label="Conversor de imágenes">
        <label class:dragging={isDragging} class="dropzone" for="file-input" ondragover={handleDragOver} ondragleave={handleDragLeave} ondrop={handleDrop}>
            <input id="file-input" type="file" accept=".jpg,.jpeg,.png,.tif,.tiff,.webp,.bmp,image/*" onchange={handleFile} />
            <span class="upload-icon" aria-hidden="true">↥</span>
            <span class="drop-title">Arrastra tu imagen aquí</span>
            <span class="drop-subtitle">o <strong>selecciona un archivo</strong> desde tu dispositivo</span>
            <span class="formats">JPG · PNG · TIFF · WEBP · BMP <b>•</b> hasta 50 MB</span>
        </label>

        {#if errorMessage}<p class="message error" role="alert">{errorMessage}</p>{/if}

        {#if selectedFile}
            <div class="file-row">
                <div class="thumb-wrap">{#if previewUrl}<img src={previewUrl} alt="Vista previa de {selectedFile.name}" class="thumbnail" />{/if}</div>
                <div class="file-details"><strong>{selectedFile.name}</strong><span>{formatBytes(selectedFile.size)} <i>·</i> {selectedFile.type || 'imagen'}</span></div>
                <button class="icon-button" type="button" aria-label="Quitar imagen" onclick={reset}>×</button>
            </div>
            <div class="conversion-bar">
                <div class="format-control"><label class="label" for="format-select">Convertir a</label><span class="format-pill"><span class="file-icon">▧</span><select id="format-select" bind:value={outputFormat} onchange={changeOutputFormat} aria-label="Formato de salida">{#each outputFormats as format}<option value={format.value}>{format.label}</option>{/each}</select><span class="chevron">⌄</span></span></div>
                <button class="convert-button" type="button" onclick={convertImage} disabled={isConverting}>
                    {#if isConverting}<span class="spinner"></span> Convirtiendo…{:else}Convertir imagen <span>→</span>{/if}
                </button>
            </div>
        {/if}

        <p class="status" aria-live="polite"><span class:complete={convertedFile} class="status-icon">{convertedFile ? '✓' : 'i'}</span> {statusMessage}</p>

        {#if convertedFile}
            <div class="result-panel">
                <div><span class="result-check">✓</span><div><strong>¡Tu imagen está lista!</strong><small>Convertida a {selectedOutput.label} · {formatBytes(convertedFile.size)}</small></div></div>
                <button class="download-button" type="button" onclick={downloadImage}>Descargar {selectedOutput.label} <span>↓</span></button>
            </div>
        {/if}
    </section>

    <p class="footnote"><span>⌁</span> Tus archivos se procesan localmente con Rust + WebAssembly. No subimos nada a servidores.</p>
</main>

<style>
    @import url('https://fonts.googleapis.com/css2?family=DM+Mono:wght@400;500&family=DM+Sans:wght@400;500;600;700&family=Space+Grotesk:wght@500;600;700&display=swap');
    :global(*) { box-sizing: border-box; }
    :global(body) { margin: 0; background: #f7f8f5; color: #18211e; font-family: 'DM Sans', sans-serif; }
    :global(button), :global(input) { font: inherit; }
    .page-shell { min-height: 100vh; max-width: 1120px; margin: auto; padding: 28px 34px 48px; }
    .topbar { display: flex; align-items: center; justify-content: space-between; }
    .brand { color: #18211e; text-decoration: none; display: flex; gap: 9px; align-items: center; font: 700 21px 'Space Grotesk', sans-serif; letter-spacing: -1px; }
    .brand-mark { display: grid; place-items: center; width: 28px; height: 28px; border-radius: 9px; background: #d8f342; color: #152013; font-size: 17px; }
    .privacy-note, .footnote { color: #7b8580; font: 11px 'DM Mono', monospace; }
    .status-dot { display: inline-block; width: 7px; height: 7px; border-radius: 50%; background: #98bd18; margin-right: 7px; }
    .hero { text-align: center; max-width: 670px; margin: 92px auto 44px; }
    .eyebrow { color: #80931b; text-transform: uppercase; letter-spacing: 2px; font: 10px 'DM Mono', monospace; }
    .eyebrow span { display: inline-block; width: 6px; height: 6px; border-radius: 50%; background: #c4e42c; margin-right: 8px; }
    h1 { font: 600 clamp(42px, 6vw, 72px)/.98 'Space Grotesk', sans-serif; letter-spacing: -4px; margin: 18px 0 20px; }
    h1 em { color: #98aa25; font-style: normal; }
    .intro { max-width: 510px; margin: auto; color: #68736e; font-size: 15px; line-height: 1.65; }
    .converter-card { background: #fff; border: 1px solid #e3e7df; border-radius: 22px; padding: 14px; box-shadow: 0 18px 50px rgba(44, 59, 36, .07); max-width: 820px; margin: auto; }
    .dropzone { min-height: 270px; border: 1.5px dashed #cfd7c8; border-radius: 14px; display: flex; flex-direction: column; align-items: center; justify-content: center; cursor: pointer; transition: .2s; }
    .dropzone:hover, .dropzone.dragging { border-color: #9dbd24; background: #fbfdea; }
    .dropzone input { display: none; }
    .upload-icon { display: grid; place-items: center; width: 50px; height: 50px; color: #748a18; background: #f0f5d9; border-radius: 15px; font-size: 29px; margin-bottom: 17px; }
    .drop-title { font: 600 18px 'Space Grotesk', sans-serif; }
    .drop-subtitle { color: #7b8580; margin-top: 7px; font-size: 13px; } .drop-subtitle strong { color: #839a16; }
    .formats { margin-top: 25px; color: #9da59e; font: 10px 'DM Mono', monospace; letter-spacing: .4px; } .formats b { color: #c7d247; margin: 0 7px; }
    .message { margin: 16px 4px 0; padding: 11px 14px; border-radius: 9px; font-size: 13px; } .error { background: #fff0ed; color: #b24d3c; }
    .file-row { display: flex; align-items: center; gap: 13px; padding: 18px 8px 13px; border-bottom: 1px solid #edf0ea; }
    .thumb-wrap { width: 48px; height: 48px; border-radius: 10px; overflow: hidden; background: #f1f3ed; flex: none; } .thumbnail { width: 100%; height: 100%; object-fit: cover; }
    .file-details { display: flex; flex: 1; min-width: 0; flex-direction: column; gap: 5px; font-size: 13px; } .file-details strong { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; } .file-details span { color: #919a94; font: 10px 'DM Mono', monospace; } .file-details i { color: #cad64e; font-style: normal; }
    .icon-button { border: 0; color: #adb5ad; background: none; cursor: pointer; font-size: 25px; line-height: 1; }
    .conversion-bar { display: flex; align-items: end; justify-content: space-between; gap: 20px; padding: 18px 8px 7px; } .format-control { display: flex; flex-direction: column; gap: 8px; } .label { color: #89938c; font: 10px 'DM Mono', monospace; text-transform: uppercase; letter-spacing: 1px; }
    .format-pill { display: flex; gap: 9px; align-items: center; min-width: 120px; padding: 11px 12px; border: 1px solid #e0e6d9; border-radius: 9px; font-size: 13px; font-weight: 600; } .file-icon { color: #a1b724; } .format-pill select { appearance: none; border: 0; outline: 0; background: transparent; color: inherit; cursor: pointer; font-weight: 600; } .chevron { color: #9aa39a; margin-left: auto; pointer-events: none; }
    .convert-button, .download-button { border: 0; border-radius: 9px; cursor: pointer; transition: transform .2s, background .2s; } .convert-button { background: #c9e52f; color: #263008; padding: 13px 19px; font-size: 13px; font-weight: 700; } .convert-button:hover, .download-button:hover { transform: translateY(-1px); background: #b9d82a; } button:disabled { cursor: wait; opacity: .7; }
    .convert-button span, .download-button span { font-size: 17px; margin-left: 8px; } .spinner { display: inline-block; width: 12px; height: 12px; border: 2px solid #71811f; border-top-color: transparent; border-radius: 50%; animation: spin .7s linear infinite; } @keyframes spin { to { transform: rotate(360deg); } }
    .status { color: #929b94; display: flex; align-items: center; gap: 7px; margin: 17px 8px 4px; font: 10px 'DM Mono', monospace; } .status-icon { display: grid; place-items: center; width: 16px; height: 16px; border-radius: 50%; background: #edf0e9; color: #87918a; font-size: 10px; } .status-icon.complete { background: #e8f5b5; color: #6d8610; }
    .result-panel { background: #f7fbe9; border: 1px solid #e6efbf; border-radius: 12px; display: flex; align-items: center; justify-content: space-between; gap: 16px; padding: 14px 15px; margin-top: 14px; } .result-panel > div { display: flex; gap: 10px; align-items: center; } .result-check { display: grid; place-items: center; width: 28px; height: 28px; border-radius: 50%; background: #d7ef67; color: #5c7410; } .result-panel strong, .result-panel small { display: block; } .result-panel strong { font-size: 13px; } .result-panel small { color: #8d9b61; font: 10px 'DM Mono', monospace; margin-top: 4px; }
    .download-button { background: #c9e52f; color: #293207; padding: 11px 14px; font-size: 12px; font-weight: 700; white-space: nowrap; } .footnote { text-align: center; margin: 25px auto 0; } .footnote span { color: #a6bd22; font-size: 16px; vertical-align: middle; margin-right: 5px; }
    @media (max-width: 600px) { .page-shell { padding: 22px 17px 34px; } .privacy-note { display: none; } .hero { margin-top: 68px; } h1 { letter-spacing: -2px; } .conversion-bar, .result-panel { align-items: stretch; flex-direction: column; } .convert-button, .download-button { width: 100%; } .dropzone { min-height: 245px; } }
</style>
