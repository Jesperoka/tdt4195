import pathlib

import matplotlib.pyplot as plt

from utils import read_im, save_im


def greyscale(im):
    """ Converts an RGB image to greyscale

    Args:
        im ([type]): [np.array of shape [H, W, 3]]

    Returns:
        im ([type]): [np.array of shape [H, W]]
    """
    return 0.212 * im[:, :, 0] + 0.7152 * im[:, :, 1] + 0.0722 * im[:, :, 2]


def inverse(im):
    """ Finds the inverse of the greyscale image

    Args:
        im ([type]): [np.array of shape [H, W]]

    Returns:
        im ([type]): [np.array of shape [H, W]]
    """
    return 255 - im


if __name__ == "__main__":
    # Hopefully the automatic tests don't rely on random globals
    output_dir = pathlib.Path("image_solutions")
    output_dir.mkdir(exist_ok=True)
    im = read_im(pathlib.Path("images", "duck.jpeg"))
    plt.imshow(im)

    # Test task 2a
    im_greyscale = greyscale(im)
    save_im(output_dir.joinpath("duck_greyscale.jpeg"), im_greyscale, cmap="gray")
    plt.imshow(im_greyscale, cmap="gray")

    # Test task 2b
    im_inverse = inverse(im_greyscale)
    save_im(output_dir.joinpath("duck_inverse.jpeg"), im_inverse, cmap="gray")
    plt.imshow(im_inverse, cmap="gray")
