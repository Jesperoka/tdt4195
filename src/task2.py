import torch
import torch.nn as nn
import matplotlib.pyplot as plt
import tqdm
import numpy as np
import utils
import dataloaders
import torchvision
from trainer import Trainer
torch.random.manual_seed(0)
np.random.seed(0)

TASK = 0
names = {0: "a", 1: "b", 2: "c", 3: "d"}
lbl_ext = " SGD" if TASK == 1 else ""

# Hyperparameters
batch_size = 64
learning_rate = 0.02 if TASK == 0 else 0.001
num_epochs = 5
loss_function = torch.nn.CrossEntropyLoss()

# Load the dataset and print some stats
image_transform = torchvision.transforms.Compose([
    torchvision.transforms.Resize((32, 32)),
    torchvision.transforms.ToTensor(),
    torchvision.transforms.Normalize([0.5], [0.5])])

dataloader_train, dataloader_test = dataloaders.load_dataset(
    batch_size, image_transform)
example_images, _ = next(iter(dataloader_train))
print(f"The tensor containing the images has shape: {example_images.shape} (batch size, number of color channels, height, width)",
      f"The maximum value in the image is {example_images.max()}, minimum: {example_images.min()}", sep="\n\t")

def create_model():
    input_channels = example_images.shape[1]
    model = nn.Sequential(
        nn.Conv2d(input_channels, 32, 5, stride=1, padding=2), nn.ReLU(),
        nn.MaxPool2d(2, stride=2),
        nn.Conv2d(32, 64, 3, stride=1, padding=1), nn.ReLU(),
        nn.MaxPool2d(2, stride=2),
        nn.Conv2d(64, 128, 3, stride=1, padding=1), nn.ReLU(),
        nn.MaxPool2d(2, stride=2),
        nn.Flatten(),  
        nn.Linear(2048, 64),
        nn.Linear(64, 10)
    )
    model = utils.to_cuda(model)
    return model

def create_train_plot(lr, optim_class):
    model = create_model()
    optimizer = optim_class(model.parameters(), lr=lr)
    trainer = Trainer(model, dataloader_train, dataloader_test, batch_size, loss_function, optimizer)
    train_loss_dict, test_loss_dict = trainer.train(num_epochs)
    utils.plot_loss(train_loss_dict, label="Train Loss"+lbl_ext+" lr = "+str(lr))
    utils.plot_loss(test_loss_dict, label="Test Loss"+lbl_ext+" lr = "+str(lr))
    return model


model_1 = create_train_plot(learning_rate, torch.optim.SGD)

if TASK == 1:
    create_train_plot(0.02, torch.optim.SGD)
    create_train_plot(learning_rate, torch.optim.Adam)

if TASK == 0: plt.ylim([0, .5])

plt.xlabel("Global Training Step")
plt.ylabel("Cross Entropy Loss")
plt.legend(bbox_to_anchor=(0.9, 0.4325)) if TASK == 1 else plt.legend()
plt.grid()
plt.savefig(utils.image_output_dir.joinpath("task2"+names[TASK]+"_plot.png"))
plt.show()

final_loss, final_acc = utils.compute_loss_and_accuracy(dataloader_test, model_1, loss_function)
print(f"Final Test loss: {final_loss}. Final Test accuracy: {final_acc}")
