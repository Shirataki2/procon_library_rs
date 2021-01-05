# Rust Competitive Programming Library

![Test](https://github.com/Shirataki2/procon_library_rs/workflows/Test/badge.svg?branch=main)

## なかみ

- Data Structure
    - Fenwick Tree
        - いわゆるBinary Index Tree
    - Segment Tree
        - いたって普通のセグ木
    - Lazy Segment Tree
        - 遅延セグメント木
    - Skew Heap
        - 結合できる優先度付きキュー
    - Sparse Table
        - 静的な配列なら定数倍高速なやつ
    - Trie
        - トライ木
    - Union Find
        - 素集合データ構造
    - Weighted Union Find
        - 重みが付いたやつ

- Graph
    - Bellman Ford
        - 負の辺がある際の単一始点最短経路
    - Dijkstra
        - ご存知単一始点最短経路
    - Dinic
        - 最大流(増加パスのうち最短のものに流す)
    - Ford Fulkerson
        - 最大流(競プロで経路無理数は出ないやろ...)
    - Kruskal
        - 無向グラフの最小全域木
    - Low Link
        - 橋と関節点
    - Prim
        - 無向グラフの最小全域木 その2
    - Strongly Connected Components
        - 強連結成分分解
    - Topological Sort
        - トポロジカルソート
    - Warshall Floyd
        - ワーシャルフロイド

- Math
    - Complex
        - 複素数
    - Divisor
        - 約数列挙
    - Factorize
        - 素因数分解
    - FFT
        - 高速フーリエ変換
    - FPS
        - 形式的冪級数
    - NTT
        - 数論変換
    - GCD
        - 最小公倍数/最大公約数/拡張ユークリッドの互除法
    - Mod Int
        - 剰余演算を演算子のオーバーロードで定義した整数型
    - Mod Ops
        - 剰余に関する演算

- String
    - Edit Distance
        - 編集距離
    - Rolling Hash
        - ロリハ(M=2^61-1でdが原子根だけどこれでいいのか？)
    - Suffix Array
        - 接尾辞配列(SA-IS)

- Utils
    - Itertools
        - 累積和と組み合わせ(iter拡張)
    - IO
        - 高速な入出力
    - Permutations
        - next_permutation
