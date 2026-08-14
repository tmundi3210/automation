# CALIBRATION REPORT · fit 2026-06-21 · n=34 scored predictions · window: last 60d

| band | n  | predicted | observed_hit | gap   | verdict |
|------|----|-----------|--------------|-------|---------|
| 50   | 9  | 0.50      | 0.56         | +0.06 | OK |
| 60   | 8  | 0.60      | 0.63         | +0.03 | OK |
| 70   | 7  | 0.70      | 0.71         | +0.01 | OK (well-calibrated) |
| 80   | 6  | 0.80      | 0.67         | -0.13 | OVERCONFIDENT → shrink 80→72 |
| 90   | 4  | 0.90      | 0.75         | -0.15 | OVERCONFIDENT → shrink 90→78 (n<min, low trust) |

Overall Brier = 0.19. Diagnosis: well-calibrated at 50–70, overconfident at 80–90.
Action: wrote recalibration_map.json recal-v2 (shrink high bands). Bands with n < min_band_n(8) noted as low-trust, adjusted conservatively. Next recalibration: 2026-07-12 (wk_023).
