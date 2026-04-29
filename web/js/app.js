// TrioLingo Web — Main Application Controller
import init, {
    list_transforms, encode, decode, run_pipeline,
    analyze_all, universal_decode, chain_decode,
    estimate_tokens, fragment_payload, randomize,
    image_stego_embed, image_stego_extract
} from '../pkg/triolingo.js';

// ═══ State ═══
const state = {
    transforms: [],
    selectedTransform: null,
    pipeline: [],
    params: {},
    wasmReady: false,
};

// ═══ Init ═══
async function boot() {
    showLoading(true);
    try {
        await init();
        state.wasmReady = true;
        state.transforms = JSON.parse(list_transforms());
        buildSidebar();
        bindEvents();
        showLoading(false);
        toast('WASM loaded — 60 transforms ready', 'success');
    } catch (e) {
        console.error('WASM init failed:', e);
        showLoading(false);
        toast('Failed to load WASM module: ' + e.message, 'error');
    }
}

// ═══ Sidebar ═══
function buildSidebar() {
    const container = document.getElementById('categoryList');
    const groups = {};
    state.transforms.forEach(t => {
        if (!groups[t.category]) groups[t.category] = [];
        groups[t.category].push(t);
    });
    // Sort categories
    const catOrder = ['Encoding','Cipher','Visual','Unicode Style','Script','Formatting','Homoglyph','Semantic','Steganography','Analysis'];
    const sortedCats = Object.keys(groups).sort((a,b) => {
        const ia = catOrder.indexOf(a), ib = catOrder.indexOf(b);
        return (ia === -1 ? 99 : ia) - (ib === -1 ? 99 : ib);
    });

    container.innerHTML = sortedCats.map(cat => {
        const items = groups[cat].sort((a,b) => a.name.localeCompare(b.name));
        return `
        <div class="category-group">
            <div class="category-header" data-cat="${cat}">
                <span>${cat} <span class="cat-count">${items.length}</span></span>
                <span class="cat-chevron">▼</span>
            </div>
            <div class="category-items" data-cat-items="${cat}">
                ${items.map(t => `
                    <div class="transform-item" data-key="${t.key}" title="${t.description}">
                        <span class="t-rev">${t.reversible ? '⟳' : '→'}</span>
                        <span class="t-name">${t.name}</span>
                        <span class="t-add">+ chain</span>
                    </div>
                `).join('')}
            </div>
        </div>`;
    }).join('');

    // Category collapse
    container.querySelectorAll('.category-header').forEach(h => {
        h.addEventListener('click', () => {
            h.classList.toggle('collapsed');
            const items = container.querySelector(`[data-cat-items="${h.dataset.cat}"]`);
            items.classList.toggle('collapsed');
        });
    });

    // Transform click → select, shift+click → add to pipeline
    container.querySelectorAll('.transform-item').forEach(item => {
        item.addEventListener('click', (e) => {
            if (e.shiftKey || e.target.classList.contains('t-add')) {
                addToPipeline(item.dataset.key);
            } else {
                selectTransform(item.dataset.key);
            }
        });
    });
}

function selectTransform(key) {
    state.selectedTransform = key;
    // Highlight
    document.querySelectorAll('.transform-item').forEach(el => el.classList.remove('active'));
    const el = document.querySelector(`.transform-item[data-key="${key}"]`);
    if (el) el.classList.add('active');
    // Show params
    const t = state.transforms.find(t => t.key === key);
    if (t && t.parameters.length > 0) {
        showParams(t);
    } else {
        document.getElementById('paramPanel').classList.add('hidden');
    }
}

