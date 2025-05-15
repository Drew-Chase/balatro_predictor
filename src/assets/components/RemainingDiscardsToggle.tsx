import {SwitchProps, useSwitch, VisuallyHidden} from "@heroui/react";

export function RemainingDiscardsToggle(props: SwitchProps)
{
    const {Component, slots, isSelected, getBaseProps, getInputProps, getWrapperProps} = useSwitch(props);

    return (
        <div className="flex flex-row gap-2 items-center">
            <p className="text-default-500 select-none">Discards Remaining</p>
            <Component {...getBaseProps()}>
                <VisuallyHidden>
                    <input {...getInputProps()} />
                </VisuallyHidden>
                <div
                    {...getWrapperProps()}
                    className={slots.wrapper({
                        class: [
                            "w-8 h-8",
                            "flex items-center justify-center",
                            "rounded-lg bg-default-300 hover:bg-default-200"
                        ]
                    })}
                >
                    {isSelected ? <p>1</p> : <p>2</p>}
                </div>
            </Component>
        </div>
    );
}