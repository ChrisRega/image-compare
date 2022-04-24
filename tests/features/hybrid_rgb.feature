Feature: RGB image comparison using hybrid mode - MSSIM for for Y channel, RMS for U and V

  Scenario: Comparing an image to the original with hybrid mode and checking the difference image
  Given the images 'tests/data/colored_primitives.png' and 'tests/data/colored_primitives_swapped.png' are loaded
  When comparing the images using the hybrid mode as rgb
  Then the rgb similarity image matches 'tests/data/colored_primitives_hybrid_compare_rgb.png'


Scenario Outline: Comparing a modified image to the original using hybrid mode algorithm
Given the images 'tests/data/pad_gaprao.png' and '<compare_image>' are loaded
When comparing the images using the hybrid mode as rgb
Then the similarity score is <result>

Examples:
| compare_image                           | result                |
| tests/data/pad_gaprao.png               | 1.0                   |
| tests/data/pad_gaprao_lighter.png       | 0.8235626302083333    |
| tests/data/pad_gaprao_noise.png         | 0.12519161783854166   |
| tests/data/pad_gaprao_gray_inverted.png | 2.7263303597768148e-5 |
| tests/data/pad_gaprao_color_filters.png | 0.664062109375        |