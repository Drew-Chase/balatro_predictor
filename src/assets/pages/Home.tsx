import {useState} from "react";
import {Jokers, JokerSelector} from "../components/JokerSelector.tsx";
import {Button, Select, SelectItem} from "@heroui/react";
import {CardOptionSelector} from "../components/CardOptionSelector.tsx";
import {RemainingDiscardsToggle} from "../components/RemainingDiscardsToggle.tsx";
import {CalculationType, Card, CardCommands, CardFace, CardSuit} from "../ts/CardCommands.ts";

export default function Home()
{
    const [joker, setJoker] = useState(Jokers[0]);
    const [cards, setCards] = useState<Card[]>(Array(5).fill({card_face: CardFace.Ace, card_suit: CardSuit.Clubs} as Card));
    const [calculationType, setCalculationType] = useState<CalculationType>(CalculationType.HighCard);
    const [remainingDiscards, setRemainingDiscards] = useState<boolean>(true);
    const [result, setResult] = useState<number | null>(null);

    const handleCardChange = (index: number, newCard: CardOption) => {
        const newCards = [...cards];
        newCards[index] = newCard;
        setCards(newCards);
    };

    const handleCalculate = async () => {
        // Convert cards to the format expected by the backend
        const formattedCards = cards.map(card => ({
            card_face: card.card_face.toString(),
            card_suit: card.card_suit.toString()
        }));

        // Call the appropriate calculation function
        const probability = await CardCommands.calculateProbability(
            calculationType,
            formattedCards,
            [] // No discarded cards for now
        );

        setResult(probability);
    };

    return (
        <div className={"flex flex-col gap-2"}>
            <JokerSelector value={joker} onChange={setJoker}/>

            <Select 
                label="Calculation Type" 
                selectedKeys={[calculationType]}
                onSelectionChange={(keys) => setCalculationType([...keys][0] as CalculationType)}
            >
                {Object.values(CalculationType).map((type) => (
                    <SelectItem key={type} value={type}>
                        {type}
                    </SelectItem>
                ))}
            </Select>

            {Array.from({length: 5}, (_, i) => i).map(i =>
                <CardOptionSelector 
                    key={i.toString()} 
                    index={i + 1} 
                    value={cards[i]} 
                    onChange={(newCard) => handleCardChange(i, newCard)}
                />
            )}

            <RemainingDiscardsToggle 
                isSelected={remainingDiscards}
                onValueChange={setRemainingDiscards}
            />

            <Button
                radius={"full"} 
                color={"primary"}
                onPress={handleCalculate}
            >
                Calculate Probability
            </Button>

            {result !== null && (
                <div className="mt-4 p-4 bg-gray-100 rounded-md">
                    <p className="text-lg font-semibold">Probability: {(result * 100).toFixed(2)}%</p>
                </div>
            )}
        </div>
    );
}
