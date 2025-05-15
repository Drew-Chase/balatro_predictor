import {Select, SelectItem} from "@heroui/react";
import {CardSuit, CardSuitSelector} from "./CardSuitSelector.tsx";

type CardOption = {
    card: number,
    suit: CardSuit,
}
type CardOptionSelectorProps = {
    index: number,
    value: CardOption,
    onChange: (option: CardOption) => void,
}

export function CardOptionSelector(props: CardOptionSelectorProps)
{
    return (
        <div className={"flex flex-row gap-2 items-center"}>
            <p className={"min-w-[70px] grow"}>Card: {props.index}</p>
            <CardSuitSelector value={props.value.suit} onChange={suit => props.onChange({...props.value, suit})}/>
            <Select label={"Number"} selectedKeys={[props.value.card.toString()]} onSelectionChange={keys => props.onChange({...props.value, card: parseInt([...keys][0] as string)})}>
                {Array.from({length: 13}, (_, i) => i + 1).map(i => <SelectItem key={i.toString()} textValue={i.toString()}>{i}</SelectItem>)}
            </Select>
        </div>
    );
}