import {Select, SelectItem} from "@heroui/react";
import {CardSuitSelector} from "./CardSuitSelector.tsx";
import {Card, CardFace} from "../ts/CardCommands.ts";

type CardOptionSelectorProps = {
    index: number,
    value: Card,
    onChange: (option: Card) => void,
}

export function CardOptionSelector(props: CardOptionSelectorProps)
{
    return (
        <div className={"flex flex-row gap-2 items-center"}>
            <p className={"min-w-[70px] grow"}>Card: {props.index}</p>
            <CardSuitSelector value={props.value.suit} onChange={suit => props.onChange({...props.value, suit})}/>
            <Select label={"Number"} selectedKeys={[props.value.card_face.toString()]} onSelectionChange={keys => props.onChange({...props.value, card_face: CardFace[keys[0] as string] as CardFace})}>
                {Object.values(CardFace).map(face => <SelectItem key={face} textValue={face}>{face}</SelectItem>)}
            </Select>
        </div>
    );
}