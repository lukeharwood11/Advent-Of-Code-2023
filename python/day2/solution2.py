import re
from functools import reduce
def main():
    with open("./input.txt", 'r') as f:
        text = f.read()
    reg = re.compile(r"Game (\d+): (.*)")
    options = [('red', 12), ('green', 13), ('blue', 14)]
    sum = 0
    # each game
    for line in text.splitlines():
        match = reg.match(line)
        values = match.group(2)
        # for each round
        min_map = { value: 0 for (value, _) in options }
        for semicolon_sep in values.split(";"):
            for pair in semicolon_sep.split(","):
                _num = re.search(r"(\d+)", pair)
                _value = int(_num.group(1))
                for value, _ in options:
                    if value in pair and min_map[value] < _value:
                        min_map[value] = _value 

        power = reduce(lambda acc, v: acc * v, min_map.values())
        sum += power
    print(sum)
    # 56580 
if __name__ == "__main__":
    main()