from poker import *
from random import choice
import pytest

@pytest.fixture
def deck():
    return FrenchDeck()

def test_basic_using(deck):
    assert len(deck) == 52
    assert deck[0] == Card('2', 'spades')

def test_choice(deck):
    card = choice(deck)   
    assert card is not None
    assert isinstance(card, Card)

def test_contains(deck):
    assert Card('2', 'hearts') in deck
    assert Card('Z', 'clubs') not in deck

def test_sort(deck):
    sorted_deck = sorted(deck, key=FrenchDeck.spades_high)
    assert sorted_deck[0] == Card('2', 'clubs')
    assert sorted_deck[1] == Card('2', 'diamonds')
    assert sorted_deck[-1] == Card('A', 'spades')