import re
import matplotlib.pyplot as plt
import numpy as np

pattern = re.compile(":     (\\d*) instructions:u:")

def get_result(file_input) -> int:
    total_icount = 0
    with open(file_input, 'r') as file:
        for line in file:
            x = re.search(pattern, line)
            if x is not None:
                total_icount += int(x.group(0)[6:].split(' ')[0]);
    return total_icount

def plot_all_sequence():
    fig, ax = plt.subplots()
    results = []
    results.append(get_result('perf.data.scripted'))
    print(results[0])
    for i in range(0, 326):
        result = get_result(f'perf.data.{i}.scripted');
        if result >= 0.9e9:
            print(result, i, "HIGH >= 0.9e9")
        if result <= 0.7e9:
            print(result, i, "LOW <= 0.7e9")
        results.append(result)

    ax.plot(results)
    return results

results = plot_all_sequence()
sorted_results = sorted(results)

plt.axhline(y=sorted_results[164], color='r', linestyle="-")

plt.xlabel("Which lint was it (check files .txt in selected_lints)")
plt.ylabel("Instruction count")
plt.show()

