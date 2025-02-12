lines = open("./wordlist.txt").readlines()
better_lines = []
for l in lines:
    if(l.strip().endswith("s")):
        continue
    better_lines.append(l)
open("./fixedwordlist.txt", "w").writelines(better_lines)