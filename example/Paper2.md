# Paper 2: Transformer Networks for NLP

## Model

This paper presents the Transformer architecture, a novel approach based entirely on attention mechanisms.
The model eliminates recurrence and convolutions, relying solely on self-attention.

Key features:
- Multi-head attention mechanism
- Positional encoding
- Encoder-decoder architecture
- 6 layers in encoder and decoder

## Data

Training corpus details:
- WMT 2014 English-German dataset
- 4.5 million sentence pairs
- Byte-pair encoding tokenization
- 37,000 token vocabulary

## Metrics

Performance results:
- BLEU score: 28.4 (English-German)
- BLEU score: 41.8 (English-French)
- Training time: 3.5 days on 8 P100 GPUs
- 65M parameters

## Evaluation

Evaluation methodology:
- Newstest2014 benchmark
- Comparison with RNN and CNN baselines
- Ablation studies on attention heads
- Analysis of attention patterns

The Transformer outperforms previous models while being more parallelizable.

## Limitations

- Quadratic complexity with sequence length
- Struggles with very long sequences
- Limited inductive bias compared to CNNs
- Requires large amounts of training data
- No inherent notion of order (needs positional encoding)