# reversi-ai

## クライアントとして動かす
```sh
cargo run -h "localhost" -p 3000 -n Player1
```

## AI同士で対戦
```sh
cargo run -debug [AI1] [AI2]
```

## プレイヤーがAIと対戦
```sh
cargo run -debug p s [AI]
```
- s $\cdots プレイヤーが先手

## AIの名前
- rn $\cdots$ random_pos
- ab $\cdots$ alpha_beta_pos
- na $\cdots$ nega_alpha_transpose_pos
- ns $\cdots$ nega_scout_transpose_pos
