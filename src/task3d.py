import utils
import numpy as np

from skimage import io
from skimage.morphology import binary_dilation


def fill_holes(im: np.ndarray, starting_points: list, num_iterations: int) -> np.ndarray:
    """
        A function that takes a binary image (im),  and a set of points 
        indicating position of holes, and fills the holes.

        args:
            im: np.ndarray of shape (H, W) with boolean values (dtype=np.bool)
            starting_points: list of list containing starting points (row, col). Ex:
                [[row1, col1], [row2, col2], ...]
            num_iterations: integer defining the number of iterations to apply the 
                            hole filling algorithm
        return:
            (np.ndarray) of shape (H, W). dtype=np.bool
    """
    B = np.ones((3,3), dtype=np.bool_)
    I_c = np.logical_not(im)

    X_0 = np.zeros_like(im, dtype=im.dtype)
    for row, column in starting_points:
        X_0[row, column] = 1

    X_k = X_0 
    for _ in range(1, num_iterations+1):
        X_k = np.logical_and(binary_dilation(X_k, B), I_c)

    return np.logical_or(X_k, im) 


if __name__ == "__main__":
    im = io.imread('images/cards.png', as_gray=True)
    binary_image = im != 0
    starting_points = [
        # (row, column)
        [50, 80],
        [275, 80],
        [50, 175],
        [275, 175],
        [50, 390],
        [275, 390],
        [175, 650]
    ]
    num_iterations = 50

    result = fill_holes(binary_image, starting_points, num_iterations)

    assert im.shape == result.shape, "Expected image shape ({}) to be same as resulting image shape ({})".format(
        im.shape, result.shape)
    assert result.dtype == bool, "Expected resulting image dtype to be bool. Was: {}".format(
        result.dtype)

    result = utils.to_uint8(result)
    utils.save_im("cards-filled.png", result)
