# Target: x - y (Tensors)

import numpy as np

import concrete.numpy as hnp


def main():
    def function_to_compile(x, y):
        return x - y

    x = hnp.EncryptedTensor(hnp.UnsignedInteger(3), shape=(3,))
    y = hnp.EncryptedTensor(hnp.UnsignedInteger(3), shape=(3,))

    inputset = [
        (np.array([6, 2, 4]), np.array([4, 1, 2])),
        (np.array([1, 3, 1]), np.array([1, 1, 0])),
        (np.array([5, 1, 2]), np.array([4, 1, 1])),
        (np.array([0, 7, 7]), np.array([0, 7, 0])),
    ]

    # Measure: Compilation Time (ms)
    engine = hnp.compile_numpy_function(function_to_compile, {"x": x, "y": y}, inputset)
    # Measure: End

    inputs = []
    labels = []
    for _ in range(4):
        sample_x = np.random.randint(3, 2 ** 3, size=(3,))
        sample_y = np.random.randint(0, 4, size=(3,))

        inputs.append([sample_x, sample_y])
        labels.append(function_to_compile(*inputs[-1]))

    correct = 0
    for input_i, label_i in zip(inputs, labels):
        # Measure: Evaluation Time (ms)
        result_i = engine.run(*input_i)
        # Measure: End

        if result_i == label_i:
            correct += 1

    # Measure: Accuracy (%) = (correct / len(inputs)) * 100


if __name__ == "__main__":
    main()