function showParams(transform) {
    const panel = document.getElementById('paramPanel');
    const controls = document.getElementById('paramControls');
    document.getElementById('paramTransformName').textContent = transform.name;

    controls.innerHTML = transform.parameters.map(p => {
        const paramType = p.param_type;
        if (paramType.startsWith('Choice')) {
            const choices = paramType.match(/\["([^"]+)"(?:,\s*"([^"]+)")*\]/);
            const options = paramType.match(/"([^"]+)"/g)?.map(s => s.replace(/"/g, '')) || [];
            return `<div class="param-group">
                <label class="param-label">${p.name}</label>
                <select class="param-select" data-param="${p.name}">
                    ${options.map(o => `<option value="${o}" ${o === p.default_value ? 'selected' : ''}>${o}</option>`).join('')}
                </select>
            </div>`;
        } else if (paramType.startsWith('Integer')) {
            const minMax = paramType.match(/min:\s*(-?\d+).*max:\s*(-?\d+)/);
            const min = minMax ? minMax[1] : 0;
            const max = minMax ? minMax[2] : 100;
            return `<div class="param-group">
                <label class="param-label">${p.name}</label>
                <input type="number" class="param-input" data-param="${p.name}" value="${p.default_value}" min="${min}" max="${max}">
            </div>`;
        } else {
            return `<div class="param-group">
                <label class="param-label">${p.name}</label>
                <input type="text" class="param-input" data-param="${p.name}" value="${p.default_value}" placeholder="${p.description}">
            </div>`;
        }
    }).join('');

    // Init params state
    state.params = {};
    transform.parameters.forEach(p => { state.params[p.name] = p.default_value; });

    // Bind param change events
    controls.querySelectorAll('.param-input, .param-select').forEach(el => {
        el.addEventListener('change', () => { state.params[el.dataset.param] = el.value; });
        el.addEventListener('input', () => { state.params[el.dataset.param] = el.value; });
    });

    panel.classList.remove('hidden');
}

// ═══ Pipeline ═══
function addToPipeline(key) {
    state.pipeline.push(key);
    renderPipeline();
    toast(`Added ${key} to pipeline`, 'success');
}

function renderPipeline() {
    const container = document.getElementById('pipelineChips');
    if (state.pipeline.length === 0) {
        container.innerHTML = '<span class="pipeline-placeholder">Click transforms to build a chain...</span>';
        return;
    }
    container.innerHTML = state.pipeline.map((key, i) =>
        (i > 0 ? '<span class="pipeline-arrow">→</span>' : '') +
        `<span class="pipeline-chip" data-idx="${i}">${key}<span class="chip-remove" data-idx="${i}">✕</span></span>`
    ).join('');

    container.querySelectorAll('.chip-remove').forEach(btn => {
        btn.addEventListener('click', (e) => {
            e.stopPropagation();
            state.pipeline.splice(parseInt(btn.dataset.idx), 1);
            renderPipeline();
        });
    });
}

function runPipelineAction(reverse) {
    if (state.pipeline.length === 0) return toast('Pipeline is empty', 'error');
    const input = document.getElementById('inputText').value;
    if (!input) return toast('Input is empty', 'error');
    const chain = state.pipeline.join(' -> ');
    const result = JSON.parse(run_pipeline(chain, input, reverse));
    if (result.ok) {
        document.getElementById('outputText').value = result.result;
        updateStats();
        toast(`Pipeline ${reverse ? 'reversed' : 'executed'} (${state.pipeline.length} steps)`, 'success');
    } else {
        toast(result.error, 'error');
    }
}

// ═══ Core Encode/Decode ═══
function doEncode() {
    const input = document.getElementById('inputText').value;
    if (!input) return toast('Input is empty', 'error');

    // If pipeline has items, run pipeline
    if (state.pipeline.length > 0) return runPipelineAction(false);

    if (!state.selectedTransform) return toast('Select a transform first', 'error');
    const params = JSON.stringify(state.params || {});
    const result = JSON.parse(encode(state.selectedTransform, input, params));
    if (result.ok) {
        document.getElementById('outputText').value = result.result;
        updateStats();
    } else {
        toast(result.error, 'error');
    }
}

