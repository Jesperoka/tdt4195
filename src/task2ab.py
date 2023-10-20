import matplotlib.pyplot as plt
import pathlib
import numpy as np
from utils import read_im, save_im
output_dir = pathlib.Path("image_solutions")
output_dir.mkdir(exist_ok=True)


im = read_im(pathlib.Path("images", "duck.jpeg"))
plt.imshow(im)



def greyscale(im):
    """ Converts an RGB image to greyscale

    Args:
        im ([type]): [np.array of shape [H, W, 3]]

    Returns:
        im ([type]): [np.array of shape [H, W]]
    """
    return 0.212*im[:,:,0] + 0.7152*im[:,:,1] + 0.0722*im[:,:,2]

im_greyscale = greyscale(im)
save_im(output_dir.joinpath("duck_greyscale.jpeg"), im_greyscale, cmap="gray")
plt.imshow(im_greyscale, cmap="gray")


def inverse(im):
    """ Finds the inverse of the greyscale image

    Args:
        im ([type]): [np.array of shape [H, W]]

    Returns:
        im ([type]): [np.array of shape [H, W]]
    """
    return 255 - im

im_inverse = inverse(im_greyscale)
save_im(output_dir.joinpath("duck_greyscale.jpeg"), im_inverse, cmap="gray")
plt.imshow(im_inverse, cmap="gray")
