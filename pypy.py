import re

pattern = re.compile(".*store\.register.*");

with open("clippy_lints/src/lib.rs", 'r') as file:
    i = 0
    output = open("600.txt", 'w');

    for line_num, line in enumerate(file):
        if line_num <= 600:
            continue
        if pattern.match(line):
            output.close();
            i += 1;
            output = open(f'{i}.txt', 'w');
            output.write("{return");
            output.write(f'{line}');
            output.write("}");
        else:
            output.write(line);


        print(i, line);
        # if not line.strip():  # Blank line?
            # if output:
                # output.close()
            # output = None
        # else:
            # if output is None:
                # i += 1
                # print(f'Creating file "{i}.txt"')
                # output = open(f'{i}.txt','w')
            # output.write(line)
#
    # if output:
        # output.close()

# print('-fini-')
