import {Select, SelectItem} from "@heroui/react";

export type CardSuit = "Hearts" | "Diamonds" | "Clubs" | "Spades";
type CardSuitSelectorProps = {
    value: CardSuit,
    onChange: (suit: CardSuit) => void,
}

export function CardSuitSelector(props: CardSuitSelectorProps)
{
    return (
        <Select label={"Suit"} selectedKeys={[props.value]} onSelectionChange={keys => props.onChange([...keys][0] as CardSuit)}>
            <SelectItem key={"Hearts"} textValue={"Hearts"}>Hearts</SelectItem>
            <SelectItem key={"Diamonds"} textValue={"Diamonds"}>Diamonds</SelectItem>
            <SelectItem key={"Clubs"} textValue={"Clubs"}>Clubs</SelectItem>
            <SelectItem key={"Spades"} textValue={"Spades"}>Spades</SelectItem>
        </Select>
    );
}