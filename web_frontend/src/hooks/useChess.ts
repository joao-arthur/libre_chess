import { useEffect, useState } from "react";
import {
    engineAddOnChangeListener,
    engineGetSettings,
    engineInit,
    engineSetDimension,
    EngineInfo,
} from "libre_chess_engine";
import { useWindowDimension } from "./useWindowDimension";

type Chess = {
    readonly init: (canvas: HTMLCanvasElement) => void;
    readonly model: EngineInfo | undefined;
};

export function useChess(): Chess {
    const [hasInited, setInit] = useState(false);
    const [model, setModel] = useState<EngineInfo | undefined>(undefined);
    const dimension = useWindowDimension();

    useEffect(() => {
        if (hasInited && dimension > 0) {
            engineSetDimension(dimension);
        }
    }, [dimension, hasInited]);

    function init(canvas: HTMLCanvasElement) {
        engineInit(canvas);
        engineAddOnChangeListener(() => {
            let obj = engineGetSettings();
            setModel({
                board_set: obj.board_set,
                board_color: obj.board_color,
            } as any);
        });
        setInit(true);
    }

    return {
        init,
        model,
    };
}
