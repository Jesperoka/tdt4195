import numpy as np
import utils
import pathlib

def otsu_thresholding(im: np.ndarray) -> int:
    """
        Otsu's thresholding algorithm that segments an image into 1 or 0 (True or False)
        The function takes in a grayscale image and outputs a threshold value

        args:
            im: np.ndarray of shape (H, W) in the range [0, 255] (dtype=np.uint8)
        return:
            (int) the computed thresholding value
    """
    assert im.dtype == np.uint8
    num_intensity_levels = 256 # from input intensity range [0, 255] assumption

    # 1. Compute normalized histogram of greyscale image
    histogram, _ = np.histogram(im, bins=num_intensity_levels, density=True)
    
    # 2. Compute cumulative sums
    cumulative_sums = np.cumsum(histogram) # never not funny 

    # 3. Compute cumulative means 
    cumulative_means = np.cumsum(np.arange(0, num_intensity_levels) * histogram)

    # 4. Compute global mean
    global_mean = cumulative_means[-1]

    # 5. Compute between-class variance
    def sigma_B(k): 
        P_k, m_k, m_G = cumulative_sums[k], cumulative_means[k], global_mean
        return (m_G*P_k - m_k)**2 / (P_k*(1 - P_k))
    between_class_variances = np.array([sigma_B(k) for k in range(0, num_intensity_levels)])

    # 6. Obtain the Otsu threshold (average if multiple)
    max_k = np.where(between_class_variances == np.max(between_class_variances)) 
    threshold = np.average(max_k)

    return int(round(threshold))


if __name__ == "__main__":
    # DO NOT CHANGE
    impaths_to_segment = [
        pathlib.Path("thumbprint.png"),
        pathlib.Path("rice-shaded.png")
    ]
    for impath in impaths_to_segment:
        im = utils.read_image(impath)
        threshold = otsu_thresholding(im)
        print("Found optimal threshold:", threshold)

        # Segment the image by threshold
        segmented_image = (im >= threshold)
        assert im.shape == segmented_image.shape, "Expected image shape ({}) to be same as thresholded image shape ({})".format(
            im.shape, segmented_image.shape)
        assert segmented_image.dtype == bool, "Expected thresholded image dtype to be bool. Was: {}".format(
            segmented_image.dtype)

        segmented_image = utils.to_uint8(segmented_image)

        save_path = "{}-segmented.png".format(impath.stem)
        utils.save_im(save_path, segmented_image)
