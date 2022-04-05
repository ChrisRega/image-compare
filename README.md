# image-compare
image comparison in rust based on the image crate

This package is heavily work-in-progress, so don't API stability just yet.

Supported:
- Comparing grayscale images
  - By RMS - score is calculated by: <img src="https://render.githubusercontent.com/render/math?math=1-\sqrt{\frac{(\sum_{x,y=0}^{x,y=w,h}\left(f(x,y)-g(x,y)\right)^2)}{w*h}}"> 
  - By MSSIM
    - SSIM is implemented as on wikipedia (https://en.wikipedia.org/wiki/Structural_similarity)
    - MSSIM is calculated by using 8x8 px windows and averaging over the results
- Grayscale operations are currently unoptimized simplistic CPU implementations