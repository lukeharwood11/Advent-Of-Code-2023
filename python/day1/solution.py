import re
def main():
    with open("./input.txt", 'r') as f:
        text = f.read()
    numbers = ['one', 'two', 'three','four', 'five', 'six', 'seven', 'eight', 'nine']
    reg = re.compile(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))")
    sum = 0
    for line in text.splitlines():
        nums = []
        _numbers = reg.findall(line)
        for number in _numbers:
            if number in numbers:
                nums.append(numbers.index(number)+1)
            else:
                nums.append(int(number))
        sum = sum + (nums[0]*10 + nums[-1])
    print(sum)
        

if __name__ == "__main__":
    main()