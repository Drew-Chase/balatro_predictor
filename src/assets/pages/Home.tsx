import {useState} from "react";
import {Jokers, JokerSelector} from "../components/JokerSelector.tsx";
import {Button, NumberInput} from "@heroui/react";
import {CardOptionSelector} from "../components/CardOptionSelector.tsx";
import {RemainingDiscardsToggle} from "../components/RemainingDiscardsToggle.tsx";

export default function Home()
{
    const [joker, setJoker] = useState(Jokers[0]);
    return (
        <div className={"flex flex-col gap-2"}>
            <JokerSelector value={joker} onChange={setJoker}/>
            {Array.from({length: 5}, (_, i) => i + 1).map(i =>
                <CardOptionSelector key={i.toString()} index={i} value={{card: 1, suit: "Hearts"}} onChange={() =>
                {
                }}/>
            )}
            <RemainingDiscardsToggle/>
            <NumberInput
                label={"Number of simulations per discard option"}
            />
            <Button radius={"full"} color={"primary"}>Find Best Move</Button>
        </div>
    );
}

