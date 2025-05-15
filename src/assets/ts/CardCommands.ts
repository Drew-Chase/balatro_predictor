import {invoke} from "@tauri-apps/api/core";

export type Card = {
    card_face: CardFace;
    card_suit: CardSuit;
};

export enum CardSuit
{
    Clubs = "Clubs",
    Diamonds = "Diamonds",
    Hearts = "Hearts",
    Spades = "Spades",
}

export enum CardFace
{
    Two = "2",
    Three = "3",
    Four = "4",
    Five = "5",
    Six = "6",
    Seven = "7",
    Eight = "8",
    Nine = "9",
    Ten = "10",
    Jack = "Jack",
    Queen = "Queen",
    King = "King",
    Ace = "Ace",
}

export enum CalculationType
{
    HighCard = "High Card",
    Pair = "Pair",
    TwoPair = "Two Pair",
    ThreeOfAKind = "Three of a Kind",
    Straight = "Straight",
    Flush = "Flush",
    FullHouse = "Full House",
    FourOfAKind = "Four of a Kind",
    StraightFlush = "Straight Flush",
    RoyalFlush = "Royal Flush"
}

export class CardCommands
{
    public static async calculateHighCardProbability(hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        console.log("Calculating high card probability for", hand, discardedCards);
        try
        {
            const probability = await invoke<number>("calculate_high_card_probability", {
                hand,
                discardedCards
            });
            console.log("High Card Probability:", probability);
            return probability;
        } catch (error)
        {
            console.error("Error calculating high card probability:", error);
            return 0;
        }
    }

    public static async calculatePairProbability(hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        try
        {
            const probability = await invoke<number>("calculate_pair_probability", {
                hand,
                discardedCards
            });
            console.log("Pair Probability:", probability);
            return probability;
        } catch (error)
        {
            console.error("Error calculating pair probability:", error);
            return 0;
        }
    }

    public static async calculateTwoPairProbability(hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        try
        {
            const probability = await invoke<number>("calculate_two_pair_probability", {
                hand,
                discardedCards
            });
            console.log("Two Pair Probability:", probability);
            return probability;
        } catch (error)
        {
            console.error("Error calculating two pair probability:", error);
            return 0;
        }
    }

    public static async calculateThreeOfAKindProbability(hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        try
        {
            const probability = await invoke<number>("calculate_three_of_a_kind_probability", {
                hand,
                discardedCards
            });
            console.log("Three of a Kind Probability:", probability);
            return probability;
        } catch (error)
        {
            console.error("Error calculating three of a kind probability:", error);
            return 0;
        }
    }

    public static async calculateStraightProbability(hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        try
        {
            const probability = await invoke<number>("calculate_straight_probability", {
                hand,
                discardedCards
            });
            console.log("Straight Probability:", probability);
            return probability;
        } catch (error)
        {
            console.error("Error calculating straight probability:", error);
            return 0;
        }
    }

    public static async calculateFlushProbability(hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        try
        {
            const probability = await invoke<number>("calculate_flush_probability", {
                hand,
                discardedCards
            });
            console.log("Flush Probability:", probability);
            return probability;
        } catch (error)
        {
            console.error("Error calculating flush probability:", error);
            return 0;
        }
    }

    public static async calculateFullHouseProbability(hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        try
        {
            const probability = await invoke<number>("calculate_full_house_probability", {
                hand,
                discardedCards
            });
            console.log("Full House Probability:", probability);
            return probability;
        } catch (error)
        {
            console.error("Error calculating full house probability:", error);
            return 0;
        }
    }

    public static async calculateFourOfAKindProbability(hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        try
        {
            const probability = await invoke<number>("calculate_four_of_a_kind_probability", {
                hand,
                discardedCards
            });
            console.log("Four of a Kind Probability:", probability);
            return probability;
        } catch (error)
        {
            console.error("Error calculating four of a kind probability:", error);
            return 0;
        }
    }

    public static async calculateStraightFlushProbability(hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        try
        {
            const probability = await invoke<number>("calculate_straight_flush_probability", {
                hand,
                discardedCards
            });
            console.log("Straight Flush Probability:", probability);
            return probability;
        } catch (error)
        {
            console.error("Error calculating straight flush probability:", error);
            return 0;
        }
    }

    public static async calculateRoyalFlushProbability(hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        try
        {
            const probability = await invoke<number>("calculate_royal_flush_probability", {
                hand,
                discardedCards
            });
            console.log("Royal Flush Probability:", probability);
            return probability;
        } catch (error)
        {
            console.error("Error calculating royal flush probability:", error);
            return 0;
        }
    }

    public static async calculateProbability(type: CalculationType, hand: Card[], discardedCards: Card[] = []): Promise<number>
    {
        switch (type)
        {
            case CalculationType.HighCard:
                return this.calculateHighCardProbability(hand, discardedCards);
            case CalculationType.Pair:
                return this.calculatePairProbability(hand, discardedCards);
            case CalculationType.TwoPair:
                return this.calculateTwoPairProbability(hand, discardedCards);
            case CalculationType.ThreeOfAKind:
                return this.calculateThreeOfAKindProbability(hand, discardedCards);
            case CalculationType.Straight:
                return this.calculateStraightProbability(hand, discardedCards);
            case CalculationType.Flush:
                return this.calculateFlushProbability(hand, discardedCards);
            case CalculationType.FullHouse:
                return this.calculateFullHouseProbability(hand, discardedCards);
            case CalculationType.FourOfAKind:
                return this.calculateFourOfAKindProbability(hand, discardedCards);
            case CalculationType.StraightFlush:
                return this.calculateStraightFlushProbability(hand, discardedCards);
            case CalculationType.RoyalFlush:
                return this.calculateRoyalFlushProbability(hand, discardedCards);
            default:
                console.error("Unknown calculation type:", type);
                return 0;
        }
    }
}
