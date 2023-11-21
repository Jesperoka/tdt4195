import numpy as np
from numpy.typing import NDArray
from skimage.morphology import binary_closing, binary_dilation, binary_opening

import utils


def remove_noise(im: np.ndarray,
                 num_morphologies: int = 6,
                 structure_dilator: NDArray[np.bool_] = np.ones((9, 9), dtype=np.bool_),
                 closing_first: bool = False) -> np.ndarray:
    """
        A function that removes noise in the input image.
        args:
            im: np.ndarray of shape (H, W) with boolean values (dtype=bool)
        return:
            (np.ndarray) of shape (H, W). dtype=bool
    """
    first_morphology, second_morphology = (binary_closing, binary_opening) if closing_first else (binary_opening,
                                                                                                  binary_closing)

    structuring_element = np.zeros((2*num_morphologies + 1, 2*num_morphologies + 1))
    n, m = structuring_element.shape
    structuring_element[n // 2, m // 2] = 1

    print("Performing morphological operations:")
    for i in range(num_morphologies):
        structuring_element = binary_dilation(structuring_element, structure_dilator)
        im = second_morphology(first_morphology(im, structuring_element), structuring_element)
        print(i + 1, "/", num_morphologies)

    return im


shape_informed_structure_dilator = np.array([[1, 1, 1], 
                                             [1, 1, 0], 
                                             [1, 0, 0]])

if __name__ == "__main__":
    # DO NOT CHANGE
    im = utils.read_image("noisy.png")

    binary_image = (im != 0)
    noise_free_image = remove_noise(binary_image)
    # noise_free_image = remove_noise(binary_image, num_morphologies=8, structure_dilator=shape_informed_structure_dilator)

    assert im.shape == noise_free_image.shape, "Expected image shape ({}) to be same as resulting image shape ({})".format(
        im.shape, noise_free_image.shape)
    assert noise_free_image.dtype == bool, "Expected resulting image dtype to be bool. Was: {}".format(
        noise_free_image.dtype)

    noise_free_image = utils.to_uint8(noise_free_image)
    utils.save_im("noisy-filtered.png", noise_free_image)
