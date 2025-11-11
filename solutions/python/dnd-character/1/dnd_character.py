import random

abilities = ('strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma')

class Character:
    def __init__(self):
        for ability in abilities:
            rolls = sorted(random.randint(1, 6) for _ in range(4))
            setattr(self, ability, sum(rolls[1:]))
        self.hitpoints = 10 + modifier(self.constitution)
    def ability(self):
        return random.choice([self.strength, self.dexterity, self.constitution, 
                             self.intelligence, self.wisdom, self.charisma])

def modifier(value):
    return (value - 10) // 2