function doDecode() {
    const input = document.getElementById('inputText').value;
    if (!input) return toast('Input is empty', 'error');

    if (state.pipeline.length > 0) return runPipelineAction(true);

    if (!state.selectedTransform) {
        // Universal decode
        const results = JSON.parse(universal_decode(input));
        if (results.length > 0) {
            document.getElementById('outputText').value = results.map((r, i) =>
                `[${i+1}] ${r.transform} (${(r.confidence * 100).toFixed(0)}%): ${r.decoded}`
            ).join('\n');
            updateStats();
            toast(`Found ${results.length} possible decodings`, 'success');
        } else {
            toast('No decodings found', 'error');
        }
        return;
    }

    const params = JSON.stringify(state.params || {});
    const result = JSON.parse(decode(state.selectedTransform, input, params));
    if (result.ok) {
        document.getElementById('outputText').value = result.result;
        updateStats();
    } else {
        toast(result.error, 'error');
    }
}

// ═══ Analysis ═══
function runAnalysis() {
    const input = document.getElementById('inputText').value;
    if (!input.trim()) { resetAnalysis(); return; }

    const report = JSON.parse(analyze_all(input));

    const dash = document.getElementById('analysisDashboard');

    // Prompt Injection
    const piRisk = document.getElementById('piRisk');
    piRisk.textContent = report.prompt_injection?.risk_level || 'CLEAN';
    piRisk.className = 'risk-badge risk-' + (report.prompt_injection?.risk_level || 'clean').toLowerCase();
    const piBody = document.getElementById('piBody');
    const piFindings = report.prompt_injection?.findings || [];
    piBody.innerHTML = piFindings.length > 0
        ? piFindings.map(f => `<div>⚠ ${f.pattern_name}: <em>${f.matched_text}</em></div>`).join('')
        : '✓ No injection patterns detected';

    // Homoglyph
    const hgRisk = document.getElementById('hgRisk');
    const mixedScripts = report.homoglyph?.mixed_scripts || false;
    hgRisk.textContent = mixedScripts ? 'MIXED' : 'CLEAN';
    hgRisk.className = 'risk-badge ' + (mixedScripts ? 'risk-medium' : 'risk-clean');
    const hgBody = document.getElementById('hgBody');
    const confusables = report.homoglyph?.confusable_chars || [];
    hgBody.innerHTML = confusables.length > 0
        ? `${confusables.length} confusable characters found`
        : '✓ No homoglyph attacks detected';

    // Steganography
    const stRisk = document.getElementById('stRisk');
    const hiddenCount = report.steganography?.hidden_chars_found || 0;
    stRisk.textContent = hiddenCount > 0 ? `${hiddenCount} FOUND` : 'CLEAN';
    stRisk.className = 'risk-badge ' + (hiddenCount > 0 ? 'risk-high' : 'risk-clean');
    document.getElementById('stBody').innerHTML = hiddenCount > 0
        ? `Found ${hiddenCount} hidden characters`
        : '✓ No steganographic content';

    // Entropy
    const enValue = document.getElementById('enValue');
    const entropy = report.entropy?.overall_entropy || 0;
    enValue.textContent = `${entropy.toFixed(3)} bits/char`;
    enValue.className = 'entropy-value ' + (entropy > 4.5 ? 'risk-high' : entropy > 3.5 ? 'risk-medium' : 'risk-clean');
    document.getElementById('enBody').textContent = report.entropy?.assessment || '';

    // Unicode
    const ucRisk = document.getElementById('ucRisk');
    const ucHidden = report.unicode?.hidden_chars || 0;
    ucRisk.textContent = ucHidden > 0 ? `${ucHidden} HIDDEN` : 'CLEAN';
    ucRisk.className = 'risk-badge ' + (ucHidden > 0 ? 'risk-medium' : 'risk-clean');
    document.getElementById('ucBody').innerHTML = ucHidden > 0
        ? (report.unicode?.findings || []).slice(0, 5).map(f => `U+${f.codepoint.toString(16).toUpperCase().padStart(4,'0')} [${f.code}]`).join('<br>')
        : '✓ No hidden Unicode';

    toast('Analysis complete', 'success');
}

