import re

def main():
    with open("./input.txt", 'r') as f:
        text = f.read()
    reg = re.compile(r"Game (\d+): (.*)")
    options = [('red', 12), ('green', 13), ('blue', 14)]
    sum = 0
    # each game
    for line in text.splitlines():
        match = reg.match(line)
        print(line)
        _id = match.group(1)
        values = match.group(2)
        # for each round
        valid = True
        for semicolon_sep in values.split(";"):
            for pair in semicolon_sep.split(","):
                _num = re.search(r"(\d+)", pair)
                _value = int(_num.group(1))
                for value, max in options:
                    if value in pair and _value > max:
                        valid = False
        if valid:
            sum += int(_id)
    print(sum)
    # 2727

if __name__ == "__main__":
    main()