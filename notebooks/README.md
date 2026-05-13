# Phase 0 notebooks

Math validation for the Kanerva substrate. These notebooks empirically verify the three load-bearing properties of HDC/SDM before any production code is written.

## Setup

```bash
python -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt
jupyter notebook
```

## Notebooks

| File | What it validates |
|---|---|
| `phase0_hdc_validation.ipynb` | The three properties: fuzzy recall, holographic degradation, compositional bind/unbind. Pinned random seeds; reproducible. |

## CI

`phase0_hdc_validation.ipynb` is executed end-to-end by the `python` job in `.github/workflows/ci.yml`. Breaking the math breaks CI.
