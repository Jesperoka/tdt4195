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
skip_training = False 
task = 3 # a: 0, b: 1, c: 2, d: 3

# Hyperparameters
num_epochs = 5
batch_size = 64
lr_1 = .0192
lr_2 = 1.0 if task == 2 else lr_1 

def create_model():
    model = nn.Sequential(nn.Flatten(), nn.Linear(28 * 28 * 1, 10))
    model = utils.to_cuda(model)
    return model

def create_model_2_layer():
    model = nn.Sequential(nn.Flatten(), nn.Linear(28*28*1, 64), nn.ReLU(), nn.Linear(64, 10))
    model = utils.to_cuda(model)
    return model

image_transform = torchvision.transforms.Compose([
    torchvision.transforms.ToTensor(),
])

image_transform_normalizaton = torchvision.transforms.Compose(
    [torchvision.transforms.ToTensor(),
     torchvision.transforms.Normalize(mean=0.5, std=0.5)])

if task == 0 or task == 3:
    dataloader_train_1, dataloader_test_1 = dataloaders.load_dataset(batch_size, image_transform)
else:
    dataloader_train_1, dataloader_test_1 = dataloaders.load_dataset(batch_size, image_transform_normalizaton)

dataloader_train_2, dataloader_test_2 = dataloaders.load_dataset(batch_size, image_transform_normalizaton)

model_1 = create_model()
model_2 = create_model_2_layer() if task == 3 else create_model()

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

    torch.save(model_1.state_dict(), "saved_models/model_1.torch")
    torch.save(model_2.state_dict(), "saved_models/model_2.torch")
else:
    model_1.load_state_dict(torch.load("saved_models/model_1.torch"))
    model_2.load_state_dict(torch.load("saved_models/model_2.torch"))


def report_final_avg_loss_and_accuracy(dataloaders, models, loss_function=torch.nn.CrossEntropyLoss(), added_texts=["",""]):
    final_loss_1, final_acc_1 = utils.compute_loss_and_accuracy(dataloaders[0], models[0], loss_function)
    final_loss_2, final_acc_2 = utils.compute_loss_and_accuracy(dataloaders[1], models[1], loss_function)
    print(f"Final Test loss {added_texts[0]}: {final_loss_1}. Final Test accuracy {added_texts[0]}: {final_acc_1}")
    print(f"Final Test loss {added_texts[1]}: {final_loss_2}. Final Test accuracy {added_texts[1]}: {final_acc_2}")


if task == 0 or task == 3:
    subtask = "a" if task == 0 else "d"
    label_1 = "Without Normalization" if task == 0 else "No hidden layers, no normalization"
    label_2 = "With Normalization" if task == 0 else "One hidden layer, with normalization"
    utils.plot_loss(train_loss_dict_1, label="Train Loss "+label_1)
    utils.plot_loss(train_loss_dict_2, label="Train Loss "+label_2)
    plt.ylim([0, 1])
    plt.legend()
    plt.xlabel("Global Training Step")
    plt.ylabel("Cross Entropy Loss")
    plt.savefig("image_solutions/task4"+subtask+"_train.png")
    report_final_avg_loss_and_accuracy([dataloader_train_1, dataloader_train_2], [model_1, model_2], added_texts=["without normalization", "with normalization"])
    plt.show()

    utils.plot_loss(test_loss_dict_1, label="Train Loss "+label_1)
    utils.plot_loss(test_loss_dict_2, label="Train Loss "+label_2)
    plt.ylim([0, 1])
    plt.legend()
    plt.xlabel("Global Training Step")
    plt.ylabel("Cross Entropy Loss")
    plt.savefig("image_solutions/task4"+subtask+"_test.png")
    report_final_avg_loss_and_accuracy([dataloader_test_1, dataloader_test_2], [model_1, model_2], added_texts=["without normalization", "with normalization"])
    plt.show()


if task == 1:
    weight = list(model_2.children())[1].weight.cpu().data
    for i in range(weight.shape[0]):
        plt.imsave("image_solutions/task4b_class_" + str(i) + ".png", weight[i,:].reshape((28,28)))


if task == 2:
    utils.plot_loss(train_loss_dict_1, label="Train Loss With lr = 0.0192")
    utils.plot_loss(train_loss_dict_2, label="Train Loss With lr = 1.0")
    plt.ylim([0, 10])
    plt.legend()
    plt.xlabel("Global Training Step")
    plt.ylabel("Cross Entropy Loss")
    plt.savefig("image_solutions/task4c_train.png")
    report_final_avg_loss_and_accuracy([dataloader_test_1, dataloader_test_2], [model_1, model_2], added_texts=["with lr = 0.0192", "with lr = 1.0"])
