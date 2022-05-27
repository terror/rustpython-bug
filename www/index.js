import init, {run} from '/rustpython_bug.js';

await init('/rustpython_bug_bg.wasm');

run();
