import re
import matplotlib.pyplot as plt
import numpy as np

time_pattern = re.compile("\\S \\d* (\\d*\\.\\d*)")
icount_pattern = re.compile(":     (\\d*) instructions:u:")

def get_result(file_input) -> int:
    icounts = []
    with open(file_input, 'r') as file:
        for line in file:
            x = re.search(icount_pattern, line)
            if x is not None:
                icounts.append(int(x.group(0)[6:].split(' ')[0]));
    # fig, ax = plt.subplots()
    plt.ylabel(file_input)
    plt.plot(icounts)

for i in range(0, 326):
    get_result(f'perf.data.{i}.scripted');

get_result("perf.data.scripted")
# get_result("perf.data.0.scripted")
# get_result("perf.data.1.scripted")
plt.show()

# results = plot_all_sequence()
# sorted_results = sorted(results)

# plt.show()

