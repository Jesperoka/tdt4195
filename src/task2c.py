import pathlib

import matplotlib.pyplot as plt
import numpy as np

from utils import normalize, read_im, save_im

output_dir = pathlib.Path("image_solutions")
output_dir.mkdir(exist_ok=True)

im = read_im(pathlib.Path("images", "duck.jpeg"))
plt.imshow(im)


# Square zero padding unit stride toeplitz matrix without using scipy
def toeplitz_zero_pad(row): 
    _dtype = row.dtype
    n = max(row.shape)

    # Padding one zero on either side assumes kernel is smaller than image
    padded_row = np.concatenate([np.zeros(1), row.squeeze(), np.zeros(1)], axis=0)
    toeplitz_matrix = np.zeros((n, n), dtype=_dtype)
    
    np.fill_diagonal(toeplitz_matrix, row[int(n/2)])                 # fill main diagonal
    for i in range(n-1):
        np.fill_diagonal(toeplitz_matrix[n-i-1:n, 0:i+1], padded_row[i])        # fill subdiagonals
        np.fill_diagonal(toeplitz_matrix[0:i+1, n-i-1:n], padded_row[n+1-i])    # fill superdiagonals # TODO: use negative index
    
    print(toeplitz_matrix)
    return toeplitz_matrix
    # return scipy.linalg.toeplitz(np.concatenate(row[:-1], np.zeros(1)), np.concatenate(row[1:], np.zeros(1)))


# Square doubly block-toeplitz matrix
def block_toeplitz_zero_pad(toeplitz_matrices):
    _dtype = toeplitz_matrices[0].dtype
    m = len(toeplitz_matrices)
    n = toeplitz_matrices[0].shape[0] # assumes square matrix

    # Padding one block zero matrix assumes kernel is smaller than image
    toeplitz_matrices.insert(0, np.zeros((n, n)))
    toeplitz_matrices.append(np.zeros((n, n)))

    block_toeplitz_matrix = np.kron(np.eye(m), toeplitz_matrices[int(m/2)]) # fill main block diagonal
    print(block_toeplitz_matrix.shape)
    for i in range(m-1):
        # Create lower and upper shift matrices # WARNING: does this kron work
        lower = np.zeros((m, m), dtype=_dtype)
        upper = np.zeros((m, m), dtype=_dtype)
        np.fill_diagonal(lower[m-i-1:m, 0:i+1], 1)
        np.fill_diagonal(upper[0:i+1, m-i-1:m], 1)
        print(lower)
        print(upper)

        # Add block diagonals to block_toeplitz_matrix
        block_toeplitz_matrix += np.kron(lower, toeplitz_matrices[i]) + np.kron(upper, toeplitz_matrices[-i-1]) # fill shift block diagonals

    return block_toeplitz_matrix


# Convolution through matrix multiplication with a contructed Toeplitz matrix for zero-padding and unit stride
def convolve_im(
    im,
    kernel,
):
    """ A function that convolves im with kernel

    Args:
        im ([type]): [np.array of shape [H, W, 3]]
        kernel ([type]): [np.array of shape [K, K]]

    Returns:
        [type]: [np.array of shape [H, W, 3]. should be same as im]
    """
    assert len(im.shape) == 3

    # Assuming odd kernel size (K x K) that is less than image size (H x W)
    (h, w, _) = im.shape
    n = kernel.shape[0] # we assume square kernel
    toeplitz_matrices = []

    for i in range(n):
        toeplitz_matrices.append(toeplitz_zero_pad(kernel[i,:]))

    r = (block_toeplitz_zero_pad(toeplitz_matrices) @ im[:,:,0].reshape((h*w, 1))).reshape((h, w))
    g = (block_toeplitz_zero_pad(toeplitz_matrices) @ im[:,:,1].reshape((h*w, 1))).reshape((h, w))
    b = (block_toeplitz_zero_pad(toeplitz_matrices) @ im[:,:,2].reshape((h*w, 1))).reshape((h, w))
    return np.concatenate([r, g, b], axis=2)


if __name__ == "__main__":
    # Define the convolutional kernels
    h_b = 1 / 256 * np.array([[1, 4, 6, 4, 1], [4, 16, 24, 16, 4], [6, 24, 36, 24, 6], [4, 16, 24, 16, 4],
                              [1, 4, 6, 4, 1]])
    sobel_x = np.array([[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]])

    # Convolve images
    # im_smoothed = convolve_im(im.copy(), h_b)
    # save_im(output_dir.joinpath("im_smoothed.jpg"), im_smoothed)
    im_sobel = convolve_im(im, sobel_x)
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
