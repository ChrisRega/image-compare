# image-compare
[![Crates.io](https://img.shields.io/crates/d/image-compare?style=flat-square)](https://crates.io/crates/image-compare)
[![Documentation](https://docs.rs/image-compare/badge.svg)](https://docs.rs/image-compare)
![CI](https://github.com/ChrisRega/image-compare/actions/workflows/rust.yml/badge.svg?branch=main "CI")
[![Coverage Status](https://coveralls.io/repos/github/ChrisRega/image-compare/badge.svg?branch=main)](https://coveralls.io/github/ChrisRega/image-compare?branch=main)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat)](LICENSE-MIT)

Simple image comparison in rust based on the image crate

Note that this crate is heavily work in progress. Algorithms are neither cross-checked not particularly fast yet.
Everything is implemented in plain CPU with rayon multithreading. 
SIMD is under investigation on a feature branch (simd-experimental).

### Supported now:
- Comparing grayscale and rgb images by structure
  - By RMS - score is calculated by: <img src="https://render.githubusercontent.com/render/math?math=1-\sqrt{\frac{(\sum_{x,y=0}^{x,y=w,h}\left(f(x,y)-g(x,y)\right)^2)}{w*h}}"> 
  - By MSSIM
    - SSIM is implemented as described on [wikipedia](https://en.wikipedia.org/wiki/Structural_similarity): <img src="https://render.githubusercontent.com/render/math?math=\mathrm{SSIM}(x,y)={\frac {(2\mu _{x}\mu _{y}+c_{1})(2\sigma _{xy}+c_{2})}{(\mu _{x}^{2}+\mu _{y}^{2}+c_{1})(\sigma _{x}^{2}+\sigma _{y}^{2}+c_{2})}}"> 
    - MSSIM is calculated by using 8x8 pixel windows for SSIM and averaging over the results
  - RGB type images are split to R,G and B channels and processed separately. 
    - The worst of the color results is propagated as score but a float-typed RGB image provides access to all values.
    - As you can see in the gherkin tests this result is not worth it currently, as it takes a lot more time
    - It could be improved, by not just propagating the individual color-score results but using the worst for each pixel
    - This approach is implemented in hybrid-mode, see below
  - "hybrid comparison"
    - Splitting the image to YUV colorspace according to T.871
    - Processing the Y channel with MSSIM
    - Comparing U and V channels via RMS
    - Recombining the differences to a nice visualization image
    - Score is calculated as: <img src="https://render.githubusercontent.com/render/math?math=\mathrm{score}=\mathrm{avg}_{x,y}\left(\mathrm{min}\left[\Delta \mathrm{MSSIM}(x,y),1- \sqrt{(1-\Delta RMS(u,x,y))^2 + (1-\Delta RMS(v,x,y))^2}\right]\right)">
    - This allows for a good separation of color differences and structure differences
- Comparing grayscale images by histogram
  - Several distance metrics implemented see [OpenCV docs](https://docs.opencv.org/4.5.5/d8/dc8/tutorial_histogram_comparison.html):
    - Correlation <img src="https://render.githubusercontent.com/render/math?math=d(H_1,H_2) = \frac{\sum_I (H_1(I) - \bar{H_1}) (H_2(I) - \bar{H_2})}{\sqrt{\sum_I(H_1(I) - \bar{H_1})^2 \sum_I(H_2(I) - \bar{H_2})^2}}">
    - Chi-Square <img src="https://render.githubusercontent.com/render/math?math=d(H_1,H_2) = \sum _I \frac{\left(H_1(I)-H_2(I)\right)^2}{H_1(I)}">
    - Intersection <img src="https://render.githubusercontent.com/render/math?math=d(H_1,H_2) = \sum _I \min (H_1(I), H_2(I))">
    - Hellinger distance <img src="https://render.githubusercontent.com/render/math?math=d(H_1,H_2) = \sqrt{1 - \frac{1}{\sqrt{\int{H_1} \int{H_2}}} \sum_I \sqrt{H_1(I) \cdot H_2(I)}}">
