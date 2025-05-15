import {Select, SelectItem} from "@heroui/react";
import {CardSuit} from "../ts/CardCommands.ts";

export type CardSuit = "Hearts" | "Diamonds" | "Clubs" | "Spades";
type CardSuitSelectorProps = {
    value: CardSuit,
    onChange: (suit: CardSuit) => void,
}

export function CardSuitSelector(props: CardSuitSelectorProps)
{
    return (
        <Select label={"Suit"} selectedKeys={[props.value]} onSelectionChange={keys => props.onChange([...keys][0] as CardSuit)}>
            {Object.values(CardSuit).map(suit => <SelectItem key={suit} textValue={suit}>{suit}</SelectItem>)}
        </Select>
    );
}