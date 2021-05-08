# メモ
## グラフ + 最短距離
### 1点からの距離のみで良い
#### ダイクストラ
- 負の辺があると使えない
- ref: https://github.com/rchaser53/at-corder-memo/blob/master/src/abc12d.rs

### 複数の点から
#### ワーシャルフロイド
- 負の辺があっても使える
- 負の閉路があると使えない
- O(V^3)になる。遅い
- ref: https://github.com/rchaser53/at-corder-memo/blob/master/src/abc12d.rs

#### ベルマンフォード < ダイクストラ？
- 正の重みのみならダイクストラの方が早い
- 負の重みの検出に使える
- 負の重みがあっても処理ができる
  - ベルマンフォードの場合、正の重みのみなら最悪ケースでもV-1回のループで終了する
  - 負の重みがある場合、値が更新され続けるので延々と処理が終了しない
    V回以上、実行された場合に負の重みがあるなどの手段で検出ができる

## 点更新、区間取得
### Fenwick tree (Binary indexed tree, BIT)
**セグメントツリーを使った方が無難そう**
- できること
  - 要素の追加
  - 特定区間の値の更新
  - 特定区間の値の和の取得

- 備考
  - SegmentTreeよりも使用メモリ量が少なく、実装が楽、その分できないこともある
    - 0番目からx番目までの区間和などが求められるので、累積和のノリで区間和は求められる
      - つまり最大値、最小値とかは駄目なはず
      - セグメント木コピペすれば良いから別に使う必要ないのでは…？
  
- ref(セグメント木でrewrite済み)
  - abc185f.rs
  - abc190f.rs (転倒数)
    - https://scrapbox.io/pocala-kyopro/%E8%BB%A2%E5%80%92%E6%95%B0  


arc39b.rs
arc39b.rsのinvの解説
- https://drken1215.hatenablog.com/entry/2018/06/08/210000
- https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a


#### abc193e.rs
特定のスパンで状態が変化する。(PとQ, XとY)
最短で重複するのはいつか？重複しない場合もある

X <= t mod(2X+2Y) < X+Y
P <= t mod(P+Q) < P+Q

X <= t1 < X+Y
P <= t2 < P+Q

t = t1(mod 2X+2Y)
t = t2(mod P+Q)
となる最小の非負整数tを求める(中国剰余定理)

ref: https://atcoder.jp/contests/abc193/editorial/812

中国剰余定理
https://qiita.com/drken/items/ae02240cd1f8edfc86fd

ユークリッドの互除法の拡張
https://qiita.com/drken/items/b97ff231e43bce50199a

ax + by = c が整数解を持つ条件
cがgcd(a, b)で割り切れること