function resetAnalysis() {
    document.getElementById('piRisk').textContent = '—';
    document.getElementById('piRisk').className = 'risk-badge risk-clean';
    document.getElementById('piBody').textContent = 'Enter text to scan';
    document.getElementById('hgRisk').textContent = '—';
    document.getElementById('hgRisk').className = 'risk-badge risk-clean';
    document.getElementById('hgBody').textContent = 'Enter text to scan';
    document.getElementById('stRisk').textContent = '—';
    document.getElementById('stRisk').className = 'risk-badge risk-clean';
    document.getElementById('stBody').textContent = 'Enter text to scan';
    document.getElementById('enValue').textContent = '—';
    document.getElementById('enValue').className = 'entropy-value risk-clean';
    document.getElementById('enBody').textContent = 'Enter text to scan';
    document.getElementById('ucRisk').textContent = '—';
    document.getElementById('ucRisk').className = 'risk-badge risk-clean';
    document.getElementById('ucBody').textContent = 'Enter text to scan';
}

// Debounced auto-analysis
let analysisTimer = null;
function scheduleAnalysis() {
    clearTimeout(analysisTimer);
    const input = document.getElementById('inputText').value;
    if (!input.trim()) { resetAnalysis(); return; }
    analysisTimer = setTimeout(() => runAnalysis(), 500);
}

// ═══ Search ═══
function setupSearch() {
    const input = document.getElementById('searchInput');
    const results = document.getElementById('searchResults');

    input.addEventListener('input', () => {
        const q = input.value.toLowerCase().trim();
        if (q.length < 1) { results.classList.add('hidden'); return; }
        const matches = state.transforms.filter(t =>
            t.key.includes(q) || t.name.toLowerCase().includes(q) || t.description.toLowerCase().includes(q) || t.category.toLowerCase().includes(q)
        ).slice(0, 12);
        if (matches.length === 0) { results.classList.add('hidden'); return; }
        results.innerHTML = matches.map(t => `
            <div class="search-result-item" data-key="${t.key}">
                <span>${t.name} <span class="sr-key">${t.key}</span></span>
                <span class="sr-cat">${t.category}</span>
            </div>
        `).join('');
        results.classList.remove('hidden');
        results.querySelectorAll('.search-result-item').forEach(item => {
            item.addEventListener('click', () => {
                selectTransform(item.dataset.key);
                input.value = '';
                results.classList.add('hidden');
            });
        });
    });

    document.addEventListener('click', (e) => {
        if (!e.target.closest('.search-box')) results.classList.add('hidden');
    });
}

// ═══ Stats ═══
function updateStats() {
    const inputEl = document.getElementById('inputText');
    const outputEl = document.getElementById('outputText');
    const inputVal = inputEl.value;
    const outputVal = outputEl.value;
    document.getElementById('inputStats').textContent =
        `Chars: ${inputVal.length} | Words: ${inputVal.split(/\s+/).filter(w=>w).length}`;

    if (outputVal && state.wasmReady) {
        const est = JSON.parse(estimate_tokens(outputVal));
        document.getElementById('outputStats').textContent =
            `Chars: ${outputVal.length} | GPT-4: ~${est.gpt4} | Claude: ~${est.claude}`;
    } else {
        document.getElementById('outputStats').textContent =
            `Chars: ${outputVal.length} | Tokens: ~0`;
    }
}

