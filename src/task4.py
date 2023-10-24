import utils

utils.assign_free_gpus()

import matplotlib.pyplot as plt
import numpy as np
import torch
import torch.nn as nn
import torchvision

import dataloaders
from trainer import Trainer

rng_seed = 0
skip_training = True

# Hyperparameters
num_epochs = 5
batch_size = 64
lr_1 = .0192
lr_2 = .0192

def create_model():
    model = nn.Sequential(nn.Flatten(), nn.Linear(28 * 28 * 1, 10))
    model = utils.to_cuda(model)
    return model

image_transform = torchvision.transforms.Compose([
    torchvision.transforms.ToTensor(),
])

image_transform_normalizaton = torchvision.transforms.Compose(
    [torchvision.transforms.ToTensor(),
     torchvision.transforms.Normalize(mean=0.5, std=0.5)])

dataloader_train_1, dataloader_test_1 = dataloaders.load_dataset(batch_size, image_transform)
dataloader_train_2, dataloader_test_2 = dataloaders.load_dataset(batch_size, image_transform_normalizaton)

model_1 = create_model()
model_2 = create_model()

optimizer_1 = torch.optim.SGD(model_1.parameters(), lr=lr_1)
optimizer_2 = torch.optim.SGD(model_2.parameters(), lr=lr_2)

trainer_1 = Trainer(model=model_1,
                    dataloader_train=dataloader_train_1,
                    dataloader_test=dataloader_test_1,
                    batch_size=batch_size,
                    loss_function=torch.nn.CrossEntropyLoss(),
                    optimizer=optimizer_1)

trainer_2 = Trainer(model=model_2,
                    dataloader_train=dataloader_train_2,
                    dataloader_test=dataloader_test_2,
                    batch_size=batch_size,
                    loss_function=torch.nn.CrossEntropyLoss(),
                    optimizer=optimizer_2)

if not skip_training:
    utils.set_np_torch_rng_seeds(seed=rng_seed)
    train_loss_dict_1, test_loss_dict_1 = trainer_1.train(num_epochs)

    utils.set_np_torch_rng_seeds(seed=rng_seed)
    train_loss_dict_2, test_loss_dict_2 = trainer_2.train(num_epochs)

    torch.save(model_2.state_dict(), "saved_models/model.torch")
else:
    state_dict = torch.load("saved_models/model.torch")
    print(state_dict.keys())
    print(model_2.state_dict().keys())
    model_2.load_state_dict(torch.load("saved_models/model.torch"))

# TASK 4a and c PLOTS
# utils.plot_loss(train_loss_dict_1, label="Train Loss Without Normalization")
# utils.plot_loss(train_loss_dict_2, label="Train Loss With Normalization")
# plt.ylim([0, 1])
# plt.legend()
# plt.xlabel("Global Training Step")
# plt.ylabel("Cross Entropy Loss")
# plt.savefig("image_solutions/task4a_train.png")
# final_loss, final_acc = utils.compute_loss_and_accuracy(dataloader_train_1, model_1, torch.nn.CrossEntropyLoss())
# print(f"Final Test loss: {final_loss}. Final Test accuracy: {final_acc}")
# plt.show()

# utils.plot_loss(test_loss_dict_1, label="Test Loss Without Normalization")
# utils.plot_loss(test_loss_dict_2, label="Test Loss With Normalization")
# plt.ylim([0, 1])
# plt.legend()
# plt.xlabel("Global Training Step")
# plt.ylabel("Cross Entropy Loss")
# plt.savefig("image_solutions/task4a_test.png")
# final_loss, final_acc = utils.compute_loss_and_accuracy(dataloader_test_1, model_1, torch.nn.CrossEntropyLoss())
# print(f"Final Test loss: {final_loss}. Final Test accuracy: {final_acc}")
# plt.show()

# TASK 4b PLOTS
# weight = list(model_2.children())[1].weight.cpu().data
# for i in range(weight.shape[0]):
#     plt.imsave("image_solutions/task4b_class_" + str(i) + ".png", weight[i,:].reshape((28,28)))

# TASK 4c PLOTS



