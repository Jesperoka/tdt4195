import pathlib

import matplotlib.pyplot as plt
import numpy as np

from multiprocessing import Pool

from utils import normalize, read_im, save_im, numpy_to_torch_dtype

output_dir = pathlib.Path("image_solutions")
output_dir.mkdir(exist_ok=True)

im = read_im(pathlib.Path("images", "duck.jpeg"))
#plt.imshow(im)


# I spent 2 whole days implementing the matrix multiplication 
# version of 2d convolution, only to realize the block matrix
# you end up with as convolution operator is too large to fit in memory,
# so naturally I spent another half a day making it work on image segments,
# only to realize that overlap needed between segments scales with kernel size
# and image width, so the implementation is slower than just using (python) for loops.
# wtf is wrong with me?

def _toeplitz_convolve_im():
    # Moved to toeplitz.py to not clutter task file
    pass

# The dumbest possible implementation of 2D convolution, just to test correctness
# EDIT: now not so sure my toeplitz one isn't dumber
def convolve_im(im, kernel):
    H, W, _ = im.shape
    K, _ = kernel.shape
    pad_size = K // 2
    padded_im = np.pad(im, ((pad_size, pad_size), (pad_size, pad_size), (0, 0)), mode='constant')
    convolved_im = np.zeros_like(im)

    for y in range(H):
        for x in range(W):
            for ch in range(3): 
                region = padded_im[y:y+K, x:x+K, ch]
                convolved_im[y, x, ch] = np.sum(region * kernel)
                
    # assert convolved_im.shape == im.shape
    return convolved_im

# Here's a little bit of a better way to do it 
def convolve_channel(args):
    channel_data, kernel = args
    H, W = channel_data.shape
    K, _ = kernel.shape
    pad_size = K // 2
    padded_channel = np.pad(channel_data, ((pad_size, pad_size), (pad_size, pad_size)), mode='constant')
    convolved_channel = np.zeros_like(channel_data)

    for y in range(H):
        for x in range(W):
            region = padded_channel[y:y+K, x:x+K]
            convolved_channel[y, x] = np.sum(region * kernel)

    return convolved_channel

def convolve_im_parallel(im, kernel):
    with Pool() as pool:
        channels = [im[:,:,ch] for ch in range(3)]
        args = [(channel, kernel) for channel in channels]
        result_channels = pool.map(convolve_channel, args)
    convolved_im = np.stack(result_channels, axis=2)
    return convolved_im

if __name__ == "__main__":
    # Define the convolutional kernels
    h_b = 1 / 256 * np.array([[1, 4, 6, 4, 1], [4, 16, 24, 16, 4], [6, 24, 36, 24, 6], [4, 16, 24, 16, 4],
                              [1, 4, 6, 4, 1]])
    sobel_x = np.array([[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]])

    # Convolve images
    im_smoothed = convolve_im_parallel(im.copy(), h_b)
    save_im(output_dir.joinpath("im_smoothed.jpg"), im_smoothed)
    im_sobel = convolve_im_parallel(im, sobel_x)
    save_im(output_dir.joinpath("im_sobel.jpg"), im_sobel)

    # DO NOT CHANGE. Checking that your function returns as expected
    assert isinstance(
        im_smoothed,
        np.ndarray), f"Your convolve function has to return a np.array. " + f"Was: {type(im_smoothed)}"
    assert im_smoothed.shape == im.shape,         f"Expected smoothed im ({im_smoothed.shape}" + \
        f"to have same shape as im ({im.shape})"
    assert im_sobel.shape == im.shape,         f"Expected smoothed im ({im_sobel.shape}" + \
        f"to have same shape as im ({im.shape})"
    plt.subplot(1, 2, 1)
    plt.imshow(normalize(im_smoothed))

    plt.subplot(1, 2, 2)
    plt.imshow(normalize(im_sobel))
    plt.show()