// ═══ Events ═══
function bindEvents() {
    // Core actions
    document.getElementById('encodeBtn').addEventListener('click', doEncode);
    document.getElementById('decodeBtn').addEventListener('click', doDecode);
    document.getElementById('swapBtn').addEventListener('click', () => {
        const i = document.getElementById('inputText');
        const o = document.getElementById('outputText');
        const tmp = i.value; i.value = o.value; o.value = tmp;
        updateStats();
    });

    // Pipeline
    document.getElementById('pipelineRunBtn').addEventListener('click', () => runPipelineAction(false));
    document.getElementById('pipelineReverseBtn').addEventListener('click', () => runPipelineAction(true));
    document.getElementById('pipelineClearBtn').addEventListener('click', () => {
        state.pipeline = [];
        renderPipeline();
    });

    // Analysis
    document.getElementById('analyzeBtn').addEventListener('click', runAnalysis);
    document.getElementById('exportReportBtn').addEventListener('click', () => {
        const input = document.getElementById('inputText').value;
        if (!input) return;
        const report = analyze_all(input);
        const blob = new Blob([report], { type: 'application/json' });
        const a = document.createElement('a');
        a.href = URL.createObjectURL(blob);
        a.download = 'triolingo_report.json';
        a.click();
    });

    // Clipboard
    document.getElementById('copyBtn').addEventListener('click', () => {
        navigator.clipboard.writeText(document.getElementById('outputText').value);
        toast('Copied to clipboard', 'success');
    });
    document.getElementById('pasteBtn').addEventListener('click', async () => {
        const text = await navigator.clipboard.readText();
        document.getElementById('inputText').value = text;
        updateStats();
    });

    // Clear & download
    document.getElementById('clearInputBtn').addEventListener('click', () => {
        document.getElementById('inputText').value = '';
        updateStats();
    });
    document.getElementById('downloadBtn').addEventListener('click', () => {
        const text = document.getElementById('outputText').value;
        if (!text) return;
        const blob = new Blob([text], { type: 'text/plain' });
        const a = document.createElement('a');
        a.href = URL.createObjectURL(blob);
        a.download = 'triolingo_output.txt';
        a.click();
    });

    // Params close
    document.getElementById('closeParamBtn').addEventListener('click', () => {
        document.getElementById('paramPanel').classList.add('hidden');
    });

    // Live stats + auto-analysis
    document.getElementById('inputText').addEventListener('input', () => {
        updateStats();
        scheduleAnalysis();
    });

    // Search
    setupSearch();

    // Mobile sidebar toggle
    const sidebar = document.getElementById('sidebar');
    const overlay = document.getElementById('sidebarOverlay');
    const toggle = document.getElementById('sidebarToggle');
    if (toggle) {
        toggle.addEventListener('click', () => {
            sidebar.classList.toggle('open');
            overlay.classList.toggle('active');
        });
    }
    if (overlay) {
        overlay.addEventListener('click', () => {
            sidebar.classList.remove('open');
            overlay.classList.remove('active');
        });
    }
    // Close sidebar on transform select (mobile)
    document.querySelectorAll('.transform-item').forEach(item => {
        item.addEventListener('click', () => {
            if (window.innerWidth <= 768) {
                sidebar.classList.remove('open');
                overlay.classList.remove('active');
            }
        });
    });

    // Keyboard shortcuts
    document.addEventListener('keydown', (e) => {
        if (e.ctrlKey && e.key === 'Enter') { e.preventDefault(); doEncode(); }
        if (e.ctrlKey && e.shiftKey && e.key === 'Enter') { e.preventDefault(); doDecode(); }
        if (e.key === '/' && e.target.tagName !== 'INPUT' && e.target.tagName !== 'TEXTAREA') {
            e.preventDefault();
            document.getElementById('searchInput').focus();
        }
    });

    // File drop
    const dropZone = document.getElementById('inputPanel');
    dropZone.addEventListener('dragover', (e) => { e.preventDefault(); dropZone.style.borderColor = 'var(--accent-cyan)'; });
    dropZone.addEventListener('dragleave', () => { dropZone.style.borderColor = ''; });
    dropZone.addEventListener('drop', (e) => {
        e.preventDefault();
        dropZone.style.borderColor = '';
        const file = e.dataTransfer.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onload = () => {
                document.getElementById('inputText').value = reader.result;
                updateStats();
                toast(`Loaded ${file.name}`, 'success');
            };
            reader.readAsText(file);
        }
    });
}

// ═══ UI Helpers ═══
function showLoading(show) {
    let overlay = document.querySelector('.loading-overlay');
    if (show && !overlay) {
        overlay = document.createElement('div');
        overlay.className = 'loading-overlay';
        overlay.innerHTML = '<div class="spinner"></div><div class="loading-text">Loading WASM module...</div>';
        document.body.appendChild(overlay);
    } else if (!show && overlay) {
        overlay.remove();
    }
}

function toast(msg, type = 'info') {
    const container = document.getElementById('toastContainer');
    const el = document.createElement('div');
    el.className = `toast ${type}`;
    el.textContent = msg;
    container.appendChild(el);
    setTimeout(() => { el.style.opacity = '0'; setTimeout(() => el.remove(), 300); }, 3000);
}

// ═══ Boot ═══
boot();
