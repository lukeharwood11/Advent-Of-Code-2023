import re
from functools import reduce
def main():
    with open("./input.txt", 'r') as f:
        text = f.read()
    reg = re.compile(r"Game (\d+): (.*)")
    sum = 0
    # each game
    for line in text.splitlines():
        match = reg.match(line)
        values = match.group(2)
        # for each round
        min_map = {
            'red': 0,
            'green': 0,
            'blue': 0
        }
        for semicolon_sep in values.split(";"):
            for pair in semicolon_sep.split(","):
                _num = re.search(r"(\d+) ([a-z]+)", pair)
                _value = int(_num.group(1))
                _color = _num.group(2)
                if min_map[_color] < _value:
                    min_map[_color] = _value

        power = reduce(lambda acc, v: acc * v, min_map.values())
        sum += power
    print(sum)
    # 56580 
if __name__ == "__main__":
    main()