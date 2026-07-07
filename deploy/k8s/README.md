# deploy/k8s/

将来のKubernetesターゲット向けプレースホルダ。containerd + runwasi を用いた
`runtimeClassName: wasm` のPodスケジューリング（Kube-Wasmパターン）を構成する。

- `runtimeclass.yaml` — WASMランタイム（Wasmtime/WasmEdge/WAMR）へのルーティング定義（未作成）。
- マニフェスト一式（Deployment/Service等）は `deploy/docker/` での動作確認後に追加する。

現時点ではDockerスコープのため未着手。
