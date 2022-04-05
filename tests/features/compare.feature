Feature: Image comparison

  Scenario Outline: Comparing an offset image to the original using RMS algorithm
    Given the images 'tests/data/pad_gaprao.png' and '<compare_image>' are loaded
    When comparing the images using RMS
    Then the similarity score is <result>

    Examples:
      | compare_image                           | result              |
      | tests/data/pad_gaprao.png               | 1.0                 |
      | tests/data/pad_gaprao_lighter.png       | 0.9201704590012584  |
      | tests/data/pad_gaprao_noise.png         | 0.7512383697679271  |
      | tests/data/pad_gaprao_gray_inverted.png | 0.497502556580533   |

  Scenario Outline: Comparing an offset image to the original using MSSIM algorithm
    Given the images 'tests/data/pad_gaprao.png' and '<compare_image>' are loaded
    When comparing the images using MSSIM
    Then the similarity score is <result>

    Examples:
      | compare_image                           | result              |
      | tests/data/pad_gaprao.png               | 1.0                 |
      | tests/data/pad_gaprao_lighter.png       | 0.9468883561804494  |
      | tests/data/pad_gaprao_noise.png         | 0.13375106098371664 |
      | tests/data/pad_gaprao_gray_inverted.png | -0.662610652798606  |

