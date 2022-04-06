# image-compare
Simple image comparison in rust based on the image crate

Note that this crate is heavily work in progress. Algorithms are neither cross-checked not particularly fast yet.
Everything is implemented in plain CPU with no SIMD or GPU usage.

Supported now:
- Comparing grayscale images
  - By RMS - score is calculated by: <img src="https://render.githubusercontent.com/render/math?math=1-\sqrt{\frac{(\sum_{x,y=0}^{x,y=w,h}\left(f(x,y)-g(x,y)\right)^2)}{w*h}}"> 
  - By MSSIM
    - SSIM is implemented as described on [wikipedia](https://en.wikipedia.org/wiki/Structural_similarity): <img src="https://render.githubusercontent.com/render/math?math=\mathrm{SSIM}(x,y)={\frac {(2\mu _{x}\mu _{y}+c_{1})(2\sigma _{xy}+c_{2})}{(\mu _{x}^{2}+\mu _{y}^{2}+c_{1})(\sigma _{x}^{2}+\sigma _{y}^{2}+c_{2})}}"> 
    - MSSIM is calculated by using 8x8 pixel windows for SSIM and averaging over the results