# deploy/kubeedge/

間欠接続環境（エッジサイト）向けプレースホルダ。CloudCore（クラウド側・スケジューリング決定）と
EdgeCore（エッジ側・接続断時のローカル自律性維持）の分離モデルを構成する。

現時点ではDockerスコープのため未着手。`deploy/k8s/` でのcontainerd+runwasi構成が固まった後に着手する。
