import utils
import numpy as np


def region_growing(im: np.ndarray, seed_points: list, T: int) -> np.ndarray:
    """
        A region growing algorithm that segments an image into 1 or 0 (True or False).
        Finds candidate pixels with a Moore-neighborhood (8-connectedness). 
        Uses pixel intensity thresholding with the threshold T as the homogeneity criteria.
        The function takes in a grayscale image and outputs a boolean image

        args:
            im: np.ndarray of shape (H, W) in the range [0, 255] (dtype=np.uint8)
            seed_points: list of list containing seed points (row, col). Ex:
                [[row1, col1], [row2, col2], ...]
            T: integer value defining the threshold to used for the homogeneity criteria.
        return:
            (np.ndarray) of shape (H, W). dtype=bool
    """
    # Get the values at the seed pixels
    seed_points = [tuple(s) for s in seed_points] # prefer tuples when fixed size
    regions = {seed_point: [seed_point] for seed_point in seed_points}
    segmented = np.zeros_like(im).astype(bool)

    def in_image(i, j, N=im.shape[0], M=im.shape[1]):
        return 0 <= i and i <= N-1 and 0 <= j and j <= M-1

    def moore_neighborhood(i, j):
        top_left, top_middle, top_right = (i-1, j-1), (i-1, j), (i-1, j+1)
        middle_left, middle_right = (i, j-1), (i, j+1)
        bottom_left, bottom_middle, bottom_right = (i+1, j-1), (i+1, j), (i+1, j+1)
        return [top_left, top_middle, top_right, middle_left, middle_right, bottom_left, bottom_middle, bottom_right]

    def grow(i: int, j: int, seed_point: tuple[int, int]) -> bool:
        # Candidate points in neighborhood unless already segmented 
        candidates = moore_neighborhood(i, j)
        candidates = [(r, c) for r, c in candidates if in_image(r, c) and not segmented[r, c]]

        # Check all candidates and add them to the region if compatible 
        grow = False
        for candidate in candidates: 
            if np.abs(int(im[candidate]) - int(im[seed_point])) <= T: # must cast to signed int to avoid underflow
                regions[seed_point].append(candidate)
                segmented[candidate] = True
                grow = True

        return grow 

    # Grow each seed 
    for seed_point in seed_points:
        segmented[seed_point] = True
        frontier_idx = 0
        keep_growing = True

        # Keep growing a seeded region as long as we keep adding candidates
        while keep_growing:
            frontier = regions[seed_point][frontier_idx:]
            frontier_idx += len(frontier) # don't double check points
            keep_growing = False

            # Check all candidate points of frontier
            for i, j in frontier:
                if grow(i, j, seed_point): 
                    keep_growing = True

    return segmented


if __name__ == "__main__":
    # DO NOT CHANGE
    im = utils.read_image("defective-weld.png")

    seed_points = [  # (row, column)
        [254, 138],  # Seed point 1
        [253, 296],  # Seed point 2
        [233, 436],  # Seed point 3
        [232, 417],  # Seed point 4
    ]
    intensity_threshold = 50 
    segmented_image = region_growing(im, seed_points, intensity_threshold)

    assert im.shape == segmented_image.shape, "Expected image shape ({}) to be same as thresholded image shape ({})".format(
        im.shape, segmented_image.shape)
    assert segmented_image.dtype == bool, "Expected thresholded image dtype to be bool. Was: {}".format(
        segmented_image.dtype)

    segmented_image = utils.to_uint8(segmented_image)
    utils.save_im("defective-weld-segmented.png", segmented_image)
