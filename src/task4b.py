from math import ceil
import matplotlib.pyplot as plt
import numpy as np
import skimage
import utils


def convolve_im(im: np.ndarray,
                kernel: np.ndarray,
                verbose=True):
    """ Convolves the image (im) with the spatial kernel (kernel),
        and returns the resulting image.

        "verbose" can be used for turning on/off visualization
        convolution

        Note: kernel can be of different shape than im.

    Args:
        im: np.array of shape [H, W]
        kernel: np.array of shape [K, K] 
        verbose: bool
    Returns:
        im: np.array of shape [H, W]
    """
    # START YOUR CODE HERE ### (You can change anything inside this block)
    n_img, m_img = im.shape
    n_ker, m_ker = kernel.shape
    row_idx_start = ceil((n_ker + 1)/2)
    col_idx_start = ceil((m_ker + 1)/2)
    row_idx_end = row_idx_start + n_img - 1
    col_idx_end = col_idx_start + m_img - 1

    im_fft = np.fft.fft2(im, (n_img+n_ker, m_img+m_ker))
    kernel_fft = np.fft.fft2(kernel, (n_img+n_ker, m_img+m_ker))
    im_fft_conv = np.multiply(im_fft, kernel_fft)
    conv_result = np.real(np.fft.ifft2(im_fft_conv)[row_idx_start:row_idx_end, col_idx_start:col_idx_end]) 

    if verbose:
        # Use plt.subplot to place two or more images beside eachother
        plt.figure(figsize=(20, 4))
        plt.subplot(1, 5, 1)
        plt.title("Img")
        plt.imshow(im, cmap="gray")

        plt.subplot(1, 5, 2)
        plt.title("FFT(Img)")
        plt.imshow(np.log(np.absolute(np.fft.fftshift(im_fft))), cmap="gray") # Visualize FFT

        plt.subplot(1, 5, 3)
        plt.title("FFT(Ker)")
        plt.imshow(np.log(np.absolute(np.fft.fftshift(kernel_fft))), cmap="gray") # Visualize FFT kernel

        plt.subplot(1, 5, 4)
        plt.title("FFT(Img) ⊙ FFT(ker)")
        plt.imshow(np.log(np.absolute(np.fft.fftshift(im_fft_conv))), cmap="gray") # Visualize filtered FFT image

        plt.subplot(1, 5, 5) 
        plt.title("FFT^(-1) ( FFT(Img) ⊙ FFT(ker) )")
        plt.imshow(conv_result, cmap="gray") # Visualize filtered spatial image

    ### END YOUR CODE HERE ###
    return conv_result


if __name__ == "__main__":
    verbose = True  # change if you want

    # Changing this code should not be needed
    im = skimage.data.camera()
    im = utils.uint8_to_float(im)

    # DO NOT CHANGE
    gaussian_kernel = np.array([
        [1, 4, 6, 4, 1],
        [4, 16, 24, 16, 4],
        [6, 24, 36, 24, 6],
        [4, 16, 24, 16, 4],
        [1, 4, 6, 4, 1],
    ]) / 256
    image_gaussian = convolve_im(im, gaussian_kernel, verbose)

    # DO NOT CHANGE
    sobel_horizontal = np.array([
        [-1, 0, 1],
        [-2, 0, 2],
        [-1, 0, 1]
    ])
    image_sobelx = convolve_im(im, sobel_horizontal, verbose)

    if verbose:
        plt.show()

    utils.save_im("camera_gaussian.png", image_gaussian)
    utils.save_im("camera_sobelx.png", image_sobelx)
