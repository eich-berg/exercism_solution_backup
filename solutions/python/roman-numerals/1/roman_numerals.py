def roman(number):
    
    ROMAN = {1000: 'M', 900: 'CM', 500: 'D', 400: 'CD', 
             100: 'C', 90: 'XC', 50: 'L', 40: 'XL', 10: 'X', 
             9: 'IX', 5: 'V', 4: 'IV', 1: 'I'}

    result = ''

    while number:
        for arabic in ROMAN.keys():
            if number >= arabic: 
                result += ROMAN[arabic]
                number -= arabic
                break
    return result