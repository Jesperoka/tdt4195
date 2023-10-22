import numpy as np

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
def toeplitz_convolve_im(
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
    # Assuming odd kernel size (K x K) that is less than image size (H x W)
    (h, w, _) = im.shape
    n = kernel.shape[0] # we assume square kernel
    toeplitz_matrices = [toeplitz_zero_pad(kernel[i,:], w) for i in range(n)]

    # This is why this implementation is bad
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

    return np.dstack((r, g, b))
