const invoke = window.__TAURI__.invoke;

export async function setWindowDecorations(decorations) {
  await invoke('set_window_decorations', { decorations });
}
