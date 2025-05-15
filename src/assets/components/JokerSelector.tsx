import {Select, SelectItem} from "@heroui/react";

export type Joker = {
    name: string,
    description: string,
}
export const Jokers: Joker[] = [
    {name: "MAXWELL", description: "Each heart discarded +1 mult"},
    {name: "WILSON", description: "Each pair in hand +3 mult"},
    {name: "WILLOW", description: "Each face card discarded +20 chips"},
    {name: "WOLFGANG", description: "Each King in hand +25 chips"},
    {name: "WOODIE", description: "Each card discarded +7 chips"},
    {name: "WEBBER", description: "Each Heart or Diamond replaced by a club or spade +2 mult"},
    {name: "WIGFRID", description: "Each Spade in hand +25 chips"},
    {name: "WICKERBOTTOM", description: "Each Queen in hand +1 mult"},
    {name: "WX78", description: "Each heart kept once then discarded +2 mult"},
    {name: "WENDY", description: "Each discard that becomes the same suit +5 chips +2 mult"},
    {name: "WES", description: "Hand is worse after discard +30 chips"},
    {name: "WINONA", description: "Each heart kept +1 mult"},
    {name: "WARLY", description: "Hand contains a heart, a club a diamond and a spade +4 mult"},
    {name: "WORTOX", description: "Each heart discarded +15 chips Each heart in hand -11 chips +1 mult"},
    {name: "WURT", description: "Each face card kept +10 chips per face card"},
    {name: "WANDA", description: "Start with 80 chips Each discard -15 chips"},
    {name: "WORMWOOD", description: "Each club kept +15 chips"},
    {name: "WALTER", description: "For each different suit in discard +15 chips"}
];
type JokerSelectorProps = {
    value: Joker,
    onChange: (joker: Joker) => void,
}

export function JokerSelector(props: JokerSelectorProps)
{
    return (
        <Select
            label={"Choose Joker"}
            selectedKeys={[props.value.name]}
            onSelectionChange={keys => props.onChange(Jokers.find(joker => joker.name === [...keys][0] as string)!)}
            description={props.value.description}
        >
            {Jokers.map(joker =>
                <SelectItem key={joker.name} textValue={joker.name}>{joker.name}</SelectItem>
            )}
        </Select>
    );
}