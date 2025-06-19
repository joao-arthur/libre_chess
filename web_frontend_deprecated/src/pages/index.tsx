import type { ReactElement } from "react";
import { useEffect, useRef, useState } from "react";
import { useWindowDimension } from "../hooks/useWindowDimension";
import initWASM, {
    engineGetBoardColorPresets,
    engineGetBoardSetPresets,
    engineSetBoardColor,
    engineSetBoardSet,
} from "libre_chess_engine";
import { useChess } from "../hooks/useChess";
import { Select } from "@/components/Select";

export default function Main(): ReactElement {
    const {
        init,
        model,
    } = useChess();
    const initiated = useRef(false);
    const [boardColorPresets, setBoardColorPresets] = useState<any[]>([]);
    const [boardSetPresets, setBoardSetPresets] = useState<any[]>([]);
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const dimension = useWindowDimension();

    useEffect(() => {
        if (!initiated.current) {
            initiated.current = true;
            initWASM().then(() => {
                if (!canvasRef.current) {
                    return;
                }
                init(canvasRef.current);
                setBoardColorPresets(engineGetBoardColorPresets());
                setBoardSetPresets(engineGetBoardSetPresets());
            });
        }
    }, []);

    function handleSetBoardColor(preset: string) {
            engineSetBoardColor(preset);
    }

    function handleSetBoardSet(preset: string) {
            engineSetBoardSet(preset);
    }

    return (
        <main className="w-screen h-screen flex">
            <canvas
                className="m-auto"
                width={dimension}
                height={dimension}
                style={{ width: dimension, height: dimension }}
                ref={canvasRef}
            />
            <div className="flex flex-col">
                <div className="flex flex-col my-1">
                    <label htmlFor="boardColor">Board Color</label>
                    <Select
                        id="boardColor"
                        options={boardColorPresets.map((item: any) => ({
                            label: item.name,
                            value: item.id,
                        }))}
                        value={model?.board_color || ""}
                        onChange={handleSetBoardColor}
                    />
                </div>
                <div className="flex flex-col my-1">
                    <label htmlFor="boardSet">Board Set</label>
                    <Select
                        id="boardSet"
                        options={boardSetPresets.map((item: any) => ({
                            label: item.name,
                            value: item.id,
                        }))}
                        value={model?.board_set || ""}
                        onChange={handleSetBoardSet}
                    />
                </div>
            </div>
        </main>
    );
}
