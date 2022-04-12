# image-compare
[![Documentation](https://docs.rs/image-compare/badge.svg)](https://docs.rs/image-compare)
![CI](https://github.com/ChrisRega/image-compare/actions/workflows/rust.yml/badge.svg?branch=main "CI")

Simple image comparison in rust based on the image crate

Note that this crate is heavily work in progress. Algorithms are neither cross-checked not particularly fast yet.
Everything is implemented in plain CPU with no SIMD or GPU usage.

### Supported now:
- Comparing grayscale images by structure
  - By RMS - score is calculated by: <img src="https://render.githubusercontent.com/render/math?math=1-\sqrt{\frac{(\sum_{x,y=0}^{x,y=w,h}\left(f(x,y)-g(x,y)\right)^2)}{w*h}}"> 
  - By MSSIM
    - SSIM is implemented as described on [wikipedia](https://en.wikipedia.org/wiki/Structural_similarity): <img src="https://render.githubusercontent.com/render/math?math=\mathrm{SSIM}(x,y)={\frac {(2\mu _{x}\mu _{y}+c_{1})(2\sigma _{xy}+c_{2})}{(\mu _{x}^{2}+\mu _{y}^{2}+c_{1})(\sigma _{x}^{2}+\sigma _{y}^{2}+c_{2})}}"> 
    - MSSIM is calculated by using 8x8 pixel windows for SSIM and averaging over the results
- Comparing grayscale images by histogram
  - Several distance metrics implemented see [OpenCV docs](https://docs.opencv.org/4.5.5/d8/dc8/tutorial_histogram_comparison.html):
    - Correlation <img src="https://render.githubusercontent.com/render/math?math=d(H_1,H_2) = \frac{\sum_I (H_1(I) - \bar{H_1}) (H_2(I) - \bar{H_2})}{\sqrt{\sum_I(H_1(I) - \bar{H_1})^2 \sum_I(H_2(I) - \bar{H_2})^2}}">
    - Chi-Square <img src="https://render.githubusercontent.com/render/math?math=d(H_1,H_2) = \sum _I \frac{\left(H_1(I)-H_2(I)\right)^2}{H_1(I)}">
    - Intersection <img src="https://render.githubusercontent.com/render/math?math=d(H_1,H_2) = \sum _I \min (H_1(I), H_2(I))">
    - Hellinger distance <img src="https://render.githubusercontent.com/render/math?math=d(H_1,H_2) = \sqrt{1 - \frac{1}{\sqrt{\int{H_1} \int{H_2}}} \sum_I \sqrt{H_1(I) \cdot H_2(I)}}">
     
### Planned:
- Histogram comparison for RGB images
- RMS for RGB images
- SIMD for RMS and MSSIM