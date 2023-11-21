import utils
import numpy as np

from skimage.morphology import binary_erosion

def extract_boundary(im: np.ndarray) -> np.ndarray:
    """
        A function that extracts the inner boundary from a boolean image.

        args:
            im: np.ndarray of shape (H, W) with boolean values (dtype=np.bool)
        return:
            (np.ndarray) of shape (H, W). dtype=np.bool
    """
    structuring_element = np.array([
        [1, 1, 1],
        [1, 1, 1],
        [1, 1, 1]
    ], dtype=bool)
    boundary = im & ~binary_erosion(im, structuring_element)
    return boundary


if __name__ == "__main__":
    im = utils.read_image("blood-vessels.png")
    binary_image = (im != 0)
    boundary = extract_boundary(binary_image)

    assert im.shape == boundary.shape, "Expected image shape ({}) to be same as resulting image shape ({})".format(
        im.shape, boundary.shape)
    assert boundary.dtype == bool, "Expected resulting image dtype to be bool. Was: {}".format(
        boundary.dtype)

    boundary = utils.to_uint8(boundary)
    utils.save_im("blood-vessels-boundary.png", boundary)
