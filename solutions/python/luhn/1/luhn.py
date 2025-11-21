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
                if d * 2 > 9:
                    sum += (d * 2) - 9 
                else:
                    sum += (d * 2)
            else:
                sum += d
        return sum % 10 == 0 