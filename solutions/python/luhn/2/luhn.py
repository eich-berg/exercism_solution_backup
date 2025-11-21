class Luhn:
    def __init__(self, card_num):
        self.card_num = card_num.replace(" ", "")

    def valid(self):
        if len(self.card_num) <= 1 or not self.card_num.isnumeric():
            return False
        digits = [int(i) for i in self.card_num]
        digits.reverse()
        sum = 0
        for i, d in enumerate(digits):
            if i % 2 == 1:
                sum += dbl - 9 if (dbl := 2 * d) > 9 else dbl
            else:
                sum += d
        return sum % 10 == 0 
