import pathlib

import matplotlib.pyplot as plt
import numpy as np

from utils import normalize, read_im, save_im

output_dir = pathlib.Path("image_solutions")
output_dir.mkdir(exist_ok=True)

im = read_im(pathlib.Path("images", "duck.jpeg"))
#plt.imshow(im)


# Square zero padding unit stride toeplitz matrix without using scipy
def toeplitz_zero_pad(row, img_w): 
    _dtype = row.dtype
    n = max(row.shape)

    # Toeplitz matrix is padded with zeros so that the matrix equals the image width
    toeplitz_matrix = np.zeros((img_w, img_w), dtype=_dtype)
    
    np.fill_diagonal(toeplitz_matrix, row[int(n/2)])                 # fill main diagonal
    for i in range(1, int(n/2 + 1)):
        np.fill_diagonal(toeplitz_matrix[i:img_w, 0:img_w-i], row[int(n/2-i)])        # fill subdiagonals
        np.fill_diagonal(toeplitz_matrix[0:img_w-i, i:img_w], row[int(n/2+i)])        # fill superdiagonals
    
    return toeplitz_matrix
    # return scipy.linalg.toeplitz(np.concatenate(row[:-1], np.zeros(img_w)), np.concatenate(row[1:], np.zeros(img_w)))


# Square doubly block-toeplitz matrix
def block_toeplitz_zero_pad(toeplitz_matrices, flat_img_length):
    _dtype = toeplitz_matrices[0].dtype
    m = len(toeplitz_matrices)
    n = toeplitz_matrices[0].shape[0] # assumes square matrix
    k = int(np.ceil(flat_img_length / n))

    block_toeplitz_matrix = np.kron(np.eye(k), toeplitz_matrices[int(m/2)]) # fill main block diagonal
    for i in range(1, int(m/2 + 1)):
        # Create lower and upper shift matrices 
        lower = np.zeros((k, k), dtype=_dtype)
        upper = np.zeros((k, k), dtype=_dtype)
        np.fill_diagonal(lower[i:k, 0:k-i], 1)
        np.fill_diagonal(upper[0:k-i, i:k], 1)

        # Add block diagonals to block_toeplitz_matrix
        block_toeplitz_matrix += np.kron(lower, toeplitz_matrices[int(m/2-i)]) + np.kron(upper, toeplitz_matrices[int(m/2+i)]) # fill shift block diagonals

    return block_toeplitz_matrix[0:flat_img_length, 0:flat_img_length]


# Convolution through matrix multiplication with a contructed Toeplitz matrix for zero-padding and unit stride
def my_convolve_im(
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

    transpose = False
    # if im.shape[1] >= im.shape[0]: 
    #     transpose = True
    #     im = np.transpose(im, axes=(1, 0, 2))

    # Assuming odd kernel size (K x K) that is less than image size (H x W)
    (h, w, _) = im.shape
    n = kernel.shape[0] # we assume square kernel
    toeplitz_matrices = [toeplitz_zero_pad(kernel[i,:], w) if not transpose else toeplitz_zero_pad(kernel[i,:], w).T  for i in range(n)]

    # TODO: transpose image such that width is smallest dim

    overlap = int(n/2)*w 
    seg_size = 5*n*w 
    assert(seg_size >= n*w and overlap == int(n/2)*w)
    im_vec_r = im[:,:,0].reshape((h*w, 1))
    im_vec_g = im[:,:,1].reshape((h*w, 1))
    im_vec_b = im[:,:,2].reshape((h*w, 1))
    r = []
    g = []
    b = []
    
    idx = 0
    while idx + seg_size < h*w:
        print(idx,"/",h*w)
        # First segment
        block_toeplitz_matrix = block_toeplitz_zero_pad(toeplitz_matrices, seg_size)
        if idx == 0: # first iteration
            r.append((block_toeplitz_matrix @ im_vec_r[idx:idx+seg_size])[:-overlap])
            g.append((block_toeplitz_matrix @ im_vec_g[idx:idx+seg_size])[:-overlap])
            b.append((block_toeplitz_matrix @ im_vec_b[idx:idx+seg_size])[:-overlap])
            idx += seg_size - overlap 

        # All middle segments # TODO: assign to vector slice
        r.append((block_toeplitz_matrix @ im_vec_r[idx:idx+seg_size])[overlap:-overlap])
        g.append((block_toeplitz_matrix @ im_vec_g[idx:idx+seg_size])[overlap:-overlap])
        b.append((block_toeplitz_matrix @ im_vec_b[idx:idx+seg_size])[overlap:-overlap])
        idx += seg_size - 2*overlap 

    # Last segment
    else:  
        seg_size = h*w - idx
        block_toeplitz_matrix = block_toeplitz_zero_pad(toeplitz_matrices, seg_size)
        r.append(block_toeplitz_matrix @ im_vec_r[idx:idx+seg_size])
        g.append(block_toeplitz_matrix @ im_vec_g[idx:idx+seg_size])
        b.append(block_toeplitz_matrix @ im_vec_b[idx:idx+seg_size])

    r = np.concatenate(r, axis=0).reshape((h, w))
    g = np.concatenate(g, axis=0).reshape((h, w))
    b = np.concatenate(b, axis=0).reshape((h, w))

    return np.dstack((r, g, b)) if not transpose else np.transpose(np.dstack((r, g, b)), axes=(1, 0, 2))

# The dumbest possible (edit: now not so sure my other one isn't dumber) implementation of 2D convolution, just to test correctness
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
                
    assert convolved_im.shape == im.shape
    return convolved_im


if __name__ == "__main__":
    # Define the convolutional kernels
    h_b = 1 / 256 * np.array([[1, 4, 6, 4, 1], [4, 16, 24, 16, 4], [6, 24, 36, 24, 6], [4, 16, 24, 16, 4],
                              [1, 4, 6, 4, 1]])
    sobel_x = np.array([[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]])

    # Convolve images
    im_smoothed = my_convolve_im(im.copy(), h_b)
    save_im(output_dir.joinpath("im_smoothed.jpg"), im_smoothed)
    im_sobel = my_convolve_im(im, sobel_x)
    save_im(output_dir.joinpath("im_sobel.jpg"), im_sobel)

    # input("wait")
    
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
