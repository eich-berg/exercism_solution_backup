def line_up(name, number):
    num_str = str(number)[-1]
    if num_str == "1" and number % 100 != 11:
        return f"{name}, you are the {number}st customer we serve today. Thank you!"
    elif num_str == "2" and number % 100 != 12:
        return f"{name}, you are the {number}nd customer we serve today. Thank you!"
    elif num_str == "3" and number % 100 != 13:
        return f"{name}, you are the {number}rd customer we serve today. Thank you!"
    else:
        return f"{name}, you are the {number}th customer we serve today. Thank you!"