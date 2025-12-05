def score(word):
    SCORES = {}
    for letters, score in [("AEIOULNRST", 1), ("DG", 2), ("BCMP", 3), 
                           ("FHVWY", 4), ("K", 5), ("JX", 8), ("QZ", 10)]:
        for letter in letters:
            SCORES[letter] = score
    sum = 0
    for w in word.upper():
        if w in SCORES:
            sum += SCORES[w]
    return sum
