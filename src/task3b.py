import utils
import numpy as np
import pathlib

from skimage.morphology import binary_erosion

def distance_transform(im: np.ndarray) -> np.ndarray:
    """
    A function that computes the distance to the closest boundary pixel.

    args:
        im: np.ndarray of shape (H, W) with boolean values (dtype=np.bool)
    return:
        (np.ndarray) of shape (H, W). dtype=np.int32
    """
    assert im.dtype == bool

    im_copy = im.copy()
    structuring_element = np.ones((3, 3), dtype=bool)
    result = np.zeros_like(im, dtype=np.int32) # int64 dtype allows higher value range

    for distance in range(max(im.shape)): 
        result[im_copy] = distance
        im_copy = binary_erosion(im_copy, structuring_element)

    assert result.dtype == np.int32
    return result # truncating conversion to task required dtype



if __name__ == "__main__":
    # <-- This image is created in task3a
    im = utils.read_image("noisy-filtered.png",
                          image_folder=pathlib.Path("image_processed"))
    binary_image = (im != 0)
    distance = distance_transform(binary_image)

    assert im.shape == distance.shape, "Expected image shape ({}) to be same as resulting image shape ({})".format(
        im.shape, distance.shape)
    assert distance.dtype == np.int32, "Expected resulting image dtype to be np.int32. Was: {}".format(
        distance.dtype)

    distance = utils.to_uint8(distance)
    utils.save_im("noisy-distance.png", distance)
