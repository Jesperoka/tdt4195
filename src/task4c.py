import skimage
import skimage.io
import skimage.transform
import os
import numpy as np
import utils
import matplotlib.pyplot as plt
from task4b import convolve_im

if __name__ == "__main__":
    # DO NOT CHANGE
    impath = os.path.join("images", "noisy_moon.png")
    im = utils.read_im(impath)

    # Fourier transform 
    f = np.fft.fft2(im)
    magnitude_spectrum_before = 20*np.log(np.abs(np.fft.fftshift(f)))

    # Create masks to manually remove peaks
    top = f[0:3, 3:-(3+1)] 
    bottom = f[-2:, 3:-(3+1)] 

    top_2 = f[0:3, 446:-(3+1)] 
    bottom_2 = f[-2:, 446:-(3+1)] 
    top_3 = f[0:3, 3:13] 
    bottom_3 = f[-2:, 3:13] 

    threshold = 9.0 
    # threshold_2 = 4.0 

    # Log amplitude profiles before modifications
    log_amplitude_top_before = np.log(np.absolute(top[0:3,:])).T
    log_amplitude_bottom_before = np.log(np.absolute(bottom[0:3,:])).T

    # Mask
    top[:] = np.array([np.zeros_like(x) if (np.log(np.absolute(x)) > threshold).any() else x for x in top[:].T]).T
    bottom[:] = np.array([np.zeros_like(x) if (np.log(np.absolute(x)) > threshold).any() else x for x in bottom[:].T]).T
    # top_2[:] = np.array([np.zeros_like(x) if (np.log(np.absolute(x)) > threshold_2).any() else x for x in top_2[:].T]).T
    # bottom_2[:] = np.array([np.zeros_like(x) if (np.log(np.absolute(x)) > threshold_2).any() else x for x in bottom_2[:].T]).T
    # top_3[:] = np.array([np.zeros_like(x) if (np.log(np.absolute(x)) > threshold_2).any() else x for x in top_3[:].T]).T
    # bottom_3[:] = np.array([np.zeros_like(x) if (np.log(np.absolute(x)) > threshold_2).any() else x for x in bottom_3[:].T]).T

    # Log amplitude profiles after modifications
    log_amplitude_top_after = np.log(np.absolute(top[0:3,:])).T
    log_amplitude_bottom_after = np.log(np.absolute(bottom[0:3,:])).T

    # Magnitude spectrum after modifications
    magnitude_spectrum_after = 20*np.log(np.abs(np.fft.fftshift(f)))

    # Inverse FFT for filtered image
    im_filtered = utils.normalize(np.fft.ifft2(f).real)

    # gaussian_kernel = np.array([
    #        [1, 4, 6, 4, 1],
    #        [4, 16, 24, 16, 4],
    #        [6, 24, 36, 24, 6],
    #        [4, 16, 24, 16, 4],
    #        [1, 4, 6, 4, 1],
    #        ]) / 256

    # image_low_pass = convolve_im(im_filtered, gaussian_kernel,
    #                              verbose=False)

    # Creating subplots
    fig, axs = plt.subplots(1, 6, figsize=(30, 5))

    # Original Image
    axs[0].imshow(im, cmap='gray')
    axs[0].set_title('Original Image')

    # Magnitude Spectrum Before
    axs[1].imshow(magnitude_spectrum_before, cmap='gray')
    axs[1].set_title('Magnitude Spectrum Before')

    # Log Amplitude Top Before
    axs[2].plot(range(top.shape[1]), log_amplitude_top_before)
    axs[2].plot(range(bottom.shape[1]), log_amplitude_bottom_before)
    axs[2].grid()
    axs[2].set_title('Log Amplitude Profile Before')

    # Magnitude Spectrum After
    axs[3].imshow(magnitude_spectrum_after, cmap='gray')
    axs[3].set_title('Magnitude Spectrum After')

    # Log Amplitude Bottom Before
    axs[4].plot(range(top.shape[1]), log_amplitude_top_after)
    axs[4].plot(range(bottom.shape[1]), log_amplitude_bottom_after)
    axs[4].grid()
    axs[4].set_title('Log Amplitude Profile After')

    # Filtered Image
    axs[5].imshow(im_filtered, cmap='gray')
    axs[5].set_title('Filtered Image')

    # axs[6].imshow(image_low_pass, cmap='gray')
    # axs[6].set_title('Filtered Image')

    # Show all subplots
    plt.show()

    ### END YOUR CODE HERE ###
    utils.save_im("moon_filtered.png", im_filtered)

