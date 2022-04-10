import collections

Card = collections.namedtuple('Card', ['rank', 'suit'])

class FrenchDeck:
    ranks = [str(n) for n in range(2, 11)] + list('JQKA')
    suits = 'spades diamonds clubs hearts'.split()

    def __init__(self) -> None:
        self._cards = [Card(rank, suit) for rank in self.ranks for suit in self.suits]
    
    def __len__(self):
        return len(self._cards)
    
    def __getitem__(self, position):
        return self._cards[position]

    @staticmethod 
    def spades_high(card):
        suits = dict(spades=3, hearts=2, diamonds=1, clubs=0)
        rank = FrenchDeck.ranks.index(card.rank)
        return rank * len(FrenchDeck.suits) + suits[card.suit]


