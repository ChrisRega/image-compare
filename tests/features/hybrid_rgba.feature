Feature: RGBA image comparison using hybrid mode - MSSIM for for Y, Alpha channels, RMS for U and V

  Scenario: Comparing an image to the original with hybrid mode and checking the difference image
    Given the images 'tests/data/colored_primitives.png' and 'tests/data/colored_primitives_alpha.png' are loaded
    When comparing the images using the hybrid mode as rgba
    Then the rgba similarity image matches 'tests/data/colored_primitives_hybrid_compare_rgb.png'


  Scenario Outline: Comparing a modified image to the original using hybrid mode algorithm
    Given the images 'tests/data/pad_gaprao.png' and '<compare_image>' are loaded
    When comparing the images using the hybrid mode as rgba
    Then the similarity score is <result>

    Examples:
      | compare_image                           | result                |
      | tests/data/pad_gaprao.png               | 1.0                   |
      | tests/data/pad_gaprao_lighter.png       | 0.948974609375        |
      | tests/data/pad_gaprao_noise.png         | 0.13009302571614584   |
      | tests/data/pad_gaprao_gray_inverted.png | 3.256663878758748e-5  |
      | tests/data/pad_gaprao_color_filters.png | 0.9884529947916667    |
      | tests/data/pad_gaprao_alpha.png         | 0.8729651692708333    |
