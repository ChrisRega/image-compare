# image-compare
Simple image comparison in rust based on the image crate

Note that this crate is heavily work in progress. Algorithms are neither cross-checked not particularly fast yet.
Everything is implemented in plain CPU with no SIMD or GPU usage.

Supported now:
- Comparing grayscale images
  - By RMS - score is calculated by: <img src="https://render.githubusercontent.com/render/math?math=1-\sqrt{\frac{(\sum_{x,y=0}^{x,y=w,h}\left(f(x,y)-g(x,y)\right)^2)}{w*h}}"> 
  - By MSSIM
    - SSIM is implemented as described on [wikipedia](https://en.wikipedia.org/wiki/Structural_similarity).
    - MSSIM is calculated by using 8x8 pixel windows for SSIM and averaging over the results